---
name: compiler-autonomy
description: Use for autonomous compiler-dev runs. Covers FSM, current_task.json, verification reports, failure pattern DB, re-prevention.
---

# Compiler dev autonomy

This skill is the **thin entry** for the autonomous build/test loop. The authoritative contract is large; it lives in the workflow + state files, not in this `SKILL.md` alone.

## Success Criteria

The autonomous loop is considered complete when:
- FSM state transition is validated against workflow rules
- current_task.json is updated with verification results
- All required gates (fmt, nextest, check-issue-queue) pass
- **All acceptance criteria from the issue are explicitly verified and documented**
- Test report is generated and saved to reports/runs/
- Failure patterns are recorded in failure pattern DB if applicable
- Cycle report is written with evidence and next steps

## Manager: run before you exit VERIFY* / report RETRO (required)

**The autonomy loop is only honest if the gates were actually executed; run these and fail the step on red.** Use `scripts/manager` as the primary repo entry. `mise run <task>` is optional sugar for the same tasks.

- `current_task.json` または issue が示す `commands.fast` / `commands.full` 相当（通常は少なくとも `scripts/manager fmt` と `scripts/manager nextest`）
- **CRITICAL: Full test suite must pass, not just filtered tests. If `cargo nextest run` fails, you MUST investigate before marking done.**
- Issue / index と整合: `scripts/manager check-issue-queue`（`issues` を扱う場合は `scripts/manager update-issue-index` も）
- 軽い一括: `scripts/manager check-repo-smoke`

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
3. **Regenerate issues index**: Run `scripts/manager update-issue-index`
4. **Verify index consistency**: Run `scripts/manager check-issue-index` to ensure the index reflects the change
5. **Document completion evidence**: In the issue file or cycle report, explicitly state how each acceptance criterion was verified with specific commands/outputs.

**Failure to complete these steps means the issue is NOT done.**

## Close (RETRO state)

- Update issue file status to "done" and add completion evidence
- Move issue file from `issues/open/` to `issues/done/`
- Run `scripts/manager update-issue-index` to regenerate `issues/index.md`
- Clear `current_task.json` to idle state
- Write cycle report to `reports/runs/<timestamp>/cycle_report.md`
- **REQUIRED**: If new failure pattern discovered, add to `failure_patterns.md` with mechanical guards
- **REQUIRED**: If new guard needed, add to `review_checklist.md`
- **REQUIRED**: Run `scripts/manager check-agent-state` to validate state files

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
# Run tests manually
cargo nextest run
# Check fmt manually
cargo fmt --all --check
# No cycle report generated
```

### After: Follow autonomous loop

```bash
# Read current_task.json for FSM state
# Run required gates
scripts/manager fmt
scripts/manager nextest
scripts/manager check-issue-queue
# Generate test report to reports/runs/<run_id>/test_report.json
# Write cycle report with evidence
# Update current_task.json with verification results
```

### Commands run

```bash
scripts/manager fmt
scripts/manager nextest
scripts/manager check-issue-queue
scripts/manager check-repo-smoke
```

## Post-change auto-execution

After completing issue work (code changes, issue file updates, cycle report), automatically:
1. Run `scripts/manager fmt`, `scripts/manager nextest`, and `scripts/manager check-issue-queue`
2. Commit changes with auto-generated commit message based on issue completion evidence
