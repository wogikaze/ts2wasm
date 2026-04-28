# Assignment: issue 236 dynamic computed keys on expression receivers

Child run id: `236-logical-assignment-computed-receivers-20260428T112342Z`
Worktree: `/home/wogikaze/wgkz/ts2wasm-236-logical-assignment-computed-receivers-20260428T112342Z`
Branch: `agent/236-logical-assignment-computed-receivers-20260428T112342Z`
Assigned issues: `236`
Issue order: `236`

## Objective

Continue issue 236 from the current master state. Static member logical assignment on expression receivers is now supported; implement or make validated design progress on the remaining dynamic computed key plus expression receiver form, such as `getObj()[key()] &&= rhs()`.

## Allowed files

- `crates/frontend/src/**`
- `crates/ir/src/**`
- `crates/backend-wasm/src/**`
- `crates/cli/tests/**`
- `fixtures/core-semantics/logical-assignment-*.ts`
- `issues/open/236-complete-logical-assignment-target-forms.md`
- `reports/runs/236-logical-assignment-computed-receivers-20260428T112342Z/`
- `reports/agents/236-logical-assignment-computed-receivers-20260428T112342Z/assignment.md`

## Forbidden files

- `artifacts/coverage/**`
- `scripts/**`
- `docs/**`
- JSON fixtures
- module-system fixtures
- unrelated issues

## Required workflow

1. Read `.agents/prompts/autonomous-child-worker.md`, `AGENTS.md`, issue 236, and this assignment.
2. Reproduce the current remaining unsupported case in `logical-assignment-member-unsupported.ts`.
3. Prefer a narrow executable slice for `getObj()[key()] &&= rhs()` preserving:
   - receiver evaluated exactly once
   - key evaluated exactly once
   - RHS short-circuited for `&&=`, `||=`, and `??=`
   - receiver and key temporaries rooted if backend evaluation may allocate
4. If implementation is unsafe, leave the unsupported diagnostic in place and commit a focused design/test progress note with evidence.
5. Add or update Node/iwasm differential fixture coverage with visible receiver/key/RHS side effects.
6. Run validation:
   - `cargo fmt --all --check`
   - `cargo nextest run -E 'test(logical_assignment)'`
   - direct `node` fixture commands
   - direct `cargo run -q -p ts2wasm-cli -- build ... && iwasm ...`
   - `scripts/manager check-issue-health`
   - `scripts/manager check-agent-state`
7. Run full `cargo nextest run` if closing issue 236 or changing shared backend behavior broadly.
8. Write `reports/runs/236-logical-assignment-computed-receivers-20260428T112342Z/cycle_report.md` and schema-valid `test_report.json`.
9. Attempt `scripts/manager discord-report --run-id 236-logical-assignment-computed-receivers-20260428T112342Z`; if webhook is unavailable, save payload/error artifacts and continue.
10. Commit all validated changes. Request merge from parent.

## Expected outcome

Use `PROGRESS` unless every issue 236 acceptance criterion is met and the full close workflow is complete.

## Parent event

End with exactly one parent event line:

`PARENT_EVENT: PROGRESS issue=236 branch=agent/236-logical-assignment-computed-receivers-20260428T112342Z commit=<hash> merge_request=yes`

Use `BLOCKED` with evidence if the combined receiver/key temporary design cannot be safely implemented in this cycle.
