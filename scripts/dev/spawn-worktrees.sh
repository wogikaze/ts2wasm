#!/usr/bin/env bash
set -euo pipefail

R="$(git rev-parse --show-toplevel 2>/dev/null || echo "$(cd "$(dirname "$0")/../.." && pwd)")"
cd "$R"

ts2wasm_usage() {
  cat >&2 <<USAGE
Usage:
  spawn-worktrees.sh [options] <issue-file>...

Batch create worktrees from issue files. Each issue gets one worktree.

Arguments:
  <issue-file>...   One or more issue file paths (glob patterns ok)

Options:
  --count N         Maximum number of worktrees to create (default: unlimited)
  --base <ref>      Base git ref for worktrees (default: master)
  --prefix <str>    Worktree name prefix (default: child)
  --target-dir <d>  Parent directory for worktrees (default: parent of repo root)
  --dry-run         Print what would be done without doing it
  -h, --help        Show this help
USAGE
  exit 1
}

BASE="master"
PREFIX="child"
COUNT=""
DRY_RUN=0
ISSUE_FILES=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --count) COUNT="$2"; shift 2 ;;
    --base) BASE="$2"; shift 2 ;;
    --prefix) PREFIX="$2"; shift 2 ;;
    --target-dir) TARGET_DIR="$2"; shift 2 ;;
    --dry-run) DRY_RUN=1; shift ;;
    -h|--help) ts2wasm_usage ;;
    *) ISSUE_FILES+=("$1"); shift ;;
  esac
done

if [[ ${#ISSUE_FILES[@]} -eq 0 ]]; then
  echo "error: at least one issue file required" >&2
  ts2wasm_usage
fi

# Resolve glob patterns
RESOLVED=()
for f in "${ISSUE_FILES[@]}"; do
  # If it's a glob pattern, expand
  if [[ "$f" == *\** ]]; then
    for match in $f; do
      [[ -f "$match" ]] && RESOLVED+=("$match")
    done
  else
    [[ -f "$f" ]] && RESOLVED+=("$f")
  fi
done

if [[ ${#RESOLVED[@]} -eq 0 ]]; then
  echo "error: no matching issue files found" >&2
  exit 1
fi

# Apply count limit
if [[ -n "$COUNT" ]]; then
  RESOLVED=("${RESOLVED[@]:0:$COUNT}")
fi

# Parent directory for worktrees
PARENT_DIR="${TARGET_DIR:-"$(dirname "$R")"}"
mkdir -p "$PARENT_DIR"

MANIFEST='{"worktrees":[]}'

for issue_file in "${RESOLVED[@]}"; do
  issue_file="$(realpath "$issue_file")"

  # Extract issue ID and title from YAML frontmatter
  ISSUE_ID=$(awk -F': ' '/^id:/{print $2;exit}' "$issue_file" 2>/dev/null || echo "unknown")
  ISSUE_TITLE=$(awk -F': ' '/^title:/{print $2;exit}' "$issue_file" 2>/dev/null | tr -d '"' | sed 's/[^a-zA-Z0-9_-]/_/g' || echo "unknown")

  TIMESTAMP=$(date +%Y%m%d%H%M%S)
  BRANCH="agent/${PREFIX}-${ISSUE_ID}-${TIMESTAMP}"
  WT_DIR="$PARENT_DIR/ts2wasm-${PREFIX}-${ISSUE_ID}-${TIMESTAMP}"

  if [[ "$DRY_RUN" -eq 1 ]]; then
    echo "[DRY-RUN] would create: $WT_DIR (branch: $BRANCH, base: $BASE, issue: $issue_file)"
    continue
  fi

  echo "Creating worktree: $WT_DIR (branch: $BRANCH)" >&2

  # Create worktree from BASE
  git worktree add "$WT_DIR" -b "$BRANCH" "$BASE" 2>&1 | sed 's/^/  /' >&2

  # Link reference corpus
  if command -v mise &>/dev/null; then
    mise run link-reference -- "$WT_DIR" 2>&1 | sed 's/^/  /' >&2 || true
  fi

  # Set up shared cargo target dir
  mkdir -p "$WT_DIR/.cargo"
  cat > "$WT_DIR/.cargo/config.toml" << TOML
[build]
target-dir = "${R}/target"
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
TOML

  # Set up dev-loop state
  SETUP_PY="$R/scripts/dev/setup-worktree.py"
  [[ -f "$SETUP_PY" ]] || SETUP_PY="$R/_worktrees/setup-worktree.py"
  if [[ -f "$SETUP_PY" ]]; then
    ISSUE_AREA=$(awk -F': ' '/^area:/{print $2;exit}' "$issue_file" 2>/dev/null || echo "runtime/semantics")
    ISSUE_AREA="${ISSUE_AREA//\"/}"
    ACCEPTANCE=$(awk '/^## Acceptance/{f=1;next} /^##/{f=0} f && /^-/{gsub(/- \[.\] /,"");printf "%s|",$0}' "$issue_file" 2>/dev/null || echo "verify gate passes")
    python3 "$SETUP_PY" \
      "$WT_DIR" \
      "$ISSUE_ID" \
      "$ISSUE_TITLE" \
      "$issue_file" \
      "$ISSUE_AREA" \
      "$ACCEPTANCE" \
      "" "" 2>&1 | sed 's/^/  /' >&2
  fi

  # Build manifest entry
  ENTRY=$(cat <<JSON
{"worktree":"$WT_DIR","branch":"$BRANCH","issue_id":"$ISSUE_ID","issue_file":"$issue_file","base":"$BASE"}
JSON
)
  MANIFEST=$(echo "$MANIFEST" | python3 -c "
import json,sys
m=json.load(sys.stdin)
m['worktrees'].append($ENTRY)
json.dump(m,sys.stdout)
")

  echo "  OK" >&2
  echo >&2
done

echo "$MANIFEST"
