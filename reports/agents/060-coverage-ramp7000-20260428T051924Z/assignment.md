# Child Assignment: issue 060 coverage ramp 7000

- Child id: `060-coverage-ramp7000-20260428T051924Z`
- Worktree: `/home/wogikaze/wgkz/ts2wasm-060-coverage-ramp7000-20260428T051924Z`
- Branch: `agent/060-coverage-ramp7000-20260428T051924Z`
- Assigned issue: `060`

## Required first checks

Run `pwd`, `git status --short --branch`, and confirm this worktree/branch before editing. You are not alone in the codebase; do not revert edits from other worktrees.

## Scope

Ramp stored test262 reference coverage from limit 6000 to limit 7000.

- Run detail coverage first and inspect new `unknown-unsupported` entries.
- If new unknowns appear, classify with reference-backed labels and create/update issues.
- If no unknowns appear, update artifacts and evidence only.
- Do not edit compiler Rust code unless classifier extraction itself is wrong.

## Allowed files

- `scripts/lib/feature-labels.sh` only if new labels are required
- `scripts/run/reference-coverage.py` only if classification extraction requires a script fix
- `artifacts/coverage/results/test262.json`
- `artifacts/coverage/reference-coverage-matrix.md`
- `issues/open/060-investigate-unknown-unsupported-cases.md`
- new/updated issue files only if backed by new coverage detail
- `issues/index.md`
- `current-state.md`
- `.agents/state/last_run.json`
- `reports/runs/060-coverage-ramp7000-20260428T051924Z/`

## Expected validation

- `TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 7000 --detail`
- `TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 7000 --json > artifacts/coverage/results/test262.json`
- `scripts/manager update-coverage-matrix`
- `scripts/manager update-issue-index`
- `scripts/manager update-coverage-matrix --check`
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`

End with exactly one `PARENT_EVENT:` line.
