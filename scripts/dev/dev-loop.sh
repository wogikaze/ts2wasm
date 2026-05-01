#!/usr/bin/env bash
set -euo pipefail

# dev-loop.sh — Autonomous compiler development loop driver
#
# Usage:
#   mise run dev-loop               Show current loop status
#   mise run dev-loop --advance     Advance to next FSM state
#   mise run dev-loop --reset       Reset to SYNC (clean slate)
#   mise run dev-loop --check       Validate state consistency
#
# FSM: SYNC → TRIAGE → TASK_SELECT → PLAN → PLAN_REVIEW_GATE → IMPLEMENT →
#      SELF_REVIEW_GATE → VERIFY_FAST → VERIFY_FULL → CLOSE_OR_SPLIT →
#      RETRO → (back to SYNC)

_ts2wasm_entry_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/lib/common.sh
source "$_ts2wasm_entry_dir/../lib/common.sh"
cd "$TS2WASM_REPO_ROOT"

# --- Paths ---
PROJECT_STATE="$TS2WASM_REPO_ROOT/.agents/state/project_state.json"
CURRENT_TASK="$TS2WASM_REPO_ROOT/.agents/state/current_task.json"
ISSUE_INDEX="$TS2WASM_REPO_ROOT/issues/index.md"
FSM_WORKFLOW="$TS2WASM_REPO_ROOT/.agents/workflows/compiler_dev_fsm.md"

# --- Colors / formatting ---
BOLD='\033[1m'
DIM='\033[2m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

log_info()  { printf "${GREEN}==>${NC} ${BOLD}%s${NC}\n" "$*"; }
log_warn()  { printf "${YELLOW}==>${NC} ${BOLD}%s${NC}\n" "$*" >&2; }
log_error() { printf "${RED}==>${NC} ${BOLD}%s${NC}\n" "$*" >&2; }
log_step()  { printf "  ${CYAN}•${NC} %s\n" "$*"; }
log_dim()   { printf "  ${DIM}%s${NC}\n" "$*"; }

# --- Helpers ---

die() {
  log_error "$*"
  exit 1
}

require_file() {
  [[ -f "$1" ]] || die "required file not found: $1"
}

read_json_field() {
  local file="$1" field="$2"
  python3 -c "
import json,sys
d=json.load(sys.stdin)
val = d.get('$field')
if val is None:
    sys.stdout.write('null')
elif isinstance(val, bool):
    sys.stdout.write(str(val).lower())
else:
    sys.stdout.write(str(val))
" < "$file"
}

write_json_file() {
  local file="$1"
  local content="$2"
  python3 -c "
import json, sys
data = json.loads(sys.stdin.read())
with open('$file', 'w') as f:
    json.dump(data, f, indent=2, ensure_ascii=False)
    f.write('\n')
" <<< "$content"
}

timestamp_utc() {
  date -u +"%Y-%m-%dT%H:%M:%SZ"
}

# --- FSM definition ---
FSM_ORDER=(
  "SYNC"
  "TRIAGE"
  "TASK_SELECT"
  "PLAN"
  "PLAN_REVIEW_GATE"
  "IMPLEMENT"
  "SELF_REVIEW_GATE"
  "VERIFY_FAST"
  "VERIFY_FULL"
  "CLOSE_OR_SPLIT"
  "RETRO"
)

# Map state → description of what the agent should do
declare -A FSM_DESC
FSM_DESC[SYNC]="Synchronize: read current-state.md, docs/11, issues/index.md; check for drift"
FSM_DESC[TRIAGE]="Triage: review open issues, ensure Ready queue is current"
FSM_DESC[TASK_SELECT]="Select one Ready issue and set as current_task"
FSM_DESC[PLAN]="Create implementation plan for the selected issue"
FSM_DESC[PLAN_REVIEW_GATE]="Review plan against review_checklist.md; revise if needed"
FSM_DESC[IMPLEMENT]="Implement the smallest slice of the plan"
FSM_DESC[SELF_REVIEW_GATE]="Self-review code against review_checklist.md before verification"
FSM_DESC[VERIFY_FAST]="Run fast gates: mise run fmt, mise run nextest (filtered)"
FSM_DESC[VERIFY_FULL]="Run full validation: mise run gate, verify all acceptance criteria"
FSM_DESC[CLOSE_OR_SPLIT]="Close issue (move to done/) or split into follow-up issues"
FSM_DESC[RETRO]="Write cycle report, log failure patterns, update guardrails"
FSM_DESC["COMPLETE"]="Loop complete — all states traversed"

fsm_index() {
  local state="$1"
  for i in "${!FSM_ORDER[@]}"; do
    if [[ "${FSM_ORDER[$i]}" == "$state" ]]; then
      echo "$i"
      return 0
    fi
  done
  return 1
}

fsm_next() {
  local state="$1"
  local idx
  idx=$(fsm_index "$state") || return 1
  if [[ "$idx" -lt $((${#FSM_ORDER[@]} - 1)) ]]; then
    echo "${FSM_ORDER[$((idx + 1))]}"
  else
    echo "COMPLETE"
  fi
}

# --- Load current state ---
load_state() {
  require_file "$PROJECT_STATE"

  FSM=$(read_json_field "$PROJECT_STATE" "fsm")
  ACTIVE_TASK_ID=$(read_json_field "$PROJECT_STATE" "active_task_id")
  MILESTONE_ID=$(read_json_field "$PROJECT_STATE" "milestone_id")
  PLAN_PATH=$(read_json_field "$PROJECT_STATE" "plan_path")
  FAST_FAILS=$(read_json_field "$PROJECT_STATE" "verify_fast_streak_fails")

  [[ -n "$FSM" ]] || FSM="SYNC"

  # Load current_task.json if it exists
  TASK_STATUS=""
  TASK_TITLE=""
  TASK_ISSUE_PATH=""
  if [[ -f "$CURRENT_TASK" ]]; then
    TASK_STATUS=$(read_json_field "$CURRENT_TASK" "status")
    TASK_TITLE=$(read_json_field "$CURRENT_TASK" "title")
    TASK_ISSUE_PATH=$(read_json_field "$CURRENT_TASK" "issue_path")
    [[ -n "$TASK_STATUS" ]] || TASK_STATUS="unknown"
    [[ -n "$TASK_TITLE" ]] || TASK_TITLE="(no title)"
  fi
}

# --- Count Ready issues ---
count_ready_issues() {
  if [[ ! -f "$ISSUE_INDEX" ]]; then
    echo 0
    return
  fi
  # Count lines between ready markers that look like data rows (start with |)
  sed -n '/<!-- generated:ready:start -->/,/<!-- generated:ready:end -->/p' "$ISSUE_INDEX" \
    | grep -c '^| *[0-9]' 2>/dev/null || echo 0
}

# --- Status output ---
cmd_status() {
  load_state

  printf '\n'
  log_info "Compiler Development Loop Status"
  printf '\n'

  # FSM progress bar
  local total=${#FSM_ORDER[@]}
  local current_idx
  current_idx=$(fsm_index "$FSM") || current_idx=-1

  printf '  FSM: '
  if [[ "$current_idx" -ge 0 ]]; then
    local progress_bar=""
    for ((i = 0; i < total; i++)); do
      if [[ "$i" -lt "$current_idx" ]]; then
        progress_bar+="✓"
      elif [[ "$i" -eq "$current_idx" ]]; then
        progress_bar+="●"
      else
        progress_bar+="○"
      fi
    done
    printf '%s' "$progress_bar"
    printf '  [%d/%d]' "$current_idx" "$total"
    printf '\n'
    log_step "Current state: ${BOLD}${FSM}${NC}"
    log_dim "${FSM_DESC[$FSM]:-}"

    local next_state
    next_state=$(fsm_next "$FSM")
    if [[ "$next_state" != "COMPLETE" ]]; then
      log_step "Next state: ${BOLD}${next_state}${NC}"
      log_dim "${FSM_DESC[$next_state]:-}"
    else
      log_info "Loop complete! Run --reset to start a new cycle."
    fi
  else
    printf "${RED}unknown${NC} (no matching FSM state)\n"
  fi
  printf '\n'

  # Task info
  log_info "Current Task"
  if [[ -n "$TASK_TITLE" && "$TASK_STATUS" != "idle" && "$TASK_STATUS" != "" ]]; then
    log_step "Title:   ${BOLD}$TASK_TITLE${NC}"
    log_step "Status:  $TASK_STATUS"
    [[ -n "$TASK_ISSUE_PATH" ]] && log_step "Issue:   $TASK_ISSUE_PATH"
    [[ -n "$MILESTONE_ID" ]] && log_step "Milestone: $MILESTONE_ID"
    [[ -n "$PLAN_PATH" && "$PLAN_PATH" != "null" ]] && log_step "Plan:    $PLAN_PATH"
  else
    log_dim "(no active task)"
  fi
  printf '\n'

  # Ready issues
  local ready_count
  ready_count=$(count_ready_issues)
  log_info "Ready Queue"
  log_step "$ready_count issues ready for selection"
  if [[ "$ready_count" -gt 0 && "$current_idx" -ge 0 ]]; then
    log_dim "Run --advance from ${FSM} to proceed."
  fi
  printf '\n'

  # Quick suggestions
  log_info "Suggested Actions"
  case "$FSM" in
    SYNC)
      log_step "mise run dev-loop --advance   → TRIAGE"
      log_step "mise run check               → smoke check"
      ;;
    TRIAGE)
      log_step "mise run dev-loop --advance   → TASK_SELECT (if Ready queue has items)"
      log_step "mise run update-issue-index   → refresh issue index"
      ;;
    TASK_SELECT)
      log_step "Pick a Ready issue and set it as current_task"
      log_step "mise run dev-loop --advance   → PLAN (after selecting issue)"
      ;;
    PLAN)
      log_step "Create implementation plan (update plan_path in project_state)"
      log_step "mise run dev-loop --advance   → PLAN_REVIEW_GATE"
      ;;
    PLAN_REVIEW_GATE)
      log_step "Review plan against checklist"
      log_step "mise run dev-loop --advance   → IMPLEMENT (if plan approved)"
      ;;
    IMPLEMENT)
      log_step "Implement code changes"
      log_step "mise run dev-loop --advance   → SELF_REVIEW_GATE"
      ;;
    SELF_REVIEW_GATE)
      log_step "Self-review code against review_checklist.md"
      log_step "mise run dev-loop --advance   → VERIFY_FAST"
      ;;
    VERIFY_FAST)
      log_step "mise run fmt && mise run nextest  → fast gates"
      log_step "mise run dev-loop --advance   → VERIFY_FULL (if gates pass)"
      ;;
    VERIFY_FULL)
      log_step "mise run gate                → full validation"
      log_step "Verify all acceptance criteria documented"
      log_step "mise run dev-loop --advance   → CLOSE_OR_SPLIT"
      ;;
    CLOSE_OR_SPLIT)
      log_step "Move issue to issues/done/ or split into follow-ups"
      log_step "mise run dev-loop --advance   → RETRO"
      ;;
    RETRO)
      log_step "Write cycle report to reports/runs/<ts>/cycle_report.md"
      log_step "Log failure patterns if any"
      log_step "mise run dev-loop --advance   → back to SYNC (new cycle)"
      ;;
  esac
  printf '\n'

  # Validation hints
  if [[ "$current_idx" -ge 0 ]]; then
    log_info "Validation"
    log_step "mise run dev-loop --check     → validate state consistency"
    log_step "mise run check agent-state   → validate JSON schemas"
  fi
  printf '\n'
}

# --- Advance FSM ---
cmd_advance() {
  load_state

  local current_idx
  current_idx=$(fsm_index "$FSM") || die "Unknown FSM state: $FSM"
  local next_state
  next_state=$(fsm_next "$FSM")
  local now
  now=$(timestamp_utc)

  case "$FSM" in
    SYNC)
      # Validate state files first
      log_step "Validating state files..."
      "$TS2WASM_REPO_ROOT/scripts/manager.py" check-agent-state 2>/dev/null || \
        log_warn "agent-state check had issues (may be expected after reset)"
      ;;
    TRIAGE)
      local ready_count
      ready_count=$(count_ready_issues)
      if [[ "$ready_count" -eq 0 ]]; then
        die "Cannot advance: Ready queue is empty. Run coverage or add issues first."
      fi
      log_step "Ready queue has $ready_count items."
      ;;
    TASK_SELECT)
      if [[ "$TASK_STATUS" == "idle" || "$TASK_STATUS" == "" ]]; then
        die "Cannot advance: no task selected. Pick a Ready issue first."
      fi
      ;;
    PLAN)
      if [[ -z "$PLAN_PATH" || "$PLAN_PATH" == "null" ]]; then
        die "Cannot advance: plan_path is not set. Create an implementation plan first."
      fi
      if [[ ! -f "$TS2WASM_REPO_ROOT/$PLAN_PATH" ]]; then
        log_warn "Plan file not found at $PLAN_PATH — advancing anyway (ensure plan exists)."
      fi
      ;;
    PLAN_REVIEW_GATE)
      # No prereq validation — agent approves the plan
      ;;
    IMPLEMENT)
      # Check that code has been modified
      if git diff --quiet HEAD 2>/dev/null; then
        if git log --oneline -1 --format="%H" 2>/dev/null | grep -q .; then
          :  # There's at least a commit, that's OK
        else
          log_warn "No uncommitted or recent changes detected. Make sure implementation is done."
        fi
      fi
      ;;
    SELF_REVIEW_GATE)
      # No prereq validation — agent reviews own code
      ;;
    VERIFY_FAST)
      # Suggest running gates first
      log_step "Reminder: run 'mise run fmt && mise run nextest' before advancing."
      ;;
    VERIFY_FULL)
      log_step "Reminder: run 'mise run gate' and verify all acceptance criteria before advancing."
      ;;
    CLOSE_OR_SPLIT)
      # Check if issue was moved to done/ (advisory, not blocking)
      if [[ -n "$TASK_ISSUE_PATH" && "$TASK_ISSUE_PATH" != "null" ]]; then
        if [[ "$TASK_ISSUE_PATH" == *"issues/open/"* ]]; then
          log_warn "Issue still in issues/open/. Move to issues/done/ before closing."
        fi
      fi
      ;;
    RETRO)
      # Loop complete — back to SYNC
      log_info "Loop complete! Resetting to SYNC for next cycle."
      write_json_file "$PROJECT_STATE" "{\"version\": 1, \"fsm\": \"SYNC\", \"active_task_id\": null, \"updated_at\": \"$now\", \"milestone_id\": null, \"run_id\": null, \"plan_path\": null, \"verify_fast_streak_fails\": 0}"
      write_json_file "$CURRENT_TASK" "{\"id\": null, \"title\": null, \"status\": \"idle\", \"issue_path\": null, \"scope\": null, \"acceptance\": null, \"commands\": null, \"risk\": null, \"notes\": \"\"}"
      log_info "Reset to SYNC with idle task. Next: mise run dev-loop"
      return 0
      ;;
  esac

  if [[ "$next_state" == "COMPLETE" ]]; then
    log_info "All FSM states traversed. Run 'mise run dev-loop --advance' once more to reset."
    return 0
  fi

  # Write the new state with proper JSON quoting via Python
  local py_active py_milestone py_plan py_fails

  if [[ "$FSM" == "SYNC" ]]; then
    py_active="None"; py_milestone="None"; py_plan="None"; py_fails=0
  else
    if [[ "$ACTIVE_TASK_ID" == "null" || -z "$ACTIVE_TASK_ID" ]]; then
      py_active="None"
    else
      py_active="'$ACTIVE_TASK_ID'"
    fi
    if [[ "$MILESTONE_ID" == "null" || -z "$MILESTONE_ID" ]]; then
      py_milestone="None"
    else
      py_milestone="'$MILESTONE_ID'"
    fi
    if [[ "$PLAN_PATH" == "null" || -z "$PLAN_PATH" ]]; then
      py_plan="None"
    else
      py_plan="'$PLAN_PATH'"
    fi
    py_fails="${FAST_FAILS:-0}"
  fi

  python3 -c "
import json
data = {
    'version': 1,
    'fsm': '$next_state',
    'active_task_id': $py_active,
    'updated_at': '$now',
    'milestone_id': $py_milestone,
    'run_id': None,
    'plan_path': $py_plan,
    'verify_fast_streak_fails': $py_fails,
}
with open('$PROJECT_STATE', 'w') as f:
    json.dump(data, f, indent=2, ensure_ascii=False)
    f.write('\n')
"

  log_info "Advanced: ${FSM} → ${next_state}"
  printf '\n'
  log_step "What to do now:"
  log_dim "${FSM_DESC[$next_state]:-}"
  printf '\n'
}

# --- Reset to SYNC ---
cmd_reset() {
  local now
  now=$(timestamp_utc)

  log_warn "Resetting loop state to SYNC with idle task..."
  write_json_file "$PROJECT_STATE" "{\"version\": 1, \"fsm\": \"SYNC\", \"active_task_id\": null, \"updated_at\": \"$now\", \"milestone_id\": null, \"run_id\": null, \"plan_path\": null, \"verify_fast_streak_fails\": 0}"
  write_json_file "$CURRENT_TASK" "{\"id\": null, \"title\": null, \"status\": \"idle\", \"issue_path\": null, \"scope\": null, \"acceptance\": null, \"commands\": null, \"risk\": null, \"notes\": \"\"}"
  log_info "Reset complete. FSM is at SYNC."
  log_step "Run 'mise run dev-loop' to see status."
}

# --- Check consistency ---
cmd_check() {
  load_state

  local errors=0

  log_info "Checking state consistency..."

  # 1. FSM state is valid
  if fsm_index "$FSM" >/dev/null 2>&1; then
    log_step "FSM state: ${FSM} ✓"
  else
    log_error "FSM state: ${FSM} ✗ (not in FSM_ORDER)"
    errors=$((errors + 1))
  fi

  # 2. project_state.json and current_task.json are consistent about active_task
  if [[ "$ACTIVE_TASK_ID" != "null" && -n "$ACTIVE_TASK_ID" ]]; then
    if [[ "$TASK_STATUS" == "idle" ]]; then
      log_error "project_state has active_task_id=${ACTIVE_TASK_ID} but current_task.status=idle ✗"
      errors=$((errors + 1))
    else
      log_step "Active task: ${ACTIVE_TASK_ID} ✓"
    fi
  else
    if [[ "$TASK_STATUS" != "idle" && "$TASK_STATUS" != "" ]]; then
      log_error "No active_task_id in project_state but current_task.status=${TASK_STATUS} ✗"
      errors=$((errors + 1))
    else
      log_step "No active task (idle) ✓"
    fi
  fi

  # 3. Plan file exists if plan_path is set
  if [[ -n "$PLAN_PATH" && "$PLAN_PATH" != "null" ]]; then
    if [[ -f "$TS2WASM_REPO_ROOT/$PLAN_PATH" ]]; then
      log_step "Plan file: $PLAN_PATH ✓"
    else
      log_error "Plan file not found: $PLAN_PATH ✗"
      errors=$((errors + 1))
    fi
  fi

  # 4. Issue file exists if issue_path is set
  if [[ -n "$TASK_ISSUE_PATH" && "$TASK_ISSUE_PATH" != "null" ]]; then
    if [[ -f "$TS2WASM_REPO_ROOT/$TASK_ISSUE_PATH" ]]; then
      log_step "Issue file: $TASK_ISSUE_PATH ✓"
    else
      log_error "Issue file not found: $TASK_ISSUE_PATH ✗"
      errors=$((errors + 1))
    fi
  fi

  # 5. issue_patterns.json is loadable
  if [[ -f "$TS2WASM_REPO_ROOT/.agents/state/issue_scope_patterns.json" ]]; then
    if python3 -c "import json; json.load(open('$TS2WASM_REPO_ROOT/.agents/state/issue_scope_patterns.json'))" 2>/dev/null; then
      log_step "issue_scope_patterns.json: valid JSON ✓"
    else
      log_error "issue_scope_patterns.json: invalid JSON ✗"
      errors=$((errors + 1))
    fi
  fi

  # 6. Run schema validation if available
  if python3 -c "import json; json.load(open('${PROJECT_STATE}'))" 2>/dev/null; then
    log_step "project_state.json: valid JSON ✓"
  else
    log_error "project_state.json: invalid JSON ✗"
    errors=$((errors + 1))
  fi
  if python3 -c "import json; json.load(open('${CURRENT_TASK}'))" 2>/dev/null; then
    log_step "current_task.json: valid JSON ✓"
  else
    log_error "current_task.json: invalid JSON ✗"
    errors=$((errors + 1))
  fi

  printf '\n'
  if [[ "$errors" -eq 0 ]]; then
    log_info "All checks passed ✓"
  else
    log_error "${errors} check(s) failed — fix before proceeding."
  fi
  return "$errors"
}

# --- Main ---

main() {
  local cmd="${1:-status}"

  case "$cmd" in
    status|--status|-s)
      cmd_status
      ;;
    advance|--advance|-a)
      cmd_advance
      ;;
    reset|--reset|-r)
      cmd_reset
      ;;
    check|--check|-c)
      cmd_check
      ;;
    --help|-h)
      cat <<HELP
Usage:
  mise run dev-loop               Show current loop status
  mise run dev-loop --advance     Advance to next FSM state
  mise run dev-loop --reset       Reset to SYNC (clean slate)
  mise run dev-loop --check       Validate state consistency

FSM: SYNC → TRIAGE → TASK_SELECT → PLAN → PLAN_REVIEW_GATE → IMPLEMENT →
     SELF_REVIEW_GATE → VERIFY_FAST → VERIFY_FULL → CLOSE_OR_SPLIT →
     RETRO → (back to SYNC)
HELP
      ;;
    *)
      die "Unknown command: $cmd. Use --help for usage."
      ;;
  esac
}

main "$@"
