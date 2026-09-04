//! Business logic, independent of the Tauri runtime so it's testable in isolation
//! (per `CLAUDE.md`'s commands -> services -> db layering).

pub mod settings;
pub mod sidecar;
