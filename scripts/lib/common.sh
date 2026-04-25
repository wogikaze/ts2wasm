# Shared helpers for scripts under scripts/{check,gate,gen,run,...}/.
#
# Contract:
# - Do not execute this file directly.
# - Do not print to stdout (human messages -> stderr).
# - Prefer `return` over `exit` from helpers here; entrypoints decide exit status.
#
# Usage from scripts/<role>/entrypoint.sh:
#   _ts2wasm_entry_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
#   # shellcheck source=scripts/lib/common.sh
#   source "$_ts2wasm_entry_dir/../lib/common.sh"
#   cd "$TS2WASM_REPO_ROOT"

[[ -n "${BASH_VERSION:-}" ]] || return 0

if [[ -z "${_ts2wasm_entry_dir:-}" ]]; then
  printf '%s\n' "ts2wasm: set _ts2wasm_entry_dir before sourcing scripts/lib/common.sh" >&2
  printf '%s\n' "  _ts2wasm_entry_dir=\"\$(cd \"\$(dirname \"\${BASH_SOURCE[0]}\")\" && pwd)\"" >&2
  return 1 2>/dev/null || exit 1
fi

TS2WASM_REPO_ROOT="$(cd "${_ts2wasm_entry_dir}/../.." && pwd)"
export TS2WASM_REPO_ROOT

ts2wasm_log() {
  printf '%s\n' "$*" >&2
}

# Returns 1 if any command is missing (message on stderr).
ts2wasm_require_cmd() {
  local c
  for c in "$@"; do
    if ! command -v "$c" >/dev/null 2>&1; then
      printf '%s\n' "ts2wasm: missing required command: $c" >&2
      return 1
    fi
  done
  return 0
}
