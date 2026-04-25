#!/usr/bin/env bash
# "Standalone" fixtures must not import the wasm module name "host" (Node host shim only).
# Pure WASI + in-wasm runtime must not need host.* imports in emitted wasm.
#
# Builds with the ts2wasm CLI, prints wasm, and fails if a `(import "host" ...` appears.
# Override list with TS2WASM_HOST_FREE_FIXTURES (space-separated paths) if needed.
#
# Usage: scripts/check_host_deny.sh
# Dependencies: cargo, wasm-tools
set -euo pipefail

repo_root="$(cd "$(dirname "${0}")/../.." && pwd)"
cd "$repo_root"

if ! command -v wasm-tools >/dev/null 2>&1; then
  echo "check_host_deny: wasm-tools is required" >&2
  exit 1
fi

# Default: subset aligned with m2 "pure" path + tiny hello; extend as the compiler grows.
# Optional:  scripts/check_host_deny.sh  fixtures/a.ts  fixtures/b.ts
#   or  TS2WASM_HOST_FREE_FIXTURES="f1 f2"
default_list=(
  fixtures/basics-hello/hello.ts
  fixtures/primitives-control-flow/number.ts
  fixtures/primitives-control-flow/string.ts
  fixtures/primitives-control-flow/boolean-if.ts
  fixtures/core-semantics/strict-equal.ts
  fixtures/arrays-objects/object.ts
)
if [[ $# -gt 0 ]]; then
  DEFAULT_FIXTURES=("$@")
elif [[ -n "${TS2WASM_HOST_FREE_FIXTURES:-}" ]]; then
  # shellcheck disable=SC2206
  DEFAULT_FIXTURES=($TS2WASM_HOST_FREE_FIXTURES)
else
  DEFAULT_FIXTURES=("${default_list[@]}")
fi
if [[ ${#DEFAULT_FIXTURES[@]} -eq 0 ]]; then
  echo "check_host_deny: no fixtures configured" >&2
  exit 1
fi

# Ensure we have a local ts2wasm binary
if ! cargo build -q -p ts2wasm-cli; then
  echo "check_host_deny: failed to build ts2wasm-cli" >&2
  exit 1
fi
TS2WASM="target/debug/ts2wasm"
if [[ ! -x "$TS2WASM" ]]; then
  echo "check_host_deny: expected binary missing: $TS2WASM" >&2
  exit 1
fi

tmpd="$(mktemp -d)"
trap 'rm -rf "$tmpd"' EXIT

for fixture in "${DEFAULT_FIXTURES[@]}"; do
  [[ -n "$fixture" ]] || continue
  fpath="$repo_root/$fixture"
  if [[ ! -f "$fpath" ]]; then
    echo "check_host_deny: missing fixture: $fixture" >&2
    exit 1
  fi
  w="$tmpd/${fixture//[\/]/_}.wasm"
  echo "check_host_deny: build $fixture" >&2
  if ! "$TS2WASM" build "$fpath" -o "$w" 2>&1; then
    echo "check_host_deny: build failed for $fixture" >&2
    exit 1
  fi
  if wasm-tools print "$w" | grep -qE '\(import "host"'; then
    echo "check_host_deny: disallowed (import \"host\" ...) in wasm for: $fixture" >&2
    exit 1
  fi
done

echo "check_host_deny: OK (no host module imports in listed fixtures)" >&2
