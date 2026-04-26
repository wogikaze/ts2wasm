#!/usr/bin/env bash
# Compatibility entrypoint for coverage matrix generation/checking.
#
# Usage:
#   scripts/gen/coverage-matrix.sh [--check]
#
# The canonical implementation is scripts/gen/coverage-matrix.py.
set -euo pipefail

_ts2wasm_entry_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec python3 "${_ts2wasm_entry_dir}/coverage-matrix.py" "$@"
