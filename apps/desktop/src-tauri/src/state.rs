//! Shared application state, registered with `tauri::Builder::manage` in `lib.rs`
//! and accessed from commands via `tauri::State<'_, AppState>`.

use std::path::PathBuf;
use std::sync::Mutex;

use crate::db::DbPool;

/// Placeholder for the Go sidecar's process handle + base URL.
///
/// Phase 7 replaces this with real process management in `src/sidecar/`: spawning
/// via `tauri-plugin-shell`'s `Command::sidecar`, reading the port handshake, and
/// restarting the process if it exits unexpectedly. For now this just reserves the
/// `AppState` slot so later phases don't need to touch the state shape.
#[derive(Debug, Default)]
pub struct SidecarHandle {
    // Unused until Phase 7 wires up real sidecar spawning + a port handshake.
    #[allow(dead_code)]
    pub base_url: Option<String>,
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
