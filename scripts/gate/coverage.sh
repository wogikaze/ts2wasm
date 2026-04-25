#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  scripts/gate/coverage.sh <base-doc> <current-doc>

Checks:
  - executed count must not decrease per suite
  - build_pass count must not decrease per suite
  - semantic_pass count must not decrease per suite
  - fail count must not increase per suite
USAGE
}

if [[ $# -ne 2 ]]; then
  usage
  exit 1
fi

base_doc="$1"
current_doc="$2"

if [[ ! -f "$current_doc" ]]; then
  echo "missing current doc: $current_doc" >&2
  exit 1
fi

if [[ ! -f "$base_doc" ]]; then
  echo "base doc not found, skipping delta gate: $base_doc"
  exit 0
fi

extract_col() {
  local file="$1"
  local suite="$2"
  local col="$3"
  awk -F'|' -v suite="$suite" -v col="$col" '
    function trim(s) { gsub(/^[ \t]+|[ \t]+$/, "", s); return s }
    /<!-- coverage-table:start -->/ { in_table=1; next }
    /<!-- coverage-table:end -->/ { in_table=0 }
    in_table && $0 ~ /^\|/ {
      row_suite = trim($2)
      if (row_suite == suite) {
        print trim($col)
        exit
      }
    }
  ' "$file"
}

status=0
for suite in "test262" "TypeScript compiler cases" "typescript-go testdata"; do
  base_executed="$(extract_col "$base_doc" "$suite" 4)"
  base_build_pass="$(extract_col "$base_doc" "$suite" 7)"
  base_semantic_pass="$(extract_col "$base_doc" "$suite" 8)"
  base_fail="$(extract_col "$base_doc" "$suite" 9)"
  current_executed="$(extract_col "$current_doc" "$suite" 4)"
  current_build_pass="$(extract_col "$current_doc" "$suite" 7)"
  current_semantic_pass="$(extract_col "$current_doc" "$suite" 8)"
  current_fail="$(extract_col "$current_doc" "$suite" 9)"

  if [[ -z "$base_executed" || -z "$base_build_pass" || -z "$base_semantic_pass" || -z "$base_fail" || -z "$current_executed" || -z "$current_build_pass" || -z "$current_semantic_pass" || -z "$current_fail" ]]; then
    echo "warning: incomplete coverage row for suite: $suite" >&2
    continue
  fi

  if (( current_executed < base_executed )); then
    echo "gate failure: executed decreased for $suite ($base_executed -> $current_executed)" >&2
    status=1
  fi

  if (( current_build_pass < base_build_pass )); then
    echo "gate failure: build_pass decreased for $suite ($base_build_pass -> $current_build_pass)" >&2
    status=1
  fi

  if (( current_semantic_pass < base_semantic_pass )); then
    echo "gate failure: semantic_pass decreased for $suite ($base_semantic_pass -> $current_semantic_pass)" >&2
    status=1
  fi

  if (( current_fail > base_fail )); then
    echo "gate failure: fail increased for $suite ($base_fail -> $current_fail)" >&2
    status=1
  fi
done

exit "$status"
