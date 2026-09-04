#!/usr/bin/env bash
# Builds the Go sidecar binary straight into `src-tauri/binaries/`, with the
# Rust target-triple suffix Tauri's `bundle.externalBin` mechanism requires
# (e.g. `sidecar-x86_64-unknown-linux-gnu`). Invoked as `apps/sidecar`'s
# `"build"` package.json script — see BUILD_TIMELINE.md Phase 7.7.
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

if ! command -v rustc >/dev/null 2>&1; then
  echo "error: rustc not found on PATH — needed to resolve the target-triple (see 'rustc -vV')" >&2
  exit 1
fi

target_triple="$(rustc -vV | awk '/^host:/ { print $2 }')"
if [ -z "$target_triple" ]; then
  echo "error: could not parse the host target-triple out of 'rustc -vV'" >&2
  exit 1
fi

out_dir="../desktop/src-tauri/binaries"
mkdir -p "$out_dir"

ext=""
case "$target_triple" in
*windows*) ext=".exe" ;;
esac

out_path="$out_dir/sidecar-${target_triple}${ext}"

echo "building sidecar (target-triple: $target_triple) -> $out_path"
go build -o "$out_path" ./cmd/sidecar
