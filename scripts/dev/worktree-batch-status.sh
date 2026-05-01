#!/usr/bin/env bash
set -euo pipefail

R="$(git rev-parse --show-toplevel 2>/dev/null || echo "$(cd "$(dirname "$0")/../.." && pwd)")"

ts2wasm_usage() {
  cat >&2 <<USAGE
Usage:
  worktree-batch-status.sh [options] [worktree-dir...]

Collect git status from multiple worktrees in parallel.

If no worktree directories are given, scans all worktrees known to 'git worktree list'.

Options:
  --format <text|json>   Output format (default: text)
  --dirty-only           Only show worktrees with uncommitted changes
  --ahead-only           Only show worktrees ahead of their base
  --base <ref>           Base ref for ahead/behind comparison (default: origin/master)
  --json                 Shorthand for --format json
  -h, --help             Show this help
USAGE
  exit 1
}

FORMAT="text"
DIRTY_ONLY=0
AHEAD_ONLY=0
BASE="origin/master"
WT_DIRS=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --format) FORMAT="$2"; shift 2 ;;
    --dirty-only) DIRTY_ONLY=1; shift ;;
    --ahead-only) AHEAD_ONLY=1; shift ;;
    --base) BASE="$2"; shift 2 ;;
    --json) FORMAT="json"; shift ;;
    -h|--help) ts2wasm_usage ;;
    *) WT_DIRS+=("$1"); shift ;;
  esac
done

# If no worktree dirs given, discover from git worktree list
if [[ ${#WT_DIRS[@]} -eq 0 ]]; then
  while IFS=' ' read -r wt_path branch; do
    [[ "$wt_path" != "$R" ]] && WT_DIRS+=("$wt_path")
  done < <(git -C "$R" worktree list --porcelain | awk '/^worktree /{w=$2} /^branch /&&w!=""{print w, substr($2,12); w=""}')
fi

if [[ ${#WT_DIRS[@]} -eq 0 ]]; then
  echo '{"worktrees":[]}'
  exit 0
fi

# Collect status in parallel
RESULTS_DIR=$(mktemp -d)
trap 'rm -rf "$RESULTS_DIR"' EXIT

collect_status() {
  local wt="$1"
  local out="$RESULTS_DIR/$(echo "$wt" | tr '/' '_')"
  {
    echo "path=$wt"
    echo "branch=$(git -C "$wt" rev-parse --abbrev-ref HEAD 2>/dev/null || echo 'detached')"
    echo "commit=$(git -C "$wt" rev-parse HEAD 2>/dev/null || echo 'none')"
    echo "short=$(git -C "$wt" log --oneline -1 2>/dev/null || echo 'no commits')"
    echo "dirty=$(git -C "$wt" status --porcelain 2>/dev/null | wc -l)"
    echo "ahead=$(git -C "$wt" rev-list --count "$BASE..HEAD" 2>/dev/null || echo 0)"
    echo "behind=$(git -C "$wt" rev-list --count "HEAD..$BASE" 2>/dev/null || echo 0)"
    echo "timestamp=$(git -C "$wt" log -1 --format=%ct 2>/dev/null || echo 0)"
  } > "$out"
}

for wt in "${WT_DIRS[@]}"; do
  collect_status "$wt" &
done
wait

# Read results
RESULTS=()
for f in "$RESULTS_DIR"/*; do
  [[ -f "$f" ]] || continue
  declare -A info
  while IFS='=' read -r key value; do
    info["$key"]="$value"
  done < "$f"

  # Filter
  [[ "$DIRTY_ONLY" -eq 1 && "${info[dirty]}" -eq 0 ]] && continue
  [[ "$AHEAD_ONLY" -eq 1 && "${info[ahead]}" -eq 0 ]] && continue

  RESULTS+=("${info[path]}|${info[branch]}|${info[ahead]}|${info[behind]}|${info[dirty]}|${info[short]}")
done

if [[ "$FORMAT" == "json" ]]; then
  echo '['
  first=1
  for entry in "${RESULTS[@]}"; do
    IFS='|' read -r path branch ahead behind dirty short <<< "$entry"
    [[ $first -eq 0 ]] && echo ','
    first=0
    cat <<JSON
  {"path":"$path","branch":"$branch","ahead":$ahead,"behind":$behind,"dirty":$dirty,"last_commit":"$short"}
JSON
  done
  echo ']'
else
  printf '%-55s %-35s %5s %5s %5s  %s\n' "WORKTREE" "BRANCH" "AHEAD" "BEHIND" "DIRTY" "LAST COMMIT"
  printf '%-55s %-35s %5s %5s %5s  %s\n' "-------" "------" "-----" "-----" "-----" "-----------"
  for entry in "${RESULTS[@]}"; do
    IFS='|' read -r path branch ahead behind dirty short <<< "$entry"
    printf '%-55s %-35s %5s %5s %5s  %s\n' "$path" "$branch" "$ahead" "$behind" "$dirty" "$short"
  done
fi
