//! Spawns and supervises the Go sidecar binary.
//!
//! `spawn_supervised` is called once from `lib.rs`'s `setup()`. It spawns the
//! sidecar via `tauri-plugin-shell`'s `Command::sidecar`, watches its stdout for
//! the `PORT=<port>` handshake line the Go binary prints on startup (see
//! `apps/sidecar/cmd/sidecar/main.go`'s `handshakePrefix`), and stores the
//! resulting `http://127.0.0.1:<port>` base URL on `AppState` so `services/`
//! code can reach it. If the process exits — crash, or the OS killing it —
//! it's restarted, up to a small bounded number of consecutive failed
//! attempts, after which the supervisor gives up rather than spin forever.
//!
//! `kill_on_exit` is wired into the app's `RunEvent::Exit` handler so the
//! sidecar doesn't linger as an orphan process after the main window closes;
//! `tauri-plugin-shell` only auto-kills children spawned via its JS-facing
//! `shell:spawn` command, not ones spawned directly from Rust like this one.

use std::time::Duration;

use tauri::{AppHandle, Manager};
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;

use crate::state::AppState;

/// Must match `handshakePrefix` in `apps/sidecar/cmd/sidecar/main.go`.
const HANDSHAKE_PREFIX: &str = "PORT=";

/// Bounded restart budget: give up after this many consecutive failed
/// starts (handshake never observed / immediate crash loop) rather than
/// retrying forever.
const MAX_CONSECUTIVE_FAILURES: u32 = 5;

/// Delay before each restart attempt. Simple fixed backoff — no need for
/// anything fancier for a single local child process.
const RESTART_DELAY: Duration = Duration::from_secs(1);

/// Name passed to `Shell::sidecar`. Must match the (extension- and
/// target-triple-less) basename of the binary declared in `tauri.conf.json`'s
/// `bundle.externalBin` (`binaries/sidecar`) — `tauri-plugin-shell` resolves the
/// platform-specific, target-triple-suffixed file for us.
const SIDECAR_NAME: &str = "sidecar";

/// Spawns the sidecar and supervises it for the lifetime of the app. Runs on
/// Tauri's async runtime; fire-and-forget from `setup()`.
pub fn spawn_supervised(app: &AppHandle) {
    let app = app.clone();
    tauri::async_runtime::spawn(supervise(app));
}

/// Kills the currently-running sidecar child process, if any. Intended to be
/// called from the app's `RunEvent::Exit` handler.
pub fn kill_on_exit(app: &AppHandle) {
    let state = app.state::<AppState>();
    let child = match state.sidecar.lock() {
        Ok(mut sidecar) => sidecar.child.take(),
        Err(_) => None,
    };
    if let Some(child) = child {
        if let Err(err) = child.kill() {
            log::warn!("sidecar: failed to kill child process on exit: {err}");
        }
    }
}

async fn supervise(app: AppHandle) {
    let mut consecutive_failures = 0u32;

    loop {
        let handshook = run_once(&app).await;
        clear_handle(&app);

        if handshook {
            consecutive_failures = 0;
        } else {
            consecutive_failures += 1;
            if consecutive_failures >= MAX_CONSECUTIVE_FAILURES {
                log::error!(
                    "sidecar: giving up after {consecutive_failures} consecutive failed starts"
                );
                return;
            }
        }

        tokio::time::sleep(RESTART_DELAY).await;
    }
}

/// Spawns the sidecar once and drives its event stream until it terminates.
/// Returns `true` if the startup handshake was observed at any point during
/// this run (used to reset the restart-failure counter).
async fn run_once(app: &AppHandle) -> bool {
    let command = match app.shell().sidecar(SIDECAR_NAME) {
        Ok(command) => command,
        Err(err) => {
            log::error!("sidecar: failed to resolve sidecar binary: {err}");
            return false;
        }
    };

    let (mut rx, child) = match command.spawn() {
        Ok(pair) => pair,
        Err(err) => {
            log::error!("sidecar: failed to spawn process: {err}");
            return false;
        }
    };

    set_child(app, child);

    let mut handshook = false;

    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(bytes) => {
                let line = String::from_utf8_lossy(&bytes);
                let line = line.trim();
                if let Some(port) = parse_handshake(line) {
                    let base_url = format!("http://127.0.0.1:{port}");
                    log::info!("sidecar: handshake received, base_url={base_url}");
                    set_base_url(app, base_url);
                    handshook = true;
                } else if !line.is_empty() {
                    log::debug!("sidecar stdout: {line}");
                }
            }
            CommandEvent::Stderr(bytes) => {
                log::debug!("sidecar stderr: {}", String::from_utf8_lossy(&bytes).trim());
            }
            CommandEvent::Error(err) => {
                log::error!("sidecar: command event error: {err}");
            }
            CommandEvent::Terminated(payload) => {
                log::warn!(
                    "sidecar: process terminated (code={:?}, signal={:?})",
                    payload.code,
                    payload.signal
                );
                break;
            }
            _ => {}
        }
    }

    handshook
}

fn parse_handshake(line: &str) -> Option<u16> {
    line.strip_prefix(HANDSHAKE_PREFIX)?.trim().parse().ok()
}

fn set_child(app: &AppHandle, child: tauri_plugin_shell::process::CommandChild) {
    let state = app.state::<AppState>();
    if let Ok(mut sidecar) = state.sidecar.lock() {
        sidecar.child = Some(child);
    };
}

fn set_base_url(app: &AppHandle, base_url: String) {
    let state = app.state::<AppState>();
    if let Ok(mut sidecar) = state.sidecar.lock() {
        sidecar.base_url = Some(base_url);
    };
}

/// Clears both the base URL and the (now-dead) child handle once a run of the
/// sidecar has ended, so nothing downstream ever observes a URL/handle for a
/// process that's no longer alive.
fn clear_handle(app: &AppHandle) {
    let state = app.state::<AppState>();
    if let Ok(mut sidecar) = state.sidecar.lock() {
        sidecar.base_url = None;
        sidecar.child = None;
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_handshake_reads_the_port_out_of_a_valid_line() {
        assert_eq!(parse_handshake("PORT=54213"), Some(54213));
    }

    #[test]
    fn parse_handshake_rejects_lines_without_the_prefix() {
        assert_eq!(parse_handshake("54213"), None);
        assert_eq!(parse_handshake(""), None);
    }

    #[test]
    fn parse_handshake_rejects_a_non_numeric_port() {
        assert_eq!(parse_handshake("PORT=abc"), None);
    }
}
