# Assignment: issue 236 non-identifier logical assignment receivers

Child run id: `236-logical-assignment-receivers-20260428T105318Z`
Worktree: `/home/wogikaze/wgkz/ts2wasm-236-logical-assignment-receivers-20260428T105318Z`
Branch: `agent/236-logical-assignment-receivers-20260428T105318Z`
Assigned issues: `236`
Issue order: `236`

## Objective

Make one safe implementation or design-progress slice for issue 236 around non-identifier member receivers, such as `getObj().value ||= rhs()`, while preserving JavaScript single-evaluation and short-circuit semantics.

## Allowed files

- `crates/frontend/src/**`
- `crates/ir/src/**`
- `crates/backend-wasm/src/**`
- `crates/cli/tests/**`
- `fixtures/core-semantics/logical-assignment-*.ts`
- `issues/open/236-complete-logical-assignment-target-forms.md`
- `reports/runs/236-logical-assignment-receivers-20260428T105318Z/`
- `reports/agents/236-logical-assignment-receivers-20260428T105318Z/assignment.md`

## Forbidden files

- `docs/**`
- `artifacts/coverage/**`
- issue 233 module files unless only broad shared test harness names are read
- unrelated fixtures

## Required workflow

1. Read `.agents/prompts/autonomous-child-worker.md`, `AGENTS.md`, issue 236, and this assignment.
2. Reproduce the current unsupported behavior using the narrow existing logical-assignment fixtures/tests.
3. Prefer one executable positive slice for static member on non-identifier receiver, e.g. `getObj().value ||= rhs()`, `&&=`, or `??=`. If implementation is unsafe, produce a source-backed design-progress commit that narrows diagnostics/tests without weakening expectations.
4. Preserve:
   - receiver evaluated exactly once
   - computed key evaluated exactly once when in scope
   - RHS short-circuited for `&&=`, `||=`, and `??=`
5. Add or narrow Node/iwasm differential fixtures with observable receiver/RHS evaluation counts.
6. Run validation:
   - `cargo fmt --all --check`
   - `cargo nextest run -E 'test(logical_assignment)'`
   - relevant direct `node` fixture commands
   - relevant `cargo run -q -p ts2wasm-cli -- build ... && iwasm ...`
   - `scripts/manager check-issue-health`
   - `scripts/manager check-agent-state`
7. Run full `cargo nextest run` only if closing issue 236 or changing shared backend semantics broadly.
8. Write `reports/runs/236-logical-assignment-receivers-20260428T105318Z/cycle_report.md` and schema-valid `test_report.json`.
9. Attempt Discord report; if webhook is unavailable, save payload/error artifacts and continue.
10. Commit all validated changes. Request merge from parent.

## Expected outcome

Use `PROGRESS` unless every issue 236 acceptance criterion is met and full close workflow is complete.

## Parent event

End with exactly one parent event line:

`PARENT_EVENT: PROGRESS issue=236 branch=agent/236-logical-assignment-receivers-20260428T105318Z commit=<hash> merge_request=yes`

Use `BLOCKED` with evidence if implementation needs a larger reference-temporary design.
