# Assignment: issue 060 coverage ramp continuation

Parent branch: `master`
Base: `8988f17`
Worktree: `/home/wogikaze/wgkz/ts2wasm-060-coverage-ramp2-20260428T022516Z`
Branch: `agent/060-coverage-ramp2-20260428T022516Z`
Issue: `issues/open/060-investigate-unknown-unsupported-cases.md`

## Scope

Continue reference-backed coverage expansion after test262 limit 750.

Primary goal:

- Raise one stored reference window safely, preferably `test262 --limit 1000`, or choose `tsc`/`tsgo` if test262 is blocked.
- Classify newly surfaced `unknown-unsupported` cases with evidence.
- Generate follow-up issues for new actionable classes and update issue index/artifacts.

Expected paths:

- `artifacts/coverage/`
- `issues/open/`
- `issues/index.md`
- `current-state.md` only if coverage facts change
- `scripts/lib/feature-labels.sh`
- `scripts/run/reference-coverage.py`
- `scripts/gen/issues-from-coverage.py`
- `reports/agents/agent-060-coverage-ramp2-20260428T022516Z/`
- `reports/runs/<timestamp>-060-coverage-ramp2/`

Do not edit compiler/runtime code in this worktree unless a tiny classifier bug blocks the run.

## Required validation

- `scripts/manager update-issue-index --check`
- `scripts/manager update-coverage-matrix --check`
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`
- `python -m py_compile scripts/run/reference-coverage.py scripts/gen/issues-from-coverage.py`

## Completion contract

Commit artifacts/issues/reports or a precise clean stop/progress report. Do not leave the branch dirty.

Final response must include exactly one parent event line:

`PARENT_EVENT: PROGRESS issue=060 branch=agent/060-coverage-ramp2-20260428T022516Z commit=<sha> merge_request=<yes|no>`
