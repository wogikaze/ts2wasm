#!/usr/bin/env bash
# Compare capability manifest "imports" with wasm module imports (wasm-tools print).
#
# Usage:
#   scripts/manager check-manifest-imports [--fixture PATH.ts]
#
# Default fixture: fixtures/basics-hello/hello.ts (pure WASI console.log).
#
# Dependencies: cargo, wasm-tools, jq, mktemp, bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

usage() {
  cat <<'USAGE'
Usage:
  scripts/manager check-manifest-imports [--fixture PATH.ts]
  scripts/manager check-manifest-imports PATH.ts

A single path ending in .ts may be given without --fixture.
Default fixture: fixtures/basics-hello/hello.ts

Fails if manifest import (module,name) pairs differ from wasm import section.
USAGE
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

fixture="fixtures/basics-hello/hello.ts"
# Optional positional:  scripts/manager check-manifest-imports  fixtures/foo.ts
if [[ -n "${1:-}" && "$1" == *.ts && -f "$1" && "$1" != --* ]]; then
  fixture="$1"
  shift
fi
while [[ $# -gt 0 ]]; do
  case "$1" in
    --fixture)
      fixture="${2:?--fixture requires a path}"
      shift 2
      ;;
    *)
      echo "unknown option: $1" >&2
      usage
      exit 1
      ;;
  esac
done

for c in cargo wasm-tools jq mktemp; do
  command -v "$c" >/dev/null 2>&1 || {
    echo "check_manifest_imports: missing required command: $c" >&2
    exit 1
  }
done

if [[ ! -f "$fixture" ]]; then
  echo "check_manifest_imports: fixture not found: $fixture" >&2
  exit 1
fi

tmpd="$(mktemp -d)"
trap 'rm -rf "$tmpd"' EXIT

wasm="$tmpd/check.wasm"
manifest="$tmpd/manifest.json"

echo "check_manifest_imports: build $fixture" >&2
cargo run -q -p ts2wasm-cli -- build "$fixture" -o "$wasm" --emit-manifest "$manifest"

manifest_imports="$tmpd/manifest.imports"
wasm_imports="$tmpd/wasm.imports"

# Extract imports from canonical manifest schema
# WASI imports: map boolean flags to actual WASI import names
# Node host imports: extract from node_host.imports array
{
  # Extract WASI imports
  jq -r '
    if .wasi.stdout == true then "wasi_snapshot_preview1\tfd_write" else empty end,
    if .wasi.stdin == true then "wasi_snapshot_preview1\tfd_read" else empty end,
    if .wasi.stderr == true then "wasi_snapshot_preview1\tfd_write" else empty end
  ' "$manifest"
  # Extract Node host imports
  jq -r '
    (.node_host.imports // [] | .[]) | split(".") | select(length >= 2) | "\(.[0])\t\(.[1])"
  ' "$manifest"
} | LC_ALL=C sort -u >"$manifest_imports"

wasm-tools print "$wasm" | sed -n 's/^[[:space:]]*(import "\([^"]*\)" "\([^"]*\)".*/\1\t\2/p' | LC_ALL=C sort -u >"$wasm_imports"

if ! cmp -s "$manifest_imports" "$wasm_imports"; then
  echo "check_manifest_imports: manifest imports != wasm imports" >&2
  echo "--- manifest (module<TAB>name) ---" >&2
  cat "$manifest_imports" >&2
  echo "--- wasm ---" >&2
  cat "$wasm_imports" >&2
  exit 1
fi

echo "check_manifest_imports: OK ($fixture)" >&2
