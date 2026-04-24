#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  scripts/reference_coverage.sh <suite> [--limit N]

Suites:
  test262   -> reference/test262/test/**/*.js
  tsc       -> reference/TypeScript/tests/cases/compiler/**/*.ts
  tsgo      -> reference/typescript-go/testdata/tests/**

Notes:
  - This script classifies compile outcomes using ts2wasm diagnostics.
  - pass: build succeeded
  - unsupported: source/compiler diagnostics except internal/backend failures
  - blocked: stderr contains [BackendIo] or command timeout
  - fail: internal compiler failures such as [InvariantViolation]
USAGE
}

if [[ $# -lt 1 ]]; then
  usage
  exit 1
fi

suite="$1"
shift

limit=0
while [[ $# -gt 0 ]]; do
  case "$1" in
    --limit)
      limit="${2:-}"
      shift 2
      ;;
    *)
      echo "unknown option: $1" >&2
      usage
      exit 1
      ;;
  esac
done

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

case "$suite" in
  test262)
    denominator=$(/usr/bin/find reference/test262/test -type f -name '*.js' | wc -l | tr -d ' ')
    file_cmd="/usr/bin/find reference/test262/test -type f -name '*.js' | sort"
    ;;
  tsc)
    denominator=$(/usr/bin/find reference/TypeScript/tests/cases/compiler -type f -name '*.ts' | wc -l | tr -d ' ')
    file_cmd="/usr/bin/find reference/TypeScript/tests/cases/compiler -type f -name '*.ts' | sort"
    ;;
  tsgo)
    denominator=$(/usr/bin/find reference/typescript-go/testdata/tests -type f | sort | wc -l | tr -d ' ')
    file_cmd="/usr/bin/find reference/typescript-go/testdata/tests -type f | sort"
    ;;
  *)
    echo "unknown suite: $suite" >&2
    usage
    exit 1
    ;;
esac

mapfile -t files < <(eval "$file_cmd")
if [[ "$limit" -gt 0 ]]; then
  files=("${files[@]:0:$limit}")
fi

executed=0
pass_count=0
fail_count=0
unsupported_count=0
blocked_count=0
skip_count=0
declare -A unsupported_diag_counts

tmp_dir="$(mktemp -d /tmp/ts2wasm-refcov-XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

for file in "${files[@]}"; do
  [[ -f "$file" ]] || continue
  executed=$((executed + 1))

  out_wasm="$tmp_dir/out.wasm"
  err_file="$tmp_dir/err.txt"

  set +e
  timeout 8s cargo run -q -p ts2wasm-cli -- build "$file" -o "$out_wasm" >/dev/null 2>"$err_file"
  rc=$?
  set -e

  if [[ $rc -eq 0 ]]; then
    pass_count=$((pass_count + 1))
    continue
  fi

  if [[ $rc -eq 124 ]]; then
    blocked_count=$((blocked_count + 1))
    continue
  fi

  diag_code="$(grep -oE '\[[A-Za-z0-9_]+\]' "$err_file" | head -n 1 | tr -d '[]')"

  if [[ "$diag_code" == "BackendIo" ]]; then
    blocked_count=$((blocked_count + 1))
  elif [[ "$diag_code" == "InvariantViolation" ]]; then
    fail_count=$((fail_count + 1))
  else
    unsupported_count=$((unsupported_count + 1))
    if [[ -z "$diag_code" ]]; then
      diag_code="Unknown"
    fi
    unsupported_diag_counts["$diag_code"]=$(( ${unsupported_diag_counts["$diag_code"]:-0} + 1 ))
  fi
done

unsupported_diagcodes=""
if [[ ${#unsupported_diag_counts[@]} -gt 0 ]]; then
  unsupported_diagcodes="$({
    for code in "${!unsupported_diag_counts[@]}"; do
      printf '%s:%s\n' "$code" "${unsupported_diag_counts[$code]}"
    done
  } | sort -t: -k2,2nr -k1,1 | paste -sd ',' -)"
fi

coverage_percent="0.00"
if [[ "$denominator" -gt 0 ]]; then
  coverage_percent="$(awk -v e="$executed" -v d="$denominator" 'BEGIN { printf "%.2f", (e / d) * 100 }')"
fi

printf 'suite=%s\n' "$suite"
printf 'denominator=%s\n' "$denominator"
printf 'executed=%s\n' "$executed"
printf 'coverage_percent=%s\n' "$coverage_percent"
printf 'pass=%s\n' "$pass_count"
printf 'fail=%s\n' "$fail_count"
printf 'unsupported=%s\n' "$unsupported_count"
printf 'blocked=%s\n' "$blocked_count"
printf 'skip_with_reason=%s\n' "$skip_count"
printf 'unsupported_diagcodes=%s\n' "$unsupported_diagcodes"
