# Assignment: issue 060 coverage ramp2000

- Agent ID: 060-coverage-ramp2000-20260428T031700Z
- Worktree: /home/wogikaze/wgkz/ts2wasm-060-coverage-ramp2000-20260428T031700Z
- Branch: agent/060-coverage-ramp2000-20260428T031700Z
- Issue: 060 (`issues/open/060-investigate-unknown-unsupported-cases.md`)

## Goal

Continue the reference-backed test262 coverage ramp after parent merged limit 1500.
Preferred next slice is limit 2000 with `--detail`, classification of any new
unknown-unsupported cases from concrete reference paths/diagnostics, issue generation
if needed, matrix/index/artifact updates, validation, report, and commit.

## Boundaries

Allowed files:

- `scripts/lib/feature-labels.sh`
- `scripts/run/reference-coverage.py` only if classifier mechanics require it
- `artifacts/coverage/results/**`
- `artifacts/coverage/reference-coverage-matrix.md`
- `issues/open/**`
- `issues/index.md`
- `issues/open/060-investigate-unknown-unsupported-cases.md`
- `reports/agents/**`
- `reports/runs/**`
- `current-state.md` only if facts materially change

Forbidden files:

- `crates/frontend/src/**`
- `crates/cli/src/**`
- runtime/backend implementation files
- `docs/**` unless issue 060 acceptance forces it

## Required Validation

- `TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 2000 --detail`
- If unknowns appear, classify from reference evidence and rerun detail until `unknown-unsupported=0` or explicitly justified.
- Store JSON artifact with `--json`.
- `scripts/manager update-coverage-matrix`
- `scripts/manager update-issue-index` if issues changed
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`
- `cargo fmt --all --check` if scripts/Rust changed

## Reporting

Save a cycle report under `reports/runs/<timestamp>-060-coverage-ramp2000/`.
If Discord webhook is unavailable, save a deferred payload and continue.
End with one `PARENT_EVENT` line.
