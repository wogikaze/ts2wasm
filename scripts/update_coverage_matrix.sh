#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  scripts/update_coverage_matrix.sh [--check]

Behavior:
  - Reads current executed counts from docs/16-coverage-matrix.md
  - Increases per-suite limits by fixed step each run (ramp-up)
  - Re-runs reference coverage sampling for each suite
  - Rewrites the coverage table rows in docs/16-coverage-matrix.md

Options:
  --check   Do not keep edits; fail if docs/16-coverage-matrix.md is stale.
USAGE
}

check_mode=0
if [[ $# -gt 0 ]]; then
  case "$1" in
    --check)
      check_mode=1
      ;;
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
fi

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

doc_file="docs/16-coverage-matrix.md"
if [[ ! -f "$doc_file" ]]; then
  echo "missing $doc_file" >&2
  exit 1
fi

extract_current_executed() {
  local suite_name="$1"
  local value
  value="$({
    grep -F "| $suite_name |" "$doc_file" || true
  } | head -n 1 | awk -F'|' '{gsub(/ /, "", $4); print $4}')"
  if [[ -z "$value" ]]; then
    echo 0
  else
    echo "$value"
  fi
}

next_limit() {
  local current="$1"
  local step="$2"
  local denominator="$3"
  local proposed=$((current + step))
  if [[ "$proposed" -lt "$step" ]]; then
    proposed="$step"
  fi
  if [[ "$proposed" -gt "$denominator" ]]; then
    proposed="$denominator"
  fi
  echo "$proposed"
}

parse_output_value() {
  local key="$1"
  local blob="$2"
  printf '%s\n' "$blob" | awk -F'=' -v k="$key" '$1==k {print substr($0, index($0,$2)); exit}'
}

run_suite() {
  local suite_key="$1"
  local suite_name="$2"
  local step="$3"

  local current
  current="$(extract_current_executed "$suite_name")"

  local baseline_out
  baseline_out="$(scripts/reference_coverage.sh "$suite_key" --limit 1)"
  local denominator
  denominator="$(parse_output_value denominator "$baseline_out")"

  local limit
  if [[ "$check_mode" -eq 1 ]]; then
    limit="$current"
    if [[ "$limit" -le 0 ]]; then
      limit="$step"
    fi
  else
    limit="$(next_limit "$current" "$step" "$denominator")"
  fi

  local out
  out="$(scripts/reference_coverage.sh "$suite_key" --limit "$limit")"

  local coverage pass fail unsupported blocked skip diag
  coverage="$(parse_output_value coverage_percent "$out")"
  pass="$(parse_output_value pass "$out")"
  fail="$(parse_output_value fail "$out")"
  unsupported="$(parse_output_value unsupported "$out")"
  blocked="$(parse_output_value blocked "$out")"
  skip="$(parse_output_value skip_with_reason "$out")"
  diag="$(parse_output_value unsupported_diagcodes "$out")"

  if [[ -z "$diag" ]]; then
    diag="-"
  fi

  printf '| %s | %s | %s | %s | %s | %s | %s | %s | %s | %s | in-progress | `scripts/reference_coverage.sh %s --limit %s` |\n' \
    "$suite_name" "$denominator" "$limit" "$coverage" "$pass" "$fail" "$unsupported" "$blocked" "$skip" "$diag" "$suite_key" "$limit"
}

row_test262="$(run_suite test262 "test262" 50)"
row_tsc="$(run_suite tsc "TypeScript compiler cases" 30)"
row_tsgo="$(run_suite tsgo "typescript-go testdata" 20)"

tmp_file="$(mktemp /tmp/ts2wasm-cov-matrix-XXXXXX.md)"
awk -v r1="$row_test262" -v r2="$row_tsc" -v r3="$row_tsgo" '
  BEGIN {state=0}
  /<!-- coverage-table:start -->/ {
    print
    print "| suite | denominator | executed | coverage% | pass | fail | unsupported | blocked | skip-with-reason | unsupported (DiagCode breakdown) | status | evidence |"
    print "|---|---:|---:|---:|---:|---:|---:|---:|---:|---|---|---|"
    print r1
    print r2
    print r3
    state=1
    next
  }
  /<!-- coverage-table:end -->/ {
    print
    state=0
    next
  }
  state==0 {print}
' "$doc_file" > "$tmp_file"

mv "$tmp_file" "$doc_file"

if [[ "$check_mode" -eq 1 ]]; then
  if ! git diff --quiet -- "$doc_file"; then
    echo "coverage matrix is stale; run scripts/update_coverage_matrix.sh and commit $doc_file" >&2
    git --no-pager diff -- "$doc_file" >&2 || true
    exit 1
  fi
fi

echo "updated $doc_file"
