# Assignment: 060 coverage ramp to 12000

- Run ID: `060-coverage-ramp12000-20260428T080100Z`
- Branch: `agent/060-coverage-ramp12000-20260428T080100Z`
- Worktree: `/home/wogikaze/wgkz/ts2wasm-060-coverage-ramp12000-20260428T080100Z`
- Issue: `issues/open/060-investigate-unknown-unsupported-cases.md`
- Slice: continue issue 060 by ramping test262 reference coverage from stored limit 11000 to limit 12000.

## Scope

- Do not implement compiler features.
- If newly surfaced `unknown-unsupported` entries appear, classify them with precise existing/new feature labels or generate reference-backed follow-up issues only when a classifier cannot be made safely.
- If zero `unknown-unsupported`, record validated PROGRESS and leave issue 060 open.

## Allowed files

- `artifacts/coverage/results/test262.json`
- `artifacts/coverage/reference-coverage-matrix.md`
- `current-state.md`
- `issues/open/060-investigate-unknown-unsupported-cases.md`
- `issues/open/` and `issues/index.md` only if follow-up issues are generated
- `scripts/` only if a classifier change is absolutely required
- `reports/runs/060-coverage-ramp12000-20260428T080100Z/**`
- `reports/agents/060-coverage-ramp12000-20260428T080100Z/assignment.md`

## Forbidden files

- Compiler/runtime implementation files unless classifier script changes explicitly require them
- `docs/**`
- Unrelated issue files

## Required validation

```sh
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 12000 --detail
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 12000 --json > artifacts/coverage/results/test262.json
scripts/manager update-coverage-matrix
scripts/manager update-coverage-matrix --check
scripts/manager check-issue-health
scripts/manager check-agent-state
scripts/manager discord-report --run-id 060-coverage-ramp12000-20260428T080100Z
```

## Reporting

- Save validation evidence under `reports/runs/060-coverage-ramp12000-20260428T080100Z/`.
- If Discord reporting fails or webhook configuration is absent, save deferred payload/error under the run directory and continue.
- Commit validated progress on this branch.
- Do not merge to parent.
