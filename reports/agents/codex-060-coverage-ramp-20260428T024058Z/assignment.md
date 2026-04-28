# Assignment: issue 060 coverage ramp

Agent ID: `codex-060-coverage-ramp-20260428T024058Z`

Worktree: `/home/wogikaze/wgkz/ts2wasm-060-coverage-ramp-20260428T024058Z`

Branch: `agent/060-coverage-ramp-20260428T024058Z`

Issue: `060` (`issues/open/060-investigate-unknown-unsupported-cases.md`)

## Goal

Continue the reference-backed coverage ramp and classify any newly visible `unknown-unsupported` cases. Preferred next slice is test262 beyond the stored 1000 window, using limit 1250 unless local validation proves a different limit is needed.

## Scope

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
- docs unless issue 060 acceptance forces it

## Validation Plan

1. Confirm branch/worktree state before edits.
2. Run `TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 1250 --detail`.
3. If `unknown-unsupported` appears, inspect detail output and reference paths/diagnostics, then add classifier labels only when the evidence is concrete.
4. Rerun detail until `unknown-unsupported=0` or document why a case is genuinely unclassifiable.
5. Store JSON artifact with `TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 1250 --json > artifacts/coverage/results/test262.json`.
6. Run `scripts/manager update-coverage-matrix`.
7. If issue files change, run `scripts/manager update-issue-index`.
8. Run `scripts/manager check-issue-health` and `scripts/manager check-agent-state`.
9. Run `cargo fmt --all --check` if classifier scripts or Rust/source files change.

## Reporting Plan

Write cycle evidence under `reports/runs/<timestamp>-060-coverage-ramp/`, including commands, outcomes, artifact paths, issue changes, and commit hash. Attempt `scripts/manager discord-report --run-id <run_id>` after committing. If unavailable or failing, save `discord_payload.json` and `reporting_error.log` in the run directory and continue.

## Merge Protocol

Commit only validated progress from this worktree and branch. Do not merge to the parent branch. End the cycle with exactly one `PARENT_EVENT` line reporting `PROGRESS`, `DONE`, or `BLOCKED`, with the branch, commit hash, and merge request status.
