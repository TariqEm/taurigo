# Taurigo

Desktop app starter repo: Tauri v2 + React + shadcn/ui + Tailwind v4 frontend, a Go
sidecar backend, an optional Python worker, a Turborepo + Bun monorepo, and a local
SQLite database.

The full phased build plan lives in [`BUILD_TIMELINE.md`](./BUILD_TIMELINE.md) — check
its Progress Tracker before starting new work.

## Prerequisites

Package management is **Bun** (`bun install` / `bun add` / `bun run`, `bunx` in place
of `npx`) — not npm/pnpm/yarn.

| Tool                                  | Install                                                           | Verify                               |
| ------------------------------------- | ----------------------------------------------------------------- | ------------------------------------ |
| Bun                                   | `curl -fsSL https://bun.sh/install \| bash`                       | `bun --version`                      |
| Node.js (LTS fallback)                | via [nvm](https://github.com/nvm-sh/nvm)                          | `node --version`                     |
| Rust (rustup, cargo, rustfmt, clippy) | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` | `rustc --version && cargo --version` |
| Tauri v2 CLI                          | `bun add -D @tauri-apps/cli@latest` (already in this repo)        | `bunx tauri --version`               |
| Go                                    | [go.dev/dl](https://go.dev/dl/)                                   | `go version`                         |
| uv (Python installer/venv manager)    | `curl -LsSf https://astral.sh/uv/install.sh \| sh`                | `uv --version`                       |
| git                                   | OS package manager                                                | `git --version`                      |
| GitHub CLI (optional)                 | [cli.github.com](https://cli.github.com/)                         | `gh auth status`                     |

### Platform build dependencies (Tauri)

Tauri wraps each OS's native webview, so building/running the desktop shell needs
platform-specific libraries in addition to the table above. Development on this repo
currently happens in **WSL2 (Ubuntu)**; release builds target **Windows, macOS, and
Ubuntu** and are produced per-platform (Tauri does not support cross-compiling the full
desktop app from one OS to another — use a CI matrix, e.g. GitHub Actions with
`windows-latest` / `macos-latest` / `ubuntu-latest` runners, or build natively on each
OS).

**Linux (Debian/Ubuntu, incl. WSL2):**

```sh
sudo apt update && sudo apt install -y \
  libwebkit2gtk-4.1-dev \
  libjavascriptcoregtk-4.1-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libxdo-dev \
  pkg-config \
  file \
  build-essential \
  curl \
  wget
```

**Windows:** [WebView2](https://developer.microsoft.com/microsoft-edge/webview2/)
(preinstalled on current Windows 10/11) + the Visual Studio Build Tools ("Desktop
development with C++" workload).

**macOS:** Xcode Command Line Tools — `xcode-select --install`.

Always cross-check against the official
[Tauri Prerequisites](https://v2.tauri.app/start/prerequisites/) page for your OS, since
required package names shift between distro releases.

## Getting started

```sh
bun install
bun run tauri dev   # from apps/desktop, once Phase 6 scaffolds the Tauri shell
```

## Repo layout

- `apps/desktop` — the Tauri shell (`src/` React frontend, `src-tauri/` Rust core).
- `apps/sidecar` — Go binary spawned by Rust as a Tauri sidecar (loopback HTTP, OS-assigned port).
- `apps/py-worker` — optional Python sidecar for ML/data work.
- `packages/ui` — shared shadcn-based components (`@repo/ui`), presentational only.
- `packages/types` — TypeScript types generated from Rust via `tauri-specta` — never
  hand-edit `bindings.ts`, regenerate with `bun run gen:bindings`.
- `packages/db-schema` — source-of-truth SQL migrations.
- `packages/brand` — design tokens, copy, logo/icon sources.

See [`CLAUDE.md`](./CLAUDE.md) for full conventions and how Claude Code is set up to
help build this repo phase by phase.

## Commits

Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/),
enforced by [commitlint](./commitlint.config.cjs) (`feat`, `fix`, `docs`, `style`,
`refactor`, `perf`, `test`, `build`, `ci`, `chore`, `revert`; scope optional and
free-form).
