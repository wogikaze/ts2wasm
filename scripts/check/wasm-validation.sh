#!/usr/bin/env bash
# Build representative fixtures to wasm, then `wasm tools validate` each binary.
#
# Usage: scripts/check_wasm_validation.sh
# Optional: TS2WASM_VALIDATE_FIXTURES="f1 f2" (space-separated, repo-root paths)
# Dependencies: cargo, wasm-tools
set -euo pipefail

repo_root="$(cd "$(dirname "${0}")/.." && pwd)"
cd "$repo_root"

if ! command -v wasm-tools >/dev/null 2>&1; then
  echo "check_wasm_validation: wasm-tools is required" >&2
  exit 1
fi

# Optional:  scripts/check_wasm_validation.sh  path/to/a.ts [more.ts ...]
default_list=(
  fixtures/basics-hello/hello.ts
  fixtures/primitives-control-flow/number.ts
  fixtures/arrays-objects/object.ts
)
if [[ $# -gt 0 ]]; then
  VFIX=("$@")
elif [[ -n "${TS2WASM_VALIDATE_FIXTURES:-}" ]]; then
  # shellcheck disable=SC2206
  VFIX=($TS2WASM_VALIDATE_FIXTURES)
else
  VFIX=("${default_list[@]}")
fi

if ! cargo build -q -p ts2wasm-cli; then
  echo "check_wasm_validation: failed to build ts2wasm-cli" >&2
  exit 1
fi
TS2WASM="target/debug/ts2wasm"
if [[ ! -x "$TS2WASM" ]]; then
  echo "check_wasm_validation: expected binary missing: $TS2WASM" >&2
  exit 1
fi

tmpd="$(mktemp -d)"
trap 'rm -rf "$tmpd"' EXIT

for fixture in "${VFIX[@]}"; do
  [[ -n "$fixture" ]] || continue
  fp="$repo_root/$fixture"
  if [[ ! -f "$fp" ]]; then
    echo "check_wasm_validation: missing: $fixture" >&2
    exit 1
  fi
  w="$tmpd/validate.wasm"
  echo "check_wasm_validation: build $fixture" >&2
  if ! "$TS2WASM" build "$fp" -o "$w" 2>&1; then
    echo "check_wasm_validation: build failed: $fixture" >&2
    exit 1
  fi
  if ! wasm-tools validate "$w"; then
    echo "check_wasm_validation: validate failed: $fixture" >&2
    exit 1
  fi
done
echo "check_wasm_validation: OK" >&2
