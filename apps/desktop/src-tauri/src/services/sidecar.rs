//! Sidecar-domain business logic backing `commands::sidecar` — talks to the Go
//! sidecar over loopback HTTP using the base URL `sidecar::manager` stored on
//! `AppState` once it read the process's port handshake.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidecarHealth {
    pub status: String,
}

#[derive(Debug, thiserror::Error)]
pub enum SidecarHealthError {
    #[error("sidecar is not running yet (no port handshake observed)")]
    NotRunning,
    #[error("request to the sidecar failed: {0}")]
    Request(#[from] reqwest::Error),
}

/// Calls `GET /health` on the sidecar and returns its parsed response.
/// Returns `SidecarHealthError::NotRunning` if the supervisor hasn't recorded
/// a base URL yet (sidecar still starting, or it's mid-restart).
pub async fn check_health(base_url: Option<String>) -> Result<SidecarHealth, SidecarHealthError> {
    let base_url = base_url.ok_or(SidecarHealthError::NotRunning)?;
    let response = reqwest::get(format!("{base_url}/health"))
        .await?
        .error_for_status()?
        .json::<SidecarHealth>()
        .await?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader};
    use std::path::PathBuf;
    use std::process::{Command, Stdio};

    /// Phase 7.8 verification: proves the Rust -> Go sidecar round trip works
    /// end to end, independent of the full Tauri runtime (this crate's tests
    /// don't have an `AppHandle` to spawn via `tauri-plugin-shell`, so this
    /// spawns the built binary directly and reads its `PORT=` handshake the
    /// same way `sidecar::manager::parse_handshake` does), then calls
    /// `check_health` against it for real over loopback HTTP.
    ///
    /// Requires `apps/sidecar`'s `bun run build` (or `bash scripts/build.sh`)
    /// to have already produced `binaries/sidecar-<target-triple>` — skips
    /// (rather than fails) if that binary isn't present, since building it is
    /// a separate step outside `cargo test`'s own build graph.
    #[test]
    fn check_health_round_trips_against_the_real_sidecar_binary() {
        // Normally installed once in `lib.rs::run()` — tests don't go through
        // that, and reqwest needs a rustls crypto provider installed before
        // building any client. See the comment there for why.
        let _ = rustls::crypto::ring::default_provider().install_default();

        let binary = sidecar_binary_path();
        if !binary.exists() {
            eprintln!(
                "skipping check_health_round_trips_against_the_real_sidecar_binary: {} not built \
                 — run `bun run build` in apps/sidecar first",
                binary.display()
            );
            return;
        }

        let mut child = Command::new(&binary)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("sidecar binary should spawn");

        let stdout = child.stdout.take().expect("stdout should be piped");
        let mut line = String::new();
        BufReader::new(stdout)
            .read_line(&mut line)
            .expect("should read the handshake line from stdout");
        let port: u16 = line
            .trim()
            .strip_prefix("PORT=")
            .expect("first stdout line should be the PORT= handshake")
            .parse()
            .expect("handshake port should be numeric");

        let base_url = format!("http://127.0.0.1:{port}");

        let runtime = tokio::runtime::Runtime::new().expect("tokio runtime should build");
        let result = runtime.block_on(check_health(Some(base_url)));

        let _ = child.kill();
        let _ = child.wait();

        let health = result.expect("health check against the running sidecar should succeed");
        assert_eq!(health.status, "ok");
    }

    /// Mirrors the `<target-triple>` suffix `apps/sidecar/scripts/build.sh`
    /// appends and the extension it uses on Windows.
    fn sidecar_binary_path() -> PathBuf {
        let target_triple = env!("TARGET_TRIPLE");
        let ext = if target_triple.contains("windows") {
            ".exe"
        } else {
            ""
        };
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("binaries")
            .join(format!("sidecar-{target_triple}{ext}"))
    }
}
