//! Sidecar-domain commands.
//!
//! `sidecar_health` is the Phase 7.8 verification command: it proves the
//! full spawn -> handshake -> `AppState` -> round-trip-HTTP-call chain works,
//! not just that the sidecar binary exists. Kept around (not deleted after
//! verification) since it's a genuinely useful health probe for the frontend
//! later (e.g. a status indicator), and costs nothing to keep.
//!
//! Like `commands::settings::PingResponse`, `SidecarHealthResponse` derives
//! `specta::Type` alongside its `serde` derives so `tauri-specta` can generate a
//! TypeScript binding for it (Phase 9, see `CLAUDE.md`) — run
//! `bun run gen:bindings` / `/gen-bindings` after changing it.

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::services::sidecar as sidecar_service;
use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct SidecarHealthResponse {
    pub status: String,
}

/// Round-trips `GET /health` to the Go sidecar via the base URL the sidecar
/// supervisor (`src/sidecar/manager.rs`) recorded on `AppState` after reading
/// the process's startup port handshake.
#[tauri::command]
#[specta::specta]
pub async fn sidecar_health(state: State<'_, AppState>) -> Result<SidecarHealthResponse, String> {
    let base_url = state
        .sidecar
        .lock()
        .map_err(|_| "sidecar state lock was poisoned".to_string())?
        .base_url
        .clone();

    sidecar_service::check_health(base_url)
        .await
        .map(|health| SidecarHealthResponse {
            status: health.status,
        })
        .map_err(|err| err.to_string())
}
