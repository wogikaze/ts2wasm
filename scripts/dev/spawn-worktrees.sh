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
  --assignment-dir <d>
                   Directory for local child assignment files (default: reports/agents)
  --dry-run         Print what would be done without doing it
  -h, --help        Show this help
USAGE
  exit 1
}

BASE="master"
PREFIX="child"
COUNT=""
DRY_RUN=0
ASSIGNMENT_DIR="reports/agents"
ISSUE_FILES=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --count) COUNT="$2"; shift 2 ;;
    --base) BASE="$2"; shift 2 ;;
    --prefix) PREFIX="$2"; shift 2 ;;
    --target-dir) TARGET_DIR="$2"; shift 2 ;;
    --assignment-dir) ASSIGNMENT_DIR="$2"; shift 2 ;;
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


issue_field() {
  local file="$1" field="$2"
  awk -F':[[:space:]]*' -v key="$field" 'tolower($1)==tolower(key){print $2; exit}' "$file" 2>/dev/null \
    | sed 's/^[[:space:]"]*//; s/[[:space:]"]*$//'
}

slugify() {
  sed 's/[^a-zA-Z0-9_-]/_/g'
}

# Parent directory for worktrees
PARENT_DIR="${TARGET_DIR:-"$(dirname "$R")"}"
mkdir -p "$PARENT_DIR"
mkdir -p "$ASSIGNMENT_DIR"

MANIFEST='{"worktrees":[]}'

for issue_file in "${RESOLVED[@]}"; do
  issue_file="$(realpath "$issue_file")"

  # Extract issue ID and title from either repo-local `Id:` headers or legacy
  # lowercase/YAML-style headers, with filename fallback.
  ISSUE_ID=$(issue_field "$issue_file" "id" || true)
  if [[ -z "$ISSUE_ID" ]]; then
    ISSUE_ID="$(basename "$issue_file" .md)"
  fi
  ISSUE_TITLE=$(issue_field "$issue_file" "title" | slugify || true)
  if [[ -z "$ISSUE_TITLE" ]]; then
    ISSUE_TITLE="$(basename "$issue_file" .md | sed -E 's/^[0-9]+[a-z]?[-_]//' | slugify)"
  fi

  TIMESTAMP=$(date +%Y%m%d%H%M%S)
  BRANCH="agent/${PREFIX}-${ISSUE_ID}-${TIMESTAMP}"
  WT_DIR="$PARENT_DIR/ts2wasm-${PREFIX}-${ISSUE_ID}-${TIMESTAMP}"
  AGENT_ID="${PREFIX}-${ISSUE_ID}-${TIMESTAMP}"
  ASSIGNMENT_PATH="$ASSIGNMENT_DIR/$AGENT_ID/assignment.md"

  if [[ "$DRY_RUN" -eq 1 ]]; then
    echo "[DRY-RUN] would create: $WT_DIR (branch: $BRANCH, base: $BASE, issue: $issue_file, assignment: $ASSIGNMENT_PATH)"
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

  mkdir -p "$(dirname "$ASSIGNMENT_PATH")"
  ISSUE_AREA=$(issue_field "$issue_file" "area" || true)
  if [[ -z "$ISSUE_AREA" ]]; then
    ISSUE_AREA=$(issue_field "$issue_file" "labels" || true)
  fi
  if [[ -z "$ISSUE_AREA" ]]; then
    ISSUE_AREA="unknown"
  fi
  cat > "$ASSIGNMENT_PATH" << MD
# Child Assignment: $AGENT_ID

- child id: $AGENT_ID
- worktree: $WT_DIR
- branch: $BRANCH
- base: $BASE
- issue: $issue_file
- issue id: $ISSUE_ID
- title: $ISSUE_TITLE
- area: $ISSUE_AREA

## Required Prompt

Use \`.agents/prompts/autonomous-child-worker.md\`.

## Scope

Read the issue for allowed files, forbidden files, acceptance criteria, and validation commands.

Do not use \`.agents/state\`, \`current_task.json\`, \`project_state.json\`, or \`dev-loop\`.

## Reporting

End each cycle with one \`PARENT_EVENT:\` line and include validation evidence.

Discord reporting is required. Use \`mise run discord-report\` when sending is available; otherwise save the markdown/payload under \`reports/runs/\` and tell the parent.
MD

  # Build manifest entry
  ENTRY=$(cat <<JSON
{"agent_id":"$AGENT_ID","worktree":"$WT_DIR","branch":"$BRANCH","issue_id":"$ISSUE_ID","issue_file":"$issue_file","base":"$BASE","assignment":"$ASSIGNMENT_PATH"}
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
