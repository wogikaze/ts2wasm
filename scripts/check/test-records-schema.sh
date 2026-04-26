#!/usr/bin/env bash
# Validate JSONL TestRecord lines (suite, case, target, status + required reason/tracking).
#
# Usage:
#   scripts/manager check-test-records-schema [file.jsonl]
#   some-runner | scripts/manager check-test-records-schema
#   some-runner | scripts/manager check-test-records-schema -
#
# When no file arg or file is "-", reads stdin.
#
# Dependencies: bash, jq
set -euo pipefail

_self="${BASH_SOURCE[0]}"
usage() {
  cat <<'USAGE'
Usage:
  scripts/manager check-test-records-schema [file.jsonl|-]
  scripts/manager check-test-records-schema --self-test

One JSON object per line. Each line must include suite, case, target, status.
status must be one of: pass fail unsupported blocked skip-with-reason
For unsupported, blocked, skip-with-reason: non-empty reason and tracking required.

Exit 1 on first invalid line.
USAGE
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

if [[ "${1:-}" == "--self-test" ]]; then
  {
    echo '{"suite":"self","case":"pass","target":"wasm","status":"pass"}'
    echo '{"suite":"self","case":"unsup","target":"node","status":"unsupported","reason":"r","tracking":"t"}'
  } | bash "$_self" -
  echo "check_test_records_schema: self-test OK" >&2
  exit 0
fi

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

command -v jq >/dev/null 2>&1 || {
  echo "check_test_records_schema: jq is required" >&2
  exit 1
}

input="${1:--}"

validate_stream() {
  local line_no=0
  while IFS= read -r line || [[ -n "$line" ]]; do
    line_no=$((line_no + 1))
    [[ -z "${line//[$'\t\r\n ']/}" ]] && continue
    if ! echo "$line" | jq -e . >/dev/null 2>&1; then
      echo "check_test_records_schema: line $line_no: invalid JSON" >&2
      return 1
    fi
    for key in suite case target status; do
      if ! echo "$line" | jq -e --arg k "$key" '.[$k] != null and (.[$k] | type == "string") and (.[$k] | length > 0)' >/dev/null 2>&1; then
        echo "check_test_records_schema: line $line_no: missing or empty string field: $key" >&2
        return 1
      fi
    done
    local status
    status="$(echo "$line" | jq -r .status)"
    case "$status" in
      pass | fail) ;;
      unsupported | blocked | skip-with-reason)
        if ! echo "$line" | jq -e '(.reason | type == "string") and (.reason | length > 0)' >/dev/null 2>&1; then
          echo "check_test_records_schema: line $line_no: status $status requires non-empty reason" >&2
          return 1
        fi
        if ! echo "$line" | jq -e '(.tracking | type == "string") and (.tracking | length > 0)' >/dev/null 2>&1; then
          echo "check_test_records_schema: line $line_no: status $status requires non-empty tracking" >&2
          return 1
        fi
        ;;
      *)
        echo "check_test_records_schema: line $line_no: invalid status: $status" >&2
        return 1
        ;;
    esac
  done
  echo "check_test_records_schema: OK ($line_no lines checked)" >&2
}

if [[ "$input" == "-" ]]; then
  validate_stream
else
  if [[ ! -f "$input" ]]; then
    echo "check_test_records_schema: not a file: $input" >&2
    exit 1
  fi
  validate_stream <"$input"
fi
