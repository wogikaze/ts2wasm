#!/usr/bin/env bash
# Fail if issues/index.md is stale or misrepresents issues/open/*.md.
#
# Behavior:
#   Runs scripts/update_issue_index.sh --check, then validates queue placeholders
#   and that every open issue ID appears in the Ready or Blocked generated tables.
#
# Options:
#   -h, --help
#
# Dependencies: bash, awk, grep, sort
#
# Output:
#   Human messages on stderr. Exit 0 on success, nonzero on failure.
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  scripts/check_issue_index.sh

Options:
  -h, --help

Dependencies: bash, awk, grep, sort

Runs scripts/update_issue_index.sh --check, then additional queue consistency checks.
Human status is printed to stderr.
USAGE
}

if [[ $# -gt 0 ]]; then
  case "$1" in
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

require_cmds() {
  local c
  for c in awk grep sort; do
    command -v "$c" >/dev/null 2>&1 || {
      echo "error: required command not found: $c" >&2
      exit 1
    }
  done
}

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

require_cmds

update_script="$repo_root/scripts/update_issue_index.sh"
if [[ ! -x "$update_script" && ! -f "$update_script" ]]; then
  echo "error: missing $update_script" >&2
  exit 1
fi
bash "$update_script" --check

if grep -qF "No ready issues yet" issues/index.md; then
  echo "error: stale Ready queue text (No ready issues yet) in issues/index.md" >&2
  exit 1
fi

shopt -s nullglob
open_files=(issues/open/*.md)
shopt -u nullglob

declare -A need
for f in "${open_files[@]}"; do
  [[ "$(basename "$f")" == ".gitkeep" ]] && continue
  base="$(basename "$f" .md)"
  id="$(grep -m1 '^\*\*ID\*\*:' "$f" 2>/dev/null | sed 's/^\*\*ID\*\*: *//' | sed 's/[[:space:]]*$//' || true)"
  if [[ -z "$id" && "$base" =~ ^([0-9]+)- ]]; then
    id="${BASH_REMATCH[1]}"
  fi
  [[ -n "$id" ]] || continue
  need["$id"]=1
done

if [[ ${#need[@]} -eq 0 ]]; then
  echo "issues/index.md queue OK (no open issues)" >&2
  exit 0
fi

# Skip fenced code blocks so example markers in issues/index.md are not parsed as tables.
awk_fence_skip_ids_in_ready_blocked() {
  awk '
    BEGIN { fence = 0; r = 0; b = 0 }
    /^```/ { fence = !fence; next }
    fence == 1 { next }
    /<!-- generated:ready:start -->/ { r = 1; next }
    /<!-- generated:ready:end -->/ { r = 0; next }
    /<!-- generated:blocked:start -->/ { b = 1; next }
    /<!-- generated:blocked:end -->/ { b = 0; next }
    (r == 1 || b == 1) && $0 ~ /^\| [[:digit:]]+ \|/ {
      split($0, a, "|")
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", a[2])
      if (a[2] != "") print a[2]
    }
  ' issues/index.md
}

mapfile -t in_index < <(awk_fence_skip_ids_in_ready_blocked | LC_ALL=C sort -n -u)

declare -A seen
for x in "${in_index[@]}"; do
  seen["$x"]=1
done

missing=0
for id in "${!need[@]}"; do
  if [[ -z "${seen[$id]-}" ]]; then
    echo "error: open issue ID ${id} is missing from Ready or Blocked tables in issues/index.md" >&2
    missing=1
  fi
done

if [[ "$missing" -ne 0 ]]; then
  exit 1
fi

mapfile -t ready_ids < <(
  awk '
    BEGIN { fence = 0; r = 0 }
    /^```/ { fence = !fence; next }
    fence == 1 { next }
    /<!-- generated:ready:start -->/ { r = 1; next }
    /<!-- generated:ready:end -->/ { r = 0; next }
    r == 1 && $0 ~ /^\| [[:digit:]]+ \|/ {
      split($0, a, "|")
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", a[2])
      if (a[2] != "") print a[2]
    }
  ' issues/index.md | LC_ALL=C sort -n -u
)

ready_count=${#ready_ids[@]}
blocked_count=$(
  awk '
    BEGIN { fence = 0; b = 0; c = 0 }
    /^```/ { fence = !fence; next }
    fence == 1 { next }
    /<!-- generated:blocked:start -->/ { b = 1; next }
    /<!-- generated:blocked:end -->/ { b = 0; next }
    b == 1 && $0 ~ /^\| [[:digit:]]+ \|/ { c++ }
    END { print c + 0 }
  ' issues/index.md
)

if [[ "$ready_count" -eq 0 && "$blocked_count" -gt 0 ]]; then
  :
elif [[ "$ready_count" -eq 0 && "${#need[@]}" -gt 0 && "$blocked_count" -eq 0 ]]; then
  echo "error: issues are open but Ready and Blocked tables list no issue IDs" >&2
  exit 1
fi

echo "issues/index.md queue OK" >&2
