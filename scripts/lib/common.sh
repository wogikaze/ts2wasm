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

# Require multiple commands with a single error message.
ts2wasm_require_cmds() {
  local c
  for c in "$@"; do
    if ! command -v "$c" >/dev/null 2>&1; then
      printf '%s\n' "error: required command not found: $c" >&2
      return 1
    fi
  done
  return 0
}

# Standard usage function template.
# Usage: ts2wasm_usage "script-name" "description" [additional_lines...]
ts2wasm_usage() {
  local script_name="$1"
  local description="$2"
  shift 2
  cat <<USAGE
Usage:
  $script_name [options]

$description

Options:
  --check   Compare against current state and fail if it would change.
  -h, --help
USAGE
  if [[ $# -gt 0 ]]; then
    printf '\n%s\n' "$@"
  fi
}

# Parse common arguments (--check, --help).
# Sets global variables: TS2WASM_CHECK_MODE, TS2WASM_SHOW_HELP.
# Returns 0 if parsing succeeded, 1 if help was requested (caller should exit 0).
ts2wasm_parse_common_args() {
  TS2WASM_CHECK_MODE=0
  TS2WASM_SHOW_HELP=0

  while [[ $# -gt 0 ]]; do
    case "$1" in
      --check)
        TS2WASM_CHECK_MODE=1
        ;;
      -h|--help)
        TS2WASM_SHOW_HELP=1
        return 1
        ;;
      *)
        printf '%s\n' "unknown option: $1" >&2
        return 2
        ;;
    esac
    shift
  done
  return 0
}

# Standard check mode comparison.
# Usage: ts2wasm_check_file "file-path" "stale-message"
ts2wasm_check_file() {
  local file="$1"
  local stale_msg="$2"
  local tmp_file
  tmp_file="$(mktemp)"
  trap 'rm -f "$tmp_file"' RETURN

  if ! cmp -s "$file" "$tmp_file"; then
    printf '%s\n' "$stale_msg" >&2
    diff -u "$file" "$tmp_file" >&2 || true
    return 1
  fi
  printf '%s\n' "$file OK (up to date)" >&2
  return 0
}
