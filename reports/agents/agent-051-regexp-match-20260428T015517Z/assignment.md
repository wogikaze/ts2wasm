# Assignment: issue 051 RegExp match/exec continuation

Parent branch: `master`
Worktree: `/home/wogikaze/wgkz/ts2wasm-051-regexp-match-20260428T015517Z`
Branch: `agent/051-regexp-match-20260428T015517Z`
Issue: `issues/open/051-implement-regexp.md`
Base: `466a4bd`

## Scope

Implement one narrow RegExp continuation slice after literal `.test` and `new RegExp("plain").test` support.

Preferred options, in order:

- `String.prototype.match` for supported plain-pattern RegExp values, returning enough observable output for a Node differential fixture.
- `RegExp.prototype.exec` for supported plain-pattern RegExp values, returning enough observable output for a Node differential fixture.

Keep unsupported patterns precise. Do not implement metacharacter semantics as substring semantics.

Expected paths:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/m2_node_diff.rs`
- `fixtures/core-semantics/`
- `issues/open/051-implement-regexp.md`
- `reports/agents/agent-051-regexp-match-20260428T015517Z/`
- `reports/runs/<timestamp>-051-regexp-match/`

Avoid docs unless a factual current-state change is required.

## Required validation

Run:

- `cargo fmt --all --check`
- `cargo nextest run -E 'test(regexp)'`
- `cargo nextest run -p ts2wasm-cli regexp`
- direct `node <fixture>` and `cargo run -p ts2wasm-cli -- build <fixture> -o /tmp/<name>.wasm && iwasm /tmp/<name>.wasm`
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`

Run full `cargo nextest run` if you touch shared runtime code in a way that could affect non-RegExp behavior. If skipped, state why in the report.

## Completion contract

Commit all validated work on this branch. If blocked, commit a report/evidence artifact instead of leaving the branch dirty.

Final response must include exactly one parent event line:

`PARENT_EVENT: PROGRESS issue=051 branch=agent/051-regexp-match-20260428T015517Z commit=<sha> merge_request=<yes|no>`

Use `merge_request=yes` only when parent can safely merge the branch.
