#!/usr/bin/env bash
set -Eeuo pipefail

ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"

MODEL="${MODEL:-opencode-go/kimi-k2.6}"
PARALLELISM="${PARALLELISM:-6}"
MAX_PARALLELISM="${MAX_PARALLELISM:-10}"
RUN_ID="${RUN_ID:-opencode-ralph-$(date +%Y%m%d-%H%M%S)}"
LOG_DIR="reports/runs/$RUN_ID"
STATE=".agents/state/milestones.json"

mkdir -p "$LOG_DIR"/{logs,plans,assignments,events,worktrees}
mkdir -p .agents/state

count_open() {
  find issues/open -type f -name '*.md' 2>/dev/null | wc -l | tr -d ' '
}

run_opencode() {
  local title="$1"
  local prompt="$2"
  local out="$3"

  opencode run \
    -m "$MODEL" \
    --dangerously-skip-permissions \
    --file "$prompt" \
    --format json \
    --title "$title" \
    >"$out.jsonl" \
    2>"$out.stderr"
}

preflight() {
  git status --short --branch | tee "$LOG_DIR/parent-status.txt"
  mise run update-issue-index -- --check || mise run update-issue-index || true
  mise run check issues || true
}

ensure_plan() {
  if [ ! -s "$STATE" ] || ! grep -q '"tasks"' "$STATE"; then
    cat >"$LOG_DIR/planner-input.md" <<EOF
Run id: $RUN_ID

Create or update .agents/state/milestones.json from current issues/open/*.md.

Current open issues:
$(find issues/open -type f -name '*.md' | sort)

Use reports/runs/$RUN_ID/plans/ for plan docs.
EOF

    run_opencode \
      "$RUN_ID-planner" \
      ".agents/prompts/opencode-planner.md" \
      "$LOG_DIR/logs/planner"
  fi
}

next_wave_json() {
  python3 - "$STATE" "$PARALLELISM" <<'PY'
import json, sys
path = sys.argv[1]
limit = int(sys.argv[2])
with open(path, encoding="utf-8") as f:
    s = json.load(f)

for m in s.get("milestones", []):
    if m.get("done"):
        continue
    tasks = [t for t in m.get("tasks", []) if t.get("status", "todo") in ("todo", "progress")]
    if not tasks:
        continue
    w = min(t.get("wave", 999999) for t in tasks)
    wave_tasks = [t for t in tasks if t.get("wave") == w][:limit]
    print(json.dumps({"milestone": m["id"], "wave": w, "tasks": wave_tasks}, ensure_ascii=False))
    sys.exit(0)

print(json.dumps({"milestone": None, "wave": None, "tasks": []}, ensure_ascii=False))
PY
}

make_builder_assignment() {
  local task_json="$1"
  local worker="$2"
  local wt="$3"
  local branch="$4"
  local assignment="$5"

  python3 - "$task_json" "$worker" "$wt" "$branch" "$assignment" <<'PY'
import json, sys
task = json.loads(sys.argv[1])
worker, wt, branch, out = sys.argv[2:6]

issue_files = task.get("issue_files", [])
allowed = task.get("allowed_files", [])
forbidden = task.get("forbidden_files", [])
validation = task.get("validation", [])

with open(out, "w", encoding="utf-8") as f:
    f.write(f"""# Builder Assignment: {worker}

Worker: {worker}
Task: {task.get("id")}
Title: {task.get("title")}
Worktree: {wt}
Branch: {branch}

Issue files:
{chr(10).join("- " + x for x in issue_files)}

Allowed files:
{chr(10).join("- " + x for x in allowed)}

Forbidden files:
{chr(10).join("- " + x for x in forbidden)}

Required validation:
{chr(10).join("- " + x for x in validation)}

Read:
- AGENTS.md
- .agents/prompts/opencode-builder.md
- .agents/state/milestones.json
- all issue files above

Execute the assigned task only.
Do not ask the human.
Do not merge.
Do not push.
Commit validated work.

End with exactly one BUILDER_EVENT line.
""")
PY
}

launch_builders() {
  local wave_json="$1"
  local wave_dir="$2"
  mkdir -p "$wave_dir"

  python3 - "$wave_json" "$wave_dir/tasks.jsonl" <<'PY'
import json, sys
w = json.loads(sys.argv[1])
with open(sys.argv[2], "w", encoding="utf-8") as f:
    for t in w.get("tasks", []):
        f.write(json.dumps(t, ensure_ascii=False) + "\n")
PY

  : >"$wave_dir/workers.tsv"

  local i=0
  while IFS= read -r task_json; do
    i=$((i + 1))

    local task_id short stamp worker wt branch assignment
    task_id="$(python3 -c 'import json,sys;print(json.loads(sys.argv[1])["id"])' "$task_json")"
    short="$(echo "$task_id" | tr -c 'A-Za-z0-9._-' '-')"
    stamp="$(date +%Y%m%d%H%M%S)"
    worker="builder-$i-$short"
    wt="../ts2wasm-$worker-$stamp"
    branch="agent/$worker-$stamp"
    assignment="$wave_dir/$worker.assignment.md"

    git worktree add "$wt" -b "$branch" HEAD 2>>"$LOG_DIR/worktree-errors.log"

    if mise tasks ls 2>/dev/null | grep -q '^link-reference'; then
      mise run link-reference -- "$wt" 2>/dev/null || true
    fi

    make_builder_assignment "$task_json" "$worker" "$wt" "$branch" "$assignment"
    cp "$assignment" "$wt/assignment.md"

    (
      wt_abs="$(cd "$wt" && pwd)"
      cd "$wt_abs"
      opencode run \
        -m "$MODEL" \
        --dangerously-skip-permissions \
        --file assignment.md \
        --file .agents/prompts/opencode-builder.md \
        --format json \
        --title "$RUN_ID-$worker" \
        >"$ROOT/$wave_dir/$worker.jsonl" \
        2>"$ROOT/$wave_dir/$worker.stderr"

      rc=$?
      git status --short --branch >"$ROOT/$wave_dir/$worker.git-status" 2>&1 || true
      git log --oneline --decorate --max-count=30 >"$ROOT/$wave_dir/$worker.git-log" 2>&1 || true
      echo "$rc" >"$ROOT/$wave_dir/$worker.exit"
    ) &

    echo "$worker|$task_id|$wt|$branch|$assignment" >>"$wave_dir/workers.tsv"
  done <"$wave_dir/tasks.jsonl"

  wait
}

run_verifier() {
  local wave_json="$1"
  local wave_dir="$2"

  cat >"$wave_dir/verifier-input.md" <<EOF
Run id: $RUN_ID
Wave:
$wave_json

Builder workers:
$(cat "$wave_dir/workers.tsv")

You are the verifier.
Inspect all listed worktrees and branches.
Merge/cherry-pick only safe commits.
Update .agents/state/milestones.json.
Update issues only when close evidence passes.
Run validation.
Commit integration state.
EOF

  opencode run \
    -m "$MODEL" \
    --dangerously-skip-permissions \
    --file .agents/prompts/opencode-verifier.md \
    --file "$wave_dir/verifier-input.md" \
    --format json \
    --title "$RUN_ID-verifier" \
    >"$wave_dir/verifier.jsonl" \
    2>"$wave_dir/verifier.stderr"
}

cleanup_wave() {
  local wave_dir="$1"

  if [ -f "$wave_dir/workers.tsv" ]; then
    while IFS='|' read -r worker task_id wt branch assignment; do
      [ -z "$worker" ] && continue
      [ -z "$wt" ] && continue
      if [ -d "$wt" ]; then
        git worktree remove "$wt" --force 2>/dev/null || true
      fi
      git branch -D "$branch" 2>/dev/null || true
    done <"$wave_dir/workers.tsv"
  fi
}

main() {
  preflight

  local loop=0
  while [ "$(count_open)" -gt 0 ]; do
    loop=$((loop + 1))

    ensure_plan

    local wave_json milestone wave wave_dir tasks_len
    wave_json="$(next_wave_json)"
    milestone="$(python3 -c 'import json,sys;print(json.loads(sys.argv[1]).get("milestone"))' "$wave_json" 2>/dev/null || echo None)"
    wave="$(python3 -c 'import json,sys;print(json.loads(sys.argv[1]).get("wave"))' "$wave_json" 2>/dev/null || echo 0)"
    tasks_len="$(python3 -c 'import json,sys;print(len(json.loads(sys.argv[1]).get("tasks",[])))' "$wave_json" 2>/dev/null || echo 0)"

    if [ "$milestone" = "None" ] || [ "$tasks_len" = "0" ]; then
      rm -f "$STATE"
      ensure_plan
      continue
    fi

    wave_dir="$LOG_DIR/wave-$loop-$milestone-$wave"
    mkdir -p "$wave_dir"
    echo "$wave_json" >"$wave_dir/wave.json"

    echo "=== wave $loop milestone=$milestone wave=$wave tasks=$tasks_len ===" | tee -a "$LOG_DIR/waves.log"

    launch_builders "$wave_json" "$wave_dir"
    run_verifier "$wave_json" "$wave_dir"
    cleanup_wave "$wave_dir"

    mise run update-issue-index || true
    mise run update-issue-index -- --check || true
    mise run check issues || true
    git status --short --branch | tee "$wave_dir/parent-status-after.txt"

    if [ "$loop" -ge 1000 ]; then
      echo "ORCHESTRATOR_STATUS: NEED_HUMAN_REVIEW"
      echo "reason=max_loops_reached" | tee -a "$LOG_DIR/final_status.txt"
      exit 2
    fi
  done

  mise run update-issue-index -- --check || true
  mise run check issues || true
  mise run fmt || cargo fmt --all --check || true
  mise run check || true

  echo "ORCHESTRATOR_STATUS: CLEAN_STOP"
  git rev-parse HEAD | tee "$LOG_DIR/final_commit.txt"
}

main "$@"
