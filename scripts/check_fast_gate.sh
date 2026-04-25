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

usage() {
  cat <<'USAGE'
Usage:
  scripts/check_fast_gate.sh [--skip-nextest]

Runs:
  cargo fmt --all --check
  scripts/check/shell-syntax.sh
  scripts/check_issue_queue.sh
  scripts/gen/coverage-matrix.sh --check
  cargo nextest run   (unless skipped)

Options:
  --skip-nextest   Skip cargo nextest (faster; use in pre-push with targeted tests).
  -h, --help
USAGE
}

skip_nextest=0
if [[ ${TS2WASM_FAST_GATE_SKIP_NEXTEST:-0} == 1 ]]; then
  skip_nextest=1
fi

while [[ $# -gt 0 ]]; do
  case "$1" in
    --skip-nextest) skip_nextest=1 ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown option: $1" >&2
      usage
      exit 1
      ;;
  esac
  shift
done

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

for c in cargo bash; do
  command -v "$c" >/dev/null 2>&1 || {
    echo "check_fast_gate: missing required command: $c" >&2
    exit 1
  }
done

run() {
  echo "check_fast_gate: $*" >&2
  "$@"
}

run cargo fmt --all --check
run bash "${repo_root}/scripts/check/shell-syntax.sh"
run bash "${repo_root}/scripts/check_issue_queue.sh"
run bash "${repo_root}/scripts/gen/coverage-matrix.sh" --check

if [[ "$skip_nextest" -eq 0 ]]; then
  run cargo nextest run
else
  echo "check_fast_gate: skipping cargo nextest (--skip-nextest)" >&2
fi

echo "check_fast_gate: OK" >&2
