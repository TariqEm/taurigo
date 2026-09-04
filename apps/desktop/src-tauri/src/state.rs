//! Shared application state, registered with `tauri::Builder::manage` in `lib.rs`
//! and accessed from commands via `tauri::State<'_, AppState>`.

use std::path::PathBuf;
use std::sync::Mutex;

use tauri_plugin_shell::process::CommandChild;

use crate::db::DbPool;

/// The Go sidecar's process handle + base URL, owned by `src/sidecar/manager.rs`.
///
/// `base_url` is `None` until the startup handshake (`PORT=<port>` on stdout) has
/// been read, and is cleared again whenever the process exits (see
/// `sidecar::manager::run_supervised`) so callers never see a stale URL for a dead
/// process. `child` is kept around so the app can kill the process on shutdown —
/// it isn't tracked by `tauri-plugin-shell`'s own child registry because we spawn
/// it directly from Rust (`Shell::sidecar`) rather than via the JS-facing
/// `shell:spawn` command.
#[derive(Debug, Default)]
pub struct SidecarHandle {
    pub base_url: Option<String>,
    pub child: Option<CommandChild>,
}

/// App-wide configuration resolved once at startup.
// Fields aren't read yet — no command needs them until later phases (e.g. the
// sidecar binary path in Phase 7). Kept here now so `AppState`'s shape is settled.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Per-user app data directory (`tauri::path::app_data_dir`).
    pub data_dir: PathBuf,
    /// Path to the sqlite database file inside `data_dir`.
    pub db_path: PathBuf,
}

/// Shared state managed by Tauri for the lifetime of the app.
pub struct AppState {
    pub db_pool: DbPool,
    #[allow(dead_code)]
    pub sidecar: Mutex<SidecarHandle>,
    #[allow(dead_code)]
    pub config: AppConfig,
}

impl AppState {
    pub fn new(db_pool: DbPool, config: AppConfig) -> Self {
        Self {
            db_pool,
            sidecar: Mutex::new(SidecarHandle::default()),
            config,
        }
    }
}
