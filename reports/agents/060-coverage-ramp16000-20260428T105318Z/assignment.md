# Assignment: issue 060 coverage ramp 16000

Child run id: `060-coverage-ramp16000-20260428T105318Z`
Worktree: `/home/wogikaze/wgkz/ts2wasm-060-coverage-ramp16000-20260428T105318Z`
Branch: `agent/060-coverage-ramp16000-20260428T105318Z`
Assigned issues: `060`
Issue order: `060`

## Objective

Continue issue 060 by expanding the stored test262 reference-coverage window from limit 15000 to limit 16000. This is a classification/artifact slice only.

## Allowed files

- `issues/open/060-investigate-unknown-unsupported-cases.md`
- `issues/open/*.md` only if new reference-backed follow-up issues are required by newly surfaced classifications
- `issues/index.md`
- `scripts/lib/feature-labels.sh` only if the 16000 detail run exposes `unknown-unsupported`
- `scripts/run/reference-coverage.py` only if classifier support is clearly required
- `artifacts/coverage/results/test262.json`
- `artifacts/coverage/reference-coverage-matrix.md`
- `current-state.md`
- `reports/runs/060-coverage-ramp16000-20260428T105318Z/`
- `reports/agents/060-coverage-ramp16000-20260428T105318Z/assignment.md`

## Forbidden files

- `crates/**`
- `fixtures/**`
- docs outside `current-state.md`
- unrelated issues

## Required workflow

1. Read `.agents/prompts/autonomous-child-worker.md`, `AGENTS.md`, issue 060, and this assignment.
2. Run a detail check:
   `TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 16000 --detail`
3. If `unknown-unsupported` appears, inspect the reference-backed paths, add narrowly justified classifier labels and follow-up issues, then rerun the detail check.
4. Store JSON atomically:
   `tmp=$(mktemp artifacts/coverage/results/test262.json.tmp.XXXXXX); TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 16000 --json > "$tmp"; mv "$tmp" artifacts/coverage/results/test262.json`
5. Run `scripts/manager update-coverage-matrix`.
6. Update issue 060 progress evidence with exact commands, counts, blocked status, and remaining acceptance risk.
7. Update `current-state.md` only if the stored coverage facts changed.
8. Run validation:
   - `scripts/manager update-coverage-matrix --check`
   - `scripts/manager update-issue-index` if issues changed
   - `scripts/manager check-issue-health`
   - `scripts/manager check-agent-state`
9. Write `reports/runs/060-coverage-ramp16000-20260428T105318Z/cycle_report.md` and a schema-valid `test_report.json`.
10. Attempt `scripts/manager discord-report --run-id 060-coverage-ramp16000-20260428T105318Z`; if webhook is unavailable, save payload/error artifacts and continue.
11. Commit all validated changes. Request merge from parent.

## Expected outcome

Use `PROGRESS` unless issue 060 fully satisfies all acceptance criteria. Do not mark issue 060 done for a single test262 ramp.

## Parent event

End with exactly one parent event line:

`PARENT_EVENT: PROGRESS issue=060 branch=agent/060-coverage-ramp16000-20260428T105318Z commit=<hash> merge_request=yes`

Use `BLOCKED` only if the reference command cannot run after one retry and no safe artifact progress can be committed.
