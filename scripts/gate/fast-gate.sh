#!/usr/bin/env bash
# Single local gate: fmt + script syntax + issue queue + coverage matrix check + nextest (optional).
#
# Usage:
#   scripts/check_fast_gate.sh [--skip-nextest]
#
# Environment:
#   TS2WASM_FAST_GATE_SKIP_NEXTEST=1  Same as --skip-nextest (for pre-push).
#
# Dependencies: cargo, bash (see nested scripts for cargo-nextest, jq, etc.)
set -euo pipefail

_ts2wasm_entry_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/lib/common.sh
source "${_ts2wasm_entry_dir}/../lib/common.sh"
# This script is in scripts/gate/, so repo root is two levels up
TS2WASM_REPO_ROOT="$(cd "${_ts2wasm_entry_dir}/../.." && pwd)"
export TS2WASM_REPO_ROOT
cd "$TS2WASM_REPO_ROOT"

skip_nextest=0
if [[ ${TS2WASM_FAST_GATE_SKIP_NEXTEST:-0} == 1 ]]; then
  skip_nextest=1
fi

while [[ $# -gt 0 ]]; do
  case "$1" in
    --skip-nextest) skip_nextest=1 ;;
    -h|--help)
      ts2wasm_usage "scripts/gate/fast-gate.sh" \
        "Runs cargo fmt --all --check, scripts/check/shell-syntax.sh, scripts/check/issue-queue.py, scripts/gen/coverage-matrix.py --check, and cargo nextest run (unless skipped)." \
        "Options:" \
        "  --skip-nextest   Skip cargo nextest (faster; use in pre-push with targeted tests)."
      exit 0
      ;;
    *)
      ts2wasm_log "unknown option: $1"
      ts2wasm_usage "scripts/gate/fast-gate.sh" \
        "Runs cargo fmt --all --check, scripts/check/shell-syntax.sh, scripts/check/issue-queue.py, scripts/gen/coverage-matrix.py --check, and cargo nextest run (unless skipped)."
      exit 1
      ;;
  esac
  shift
done

ts2wasm_require_cmds cargo bash

run() {
  ts2wasm_log "check_fast_gate: $*"
  "$@"
}

run cargo fmt --all --check
run bash "${TS2WASM_REPO_ROOT}/scripts/check/shell-syntax.sh"
run python3 "${TS2WASM_REPO_ROOT}/scripts/check/issue-queue.py"
run python3 "${TS2WASM_REPO_ROOT}/scripts/gen/coverage-matrix.py" --check

if [[ "$skip_nextest" -eq 0 ]]; then
  run cargo nextest run
else
  ts2wasm_log "check_fast_gate: skipping cargo nextest (--skip-nextest)"
fi

ts2wasm_log "check_fast_gate: OK"
