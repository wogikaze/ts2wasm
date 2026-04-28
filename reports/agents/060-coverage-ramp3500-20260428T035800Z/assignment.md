# Assignment: issue 060 coverage ramp3500

- Agent ID: 060-coverage-ramp3500-20260428T035800Z
- Worktree: /home/wogikaze/wgkz/ts2wasm-060-coverage-ramp3500-20260428T035800Z
- Branch: agent/060-coverage-ramp3500-20260428T035800Z
- Issue: 060 (`issues/open/060-investigate-unknown-unsupported-cases.md`)

## Goal

Continue the reference-backed test262 coverage ramp after parent merged limit 3000.
Preferred next slice is limit 3500 with `--detail`, classification of new
unknown-unsupported cases from concrete reference paths/diagnostics, issue generation
if needed, matrix/index/artifact updates, validation, report, and commit.

## Boundaries

Allowed files: coverage artifacts, coverage matrix, issue files/index, classifier
scripts if needed, reports, and `current-state.md` only for factual coverage state.
Forbidden files: compiler/frontend/runtime implementation and docs unless classification
mechanics require otherwise.

## Required Validation

- `TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 3500 --detail`
- Store JSON artifact with `--json`.
- `scripts/manager update-coverage-matrix`
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`
- `cargo fmt --all --check` only if scripts/Rust changed

End with one `PARENT_EVENT` line.
