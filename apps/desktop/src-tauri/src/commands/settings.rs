//! Settings-domain commands.
//!
//! There's no real settings feature yet — just a health/ping command to prove the
//! `invoke_handler` + `AppState` + `services/` pattern end to end before any real
//! feature logic exists (Phase 6's goal).
//!
//! `PingResponse` derives `specta::Type` alongside its `serde` derives so
//! `tauri-specta` can generate a TypeScript binding for it (Phase 9, see
//! `CLAUDE.md`) — run `bun run gen:bindings` / `/gen-bindings` after changing it.

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::services::settings as settings_service;
use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct PingResponse {
    pub message: String,
    pub db_ok: bool,
}

/// Health-check command: confirms the frontend <-> Rust IPC wiring and that the
/// pooled DB connection (registered in `AppState`) is reachable.
#[tauri::command]
#[specta::specta]
pub fn ping(state: State<'_, AppState>) -> PingResponse {
    let db_ok = settings_service::check_db_connection(&state.db_pool);
    PingResponse {
        message: "pong".to_string(),
        db_ok,
    }
}
