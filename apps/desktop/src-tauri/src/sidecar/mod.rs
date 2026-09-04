//! Owns the Go sidecar's process lifecycle: spawning it, reading its startup
//! handshake, and restarting it if it exits unexpectedly. See `manager` for the
//! implementation and `CLAUDE.md`/`BUILD_TIMELINE.md` Phase 7 for the design.
//!
//! This module intentionally stays separate from `services/` — it isn't
//! request/response business logic invoked by a command, it's a long-lived
//! supervisor task started once from `lib.rs::run`'s `setup()`. The one piece of
//! actual "business logic" that talks to the sidecar over HTTP (the `/health`
//! round-trip used by `commands::sidecar`) lives in `services::sidecar`, per
//! `CLAUDE.md`'s commands-stay-thin convention.

pub mod manager;

pub use manager::{kill_on_exit, spawn_supervised};
