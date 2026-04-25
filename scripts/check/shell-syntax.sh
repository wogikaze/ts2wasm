#!/usr/bin/env bash
# Validate shell scripts before running coverage/test workflows.
#
# Usage: scripts/check/shell-syntax.sh
#
# Note: `bash -n` is syntax-only. It does not prove runtime behavior.
# After editing a script, also run a representative command (see
# `.agents/skills/scripts-workflow/SKILL.md`).

set -euo pipefail

_ts2wasm_entry_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../lib/common.sh
source "${_ts2wasm_entry_dir}/../lib/common.sh"
cd "$TS2WASM_REPO_ROOT"

ts2wasm_log "Running bash -n on scripts/**/*.sh and scripts/manager"

shopt -s nullglob
paths=(
  "${TS2WASM_REPO_ROOT}/scripts"/*.sh
  "${TS2WASM_REPO_ROOT}/scripts/check"/*.sh
  "${TS2WASM_REPO_ROOT}/scripts/gate"/*.sh
  "${TS2WASM_REPO_ROOT}/scripts/gen"/*.sh
  "${TS2WASM_REPO_ROOT}/scripts/run"/*.sh
  "${TS2WASM_REPO_ROOT}/scripts/report"/*.sh
  "${TS2WASM_REPO_ROOT}/scripts/perf"/*.sh
  "${TS2WASM_REPO_ROOT}/scripts/dev"/*.sh
  "${TS2WASM_REPO_ROOT}/scripts/lib"/*.sh
)

for script in "${paths[@]}"; do
  [[ -f "$script" ]] || continue
  bash -n "$script"
  ts2wasm_log "OK: $script"
done

if [[ -f "${TS2WASM_REPO_ROOT}/scripts/manager" ]]; then
  bash -n "${TS2WASM_REPO_ROOT}/scripts/manager"
  ts2wasm_log "OK: scripts/manager"
fi

ts2wasm_log "All shell syntax checks passed"
