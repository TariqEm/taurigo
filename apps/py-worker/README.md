# py-worker (Track A — dev-only scripts)

Per `BUILD_TIMELINE.md` Phase 8, this repo took **Track A**: Python stays a dev tool,
not a shipped component.

- Scripts live under `scripts/` and run with `uv run`, e.g.:

  ```sh
  uv run apps/py-worker/scripts/example_embed.py "some text"
  ```

- Nothing here is bundled into the Tauri app, packaged with PyInstaller, or spawned
  as a sidecar. There's no `/health` endpoint or port handshake to manage from Rust.
- Add dependencies with `uv add <package>` from inside `apps/py-worker/` as scripts
  need them (e.g. a local embeddings library) — they stay dev-machine-only.
- Not part of `bun run build`/`turbo` — Turborepo doesn't orchestrate this package.

If a feature later needs Python at **runtime in the shipped app**, that's Phase 8
Track B: a packaged Python sidecar mirroring `apps/sidecar`'s pattern (own binary,
OS-assigned port, Rust-side process management, scoped `shell:allow-execute`
capability). Revisit the phase and switch tracks rather than growing this into that.
