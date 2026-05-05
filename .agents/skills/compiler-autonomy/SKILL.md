---
name: compiler-autonomy
description: Use for autonomous compiler-dev runs. Covers FSM, current_task.json, verification reports, failure pattern DB, re-prevention.
---

# Compiler dev autonomy

This skill is the **thin entry** for the autonomous build/test loop. The authoritative contract is large; it lives in the workflow + state files, not in this `SKILL.md` alone.

## Entry

**When this skill loads (at session start, or when idle detected), ALWAYS run:**

```bash
mise run dev-loop
```

This shows the current FSM state and what to do next. The dev-loop script (`scripts/dev/dev-loop.sh`) is the primary loop driver.

## Loop interface

```bash
mise run dev-loop               # Show status + suggested actions
mise run dev-loop --advance     # Advance to next FSM state
mise run dev-loop --commit      # Commit changes (auto-generated message)
mise run dev-loop --commit "feat: X"  # Commit with custom message
mise run dev-loop --reset       # Reset to SYNC (clean slate)
mise run dev-loop --check       # Validate state consistency
```

## Success Criteria

The autonomous loop is considered complete when:
- FSM state transition is validated against workflow rules
- current_task.json is updated with verification results
- All required gates (fmt, nextest, check issues) pass
- **All acceptance criteria from the issue are explicitly verified and documented**
- Test report is generated and saved to reports/runs/
- Failure patterns are recorded in failure pattern DB if applicable
- Cycle report is written with evidence and next steps

## Manager: run before you exit VERIFY* / report RETRO (required)

**The autonomy loop is only honest if the gates were actually executed; run these and fail the step on red.** Use `mise` as the primary repo entry. `mise run <task>` is optional sugar for the same tasks.

- `current_task.json` または issue が示す `commands.fast` / `commands.full` 相当（通常は少なくとも `mise run fmt` と `mise run nextest`）
- **CRITICAL: Full test suite must pass, not just filtered tests. If `cargo nextest run` fails, you MUST investigate before marking done.**
- Issue / index と整合: `mise run check issues`（`issues` を扱う場合は `mise run update-issue-index` も）
- 軽い一括: `mise run check`

## Acceptance Criteria Verification (CRITICAL)

**Before marking VERIFY_FULL as complete, you MUST:**

1. Read the issue's "Acceptance Criteria" section
2. For each criterion, verify it with explicit evidence:
   - Run the validation commands from the issue
   - Check the actual output matches expected behavior
   - Add test fixtures if required by the issue
3. Document the verification in decision_log.md with specific evidence
4. If any criterion is not met, DO NOT mark the task as done

**Common failure modes:**
- Assuming "tests pass" means "acceptance criteria met"
- Skipping fixture requirements
- Not verifying alias behavior (e.g., --emit-capabilities vs --emit-manifest)
- Leaving unused code (e.g., transitional schemas)
- Running filtered tests instead of full suite when pre-existing failures exist

## Issue Completion Checklist (REQUIRED before marking done)

**Before marking an issue as complete, you MUST:**

1. **Verify full test suite passes**: Run `cargo nextest run` (no filters). If there are pre-existing failures, document them in the cycle report and confirm they are unrelated to your changes.
2. **Update issue file**: Move issue from `issues/open/` to `issues/done/` and update frontmatter:
   - Change `Status: open` → `Status: done`
   - Add `Completed: <date>` field
3. **Regenerate issues index**: Run `mise run update-issue-index`
4. **Verify index consistency**: Run `mise run check issue-index` to ensure the index reflects the change
5. **Document completion evidence**: In the issue file or cycle report, explicitly state how each acceptance criterion was verified with specific commands/outputs.

**Failure to complete these steps means the issue is NOT done.**

## Close (RETRO state)

- Update issue file status to "done" and add completion evidence
- Move issue file from `issues/open/` to `issues/done/`
- Run `mise run update-issue-index` to regenerate `issues/index.md`
- Clear `current_task.json` to idle state
- Write cycle report to `reports/runs/<timestamp>/cycle_report.md`
- **REQUIRED**: If new failure pattern discovered, add to `failure_patterns.md` with mechanical guards
- **REQUIRED**: If new guard needed, add to `review_checklist.md`
- **REQUIRED**: Run `mise run check agent-state` to validate state files
<<<<<<< HEAD
- **REQUIRED**: Run `mise run dev-loop --advance` to advance RETRO → SYNC (restart loop)
||||||| merged common ancestors
- **REQUIRED**: Run `scripts/manager check-agent-state` to validate state files
=======
>>>>>>> origin/master

## Read order

1. `../../workflows/compiler_dev_fsm.md` — FSM, failure edges, done/forbidden, RETRO rules
2. `../../state/current_task.json` (and `../../state/project_state.json`) when work is in progress
3. `references/coding_standard.md` — project-bound rules
4. `references/review_checklist.md` — pre-verify gate
5. `references/failure_patterns.md` — FP-NNN failure DB (curated, do not bloat)

## State and artifacts

- JSON Schemas: `../../state/schemas/`
- Example `test_report`: `../../state/examples/test_report.json`
- Run output location: `reports/runs/<run_id>/test_report.json` (repo root)

## Related Skills

- milestone: for roadmap / vertical slices
- gatekeeper-review: for merge gates
- scripts-workflow: for adding automation that implements a guard
- issue-state-sync: for syncing issue state after autonomous runs

## Example Usage

### Before: Manual autonomous run

```bash
# Check status manually
mise run dev-loop
```

### After: Follow autonomous loop

```bash
<<<<<<< HEAD
# Start every session
mise run dev-loop

# Each step
mise run dev-loop --advance     # move to next FSM state
mise run fmt                    # fast gate
mise run nextest                # fast gate
mise run gate                   # full validation
mise run update-issue-index     # after closing issues
mise run dev-loop --check       # validate consistency
mise run dev-loop --advance     # RETRO → SYNC, loop restarts
||||||| merged common ancestors
# Read current_task.json for FSM state
# Run required gates
scripts/manager fmt
scripts/manager nextest
scripts/manager check-issue-health
# Generate test report to reports/runs/<run_id>/test_report.json
# Write cycle report with evidence
# Update current_task.json with verification results
=======
# Read current_task.json for FSM state
# Run required gates
mise run fmt
mise run nextest
mise run check issues
# Generate test report to reports/runs/<run_id>/test_report.json
# Write cycle report with evidence
# Update current_task.json with verification results
>>>>>>> origin/master
```

### Commands run per cycle

```bash
<<<<<<< HEAD
mise run dev-loop
mise run dev-loop --advance   # SYNC → TRIAGE
mise run dev-loop --advance   # TRIAGE → TASK_SELECT
# ... through all FSM states ...
mise run dev-loop --advance   # RETRO → SYNC
||||||| merged common ancestors
scripts/manager fmt
scripts/manager nextest
scripts/manager check-issue-health
scripts/manager check-repo-smoke
=======
mise run fmt
mise run nextest
mise run check issues
mise run check
>>>>>>> origin/master
```

## Post-change auto-execution

After completing issue work (code changes, issue file updates, cycle report), automatically:
1. Run `mise run fmt`, `mise run nextest`, and `mise run check issues`
<<<<<<< HEAD
2. Run `mise run dev-loop --check` to validate state
3. Run `mise run dev-loop --advance` to progress the FSM
4. Commit changes with auto-generated commit message based on issue completion evidence
||||||| merged common ancestors
1. Run `scripts/manager fmt`, `scripts/manager nextest`, and `scripts/manager check-issue-health`
2. Commit changes with auto-generated commit message based on issue completion evidence
=======
2. Commit changes with auto-generated commit message based on issue completion evidence
>>>>>>> origin/master
