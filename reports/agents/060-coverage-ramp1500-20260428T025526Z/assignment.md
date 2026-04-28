# Assignment

- Agent ID: 060-coverage-ramp1500-20260428T025526Z
- Worktree: /home/wogikaze/wgkz/ts2wasm-060-coverage-ramp1500-20260428T025526Z
- Branch: agent/060-coverage-ramp1500-20260428T025526Z
- Issue: 060 (`issues/open/060-investigate-unknown-unsupported-cases.md`)
- Goal: continue reference-backed test262 coverage ramp after parent merged limit 1250; preferred next slice is limit 1500 with detail diagnostics, classification of new unknown-unsupported cases, issue generation if needed, matrix/index/artifact updates, validation, report, and commit.

## Boundaries

- Allowed files: classifier scripts if required, coverage artifacts/results, coverage matrix, issues, reports, and current-state only for material fact changes.
- Forbidden files: compiler/runtime/backend implementation paths and docs unless issue 060 acceptance requires them.

## Required Validation

- `TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 1500 --detail`
- JSON artifact from the detail run
- `scripts/manager update-coverage-matrix`
- `scripts/manager update-issue-index` if issues changed
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`
- `cargo fmt --all --check` if scripts or Rust changed
