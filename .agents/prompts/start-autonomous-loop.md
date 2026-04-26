# Start autonomous compiler development loop

Use this prompt to invoke the compiler-autonomy skill and begin the FSM-driven development cycle.

## Prompt

```
Start autonomous compiler development loop. Invoke the compiler-autonomy skill, read workflows/compiler_dev_fsm.md and state/current_task.json, then follow the FSM states (SYNC → TRIAGE → TASK_SELECT → PLAN → IMPLEMENT → VERIFY → RETRO).
```

## When to use

- When you want the agent to work through issues autonomously following the FSM contract
- When starting a new development cycle on the compiler
- When resuming work after a context switch

## What happens

1. Agent reads the FSM contract from `workflows/compiler_dev_fsm.md`
2. Agent checks current state from `state/current_task.json` and `state/project_state.json`
3. Agent follows the state machine: SYNC → TRIAGE → TASK_SELECT → PLAN → IMPLEMENT → VERIFY → RETRO
4. Agent updates state files and writes cycle reports to `reports/runs/`

## Detailed Steps

### Preflight (SYNC state)

- Read `workflows/compiler_dev_fsm.md` to understand FSM contract
- Read `state/current_task.json` to check if a task is already in progress
- Read `docs/11-shared-definitions.md` for workstreams and gates
- Read `docs/12-coding-standard.md` for coding standards
- Read `issues/index.md` to see the current issue queue
- Run `scripts/manager check-agent-state` to validate state files

### Task Selection (TASK_SELECT state)

- If `current_task.json` is idle, select a Ready issue from `issues/index.md`
- Prioritize P0 issues, then P1, then P2
- Check dependencies are satisfied (issue must not be in Blocked queue)
- Update `current_task.json` with selected task details

### Planning (PLAN state)

- Read the selected issue file from `issues/open/`
- Identify scope (allowed_files, forbidden_files)
- List acceptance criteria
- Define validation commands
- Create implementation plan

### Implementation (IMPLEMENT state)

- Make changes only within scope.allowed_files
- Do not modify forbidden files
- Follow coding standards from docs/12
- Run incremental validation as needed

### Verification (VERIFY state)

- Run all validation commands from the issue
- `scripts/manager fmt`
- `scripts/manager nextest` (full suite, no filters)
- Any fixture-specific tests (e.g., `iwasm fixture.wasm`)
- Verify all acceptance criteria with evidence

### Close (RETRO state)

**Prerequisites (all must be satisfied):**
- All acceptance criteria in the issue are satisfied with evidence in the repository
- Required verification / Close gate commands from the issue have been executed with exit 0 recorded
- No remaining tasks, STOP_IF, or blocked declarations remain in the issue body
- Implementation, verification, or documentation changes have been committed (not just mechanical checkbox updates)

**Close procedure:**

1. **Commit implementation changes** (if not already committed):
   - Run all validation commands from the issue
   - `scripts/manager fmt`
   - `scripts/manager nextest` (full suite, no filters)
   - Any fixture-specific tests (e.g., `iwasm fixture.wasm`)
   - Commit with descriptive message (e.g., `feat(area): implement NNN description`)
   - Record the commit hash

2. **Move issue to done**:
   - `git mv issues/open/<slug>.md issues/done/<slug>.md` (preserves history and path)
   - Update issue frontmatter: set Status to done
   - Add Close note with:
     - Date
     - Commit hash(es) providing evidence
     - Mapping of acceptance criteria to evidence
     - Verification commands executed and results

3. **Regenerate index and verify**:
   - `scripts/manager update-issue-index` to regenerate `issues/index.md`
   - `scripts/manager update-issue-index --check`
   - `scripts/manager check-issue-health`

4. **Final validation**:
   - `scripts/manager check-agent-state`
   - `scripts/manager check-repo-smoke`

5. **Commit close changes**:
   - Group into 1 commit or logically split into few commits (e.g., `chore(issues): close #NNN …`)
   - Ensure issue move and index update are in the same commit or logical sequence

6. **Cleanup and reporting**:
   - Clear `current_task.json` to idle state
   - Write cycle report to `reports/runs/<timestamp>/cycle_report.md`
   - Update failure_patterns.md if new failure pattern discovered
   - Update review_checklist.md if new guard needed

**False-done prevention:**
- Do not mark an issue as done without implementation-backed evidence
- Do not move issues with remaining `[ ]` checkboxes or open Status to done/
- Do not close issues based on text-only changes without verification
- Do not close issues with upstream dependencies still open

**Issue addition** (when Ready queue is low):

- Run reference-coverage with --detail flag:

  ```bash
  scripts/manager reference-coverage test262 --limit 500 --detail
  ```

- Pipe to gen-issues-from-coverage to auto-generate issues:

  ```bash
  scripts/manager reference-coverage test262 --limit 500 --detail | \
    scripts/manager gen-issues-from-coverage --suite test262
  ```

- Run `scripts/manager update-issue-index` to regenerate `issues/index.md`

- Commit changes:

  ```bash
  git add issues/ scripts/gen/issues-from-coverage.py
  git commit -m "feat(issues): add issues NNN-XXX from coverage"
  ```

**Coverage expansion** (when implementation targets decrease):
- Increase --limit in reference-coverage (e.g., 50 → 100 → 500 → 1000)
- Add new test suites if needed (tsc, tsgo)
- Auto-generate issues from expanded coverage
- Update issue index and commit

## Critical Requirements

**A loop is NOT complete until:**
- Full test suite passes (`scripts/manager nextest` with no filters)
- Issue file is moved from `issues/open/` to `issues/done/` with updated frontmatter
- `issues/index.md` is regenerated and verified
- All acceptance criteria are explicitly verified with evidence
- **Mechanical guards added** to `failure_patterns.md` or `review_checklist.md` if new patterns discovered
- **State files validated** with `scripts/manager check-agent-state`
- **Command outputs saved** to `reports/runs/<run_id>/stdout.log` and `stderr.log`

Do not mark an issue as done without completing these steps.

## Worktree/Branch Naming Convention

When starting a new task, create a worktree/branch following this pattern:

```
git worktree add ../ts2wasm-NNN-short-title NNNNNNNNNNNNN
# or
git checkout -b NNNNNNNNNNNNNN
```

Where:
- `NNN` is the issue ID (e.g., 012)
- `short-title` is a brief description (kebab-case, max 20 chars)
- `NNNNNNNNNNNNNN` is a 12-character timestamp or random suffix for uniqueness

Examples:
- `git worktree add ../ts2wasm-012-computed-prop 202604260728`
- `git checkout -b 012-computed-prop-202604260728`
