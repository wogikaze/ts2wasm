# Assignment: agent-217-gc-header-20260427T223251Z

Child id: agent-217-gc-header-20260427T223251Z
Worktree: `/home/wogikaze/wgkz/arukellt-217-gc-header-20260427T223251Z`
Branch: `agent/217-gc-header-20260427T223251Z`

Assigned issues:

- 217: Implement GC heap header and trigger accounting

Issue order:

1. 217

Allowed files:

- `crates/runtime-abi/src/**`
- `crates/backend-wasm/src/runtime_core.rs`
- `crates/backend-wasm/src/runtime_fn.rs`
- `crates/backend-wasm/src/runtime_builder.rs`
- `crates/backend-wasm/src/**/tests*`
- `docs/14-runtime-abi.md`
- `issues/open/217-implement-gc-heap-header-and-trigger-accounting.md`
- `issues/done/217-implement-gc-heap-header-and-trigger-accounting.md`
- `issues/open/017b-implement-gc-strategy.md`
- `issues/index.md`
- `reports/agents/agent-217-gc-header-20260427T223251Z/**`
- `reports/runs/agent-217-gc-header-20260427T223251Z/**`

Forbidden files:

- frontend parser changes
- CLI dump command work
- labeled control-flow fixtures except if full-suite fallout proves a direct dependency

Expected validation commands:

- `cargo nextest run -p ts2wasm-runtime-abi`
- `cargo nextest run -p ts2wasm-backend-wasm`
- `cargo fmt --all --check`
- `scripts/manager update-issue-index --check`
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`
- `scripts/manager check-repo-smoke`
- `cargo nextest run`

Webhook/reporting requirement:

- Attempt webhook if configured.
- If webhook is unavailable, save deferred payload/error under the assigned reports directory and continue.

Merge request protocol:

- Commit validated work on this branch.
- Do not merge to parent.
- Request parent merge with a `PARENT_EVENT` line.

Parent event protocol:

- `PARENT_EVENT: DONE issue=217 branch=agent/217-gc-header-20260427T223251Z commit=<hash> merge_request=yes`
- `PARENT_EVENT: PROGRESS issue=217 branch=agent/217-gc-header-20260427T223251Z commit=<hash-or-none> merge_request=no`
- `PARENT_EVENT: BLOCKED issue=217 branch=agent/217-gc-header-20260427T223251Z commit=<hash-or-none> reason=<short-reason>`
- `PARENT_EVENT: FAILED issue=217 branch=agent/217-gc-header-20260427T223251Z reason=<short-reason>`
