# Assignment: issue 051 RegExp exec continuation

Parent branch: `master`
Base: `334fb90`
Worktree: `/home/wogikaze/wgkz/ts2wasm-051-regexp-exec-20260428T020949Z`
Branch: `agent/051-regexp-exec-20260428T020949Z`
Issue: `issues/open/051-implement-regexp.md`

## Scope

Implement one narrow `RegExp.prototype.exec` continuation slice for the existing plain byte pattern subset.

Primary goal:

- Support direct literal or `new RegExp("plain")` `exec(...)` enough for a Node/iwasm fixture to observe hit and miss behavior.
- Preserve precise diagnostics for unsupported patterns and flags.
- Do not claim full match-array semantics if the implementation only exposes the currently observable subset.

Expected paths:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `issues/open/051-implement-regexp.md`
- `reports/agents/agent-051-regexp-exec-20260428T020949Z/`
- `reports/runs/<timestamp>-051-regexp-exec/`

## Required validation

- `cargo fmt --all --check`
- `cargo nextest run -E 'test(regexp)'`
- `cargo nextest run -p ts2wasm-cli regexp`
- direct `node <fixture>` and `cargo run -p ts2wasm-cli -- build <fixture> -o /tmp/<name>.wasm && iwasm /tmp/<name>.wasm`
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`

Run full `cargo nextest run` if the runtime helper shares behavior beyond RegExp. If skipped, state why.

## Completion contract

Commit all validated work. If blocked, commit evidence/report instead of leaving the branch dirty.

Final response must include exactly one parent event line:

`PARENT_EVENT: PROGRESS issue=051 branch=agent/051-regexp-exec-20260428T020949Z commit=<sha> merge_request=<yes|no>`
