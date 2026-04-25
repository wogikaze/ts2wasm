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
- Test report is generated and saved to reports/runs/
- Failure patterns are recorded in failure pattern DB if applicable
- Cycle report is written with evidence and next steps

## Mise: run before you exit VERIFY* / report RETRO (required)

**The autonomy loop is only honest if the gates were actually executed; run these and fail the step on red.** Without `mise`, use `scripts/manager` with the same name. First time: `mise trust` ([docs](https://mise.jdx.dev/cli/trust.html)).

- `current_task.json` または issue が示す `commands.fast` / `commands.full` 相当（通常は少なくとも `mise run fmt` と `mise run nextest`）
- Issue / index と整合: `mise run check-issue-queue`（`issues` を扱う場合は `mise run update-issue-index` も）
- 軽い一括: `mise run check-repo-smoke`

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
mise run fmt
mise run nextest
mise run check-issue-queue
# Generate test report to reports/runs/<run_id>/test_report.json
# Write cycle report with evidence
# Update current_task.json with verification results
```

### Commands run

```bash
mise run fmt
mise run nextest
mise run check-issue-queue
mise run check-repo-smoke
```
