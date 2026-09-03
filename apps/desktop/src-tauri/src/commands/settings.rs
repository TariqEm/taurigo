//! Settings-domain commands.
//!
//! There's no real settings feature yet — just a health/ping command to prove the
//! `invoke_handler` + `AppState` + `services/` pattern end to end before any real
//! feature logic exists (Phase 6's goal).
//!
//! Note: once Phase 9 adds the `specta`/`tauri-specta` dependencies and export
//! wiring, request/response types used by commands should derive `specta::Type`
//! alongside their `serde` derives so `tauri-specta` can generate TypeScript
//! bindings for them (see `CLAUDE.md`). `PingResponse` intentionally omits that
//! derive today since the crate doesn't depend on `specta` yet — add it when
//! Phase 9 lands and regenerate bindings via `/gen-bindings`.

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::services::settings as settings_service;
use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingResponse {
    pub message: String,
    pub db_ok: bool,
}

/// Health-check command: confirms the frontend <-> Rust IPC wiring and that the
/// pooled DB connection (registered in `AppState`) is reachable.
#[tauri::command]
pub fn ping(state: State<'_, AppState>) -> PingResponse {
    let db_ok = settings_service::check_db_connection(&state.db_pool);
    PingResponse {
        message: "pong".to_string(),
        db_ok,
    }
}
