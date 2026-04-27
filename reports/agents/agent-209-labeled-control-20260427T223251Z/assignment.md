# Assignment: agent-209-labeled-control-20260427T223251Z

Child id: agent-209-labeled-control-20260427T223251Z
Worktree: `/home/wogikaze/wgkz/arukellt-209-labeled-control-20260427T223251Z`
Branch: `agent/209-labeled-control-20260427T223251Z`

Assigned issues:

- 209: Implement labeled break and continue

Issue order:

1. 209

Allowed files:

- `crates/frontend/src/**`
- `crates/ir/src/**`
- `crates/backend-wasm/src/**`
- `crates/cli/tests/**`
- `fixtures/control-flow-and-exceptions/**`
- `docs/language-reference/javascript-features.md`
- `current-state.md`
- `issues/open/209-implement-labeled-break-continue.md`
- `issues/done/209-implement-labeled-break-continue.md`
- `issues/index.md`
- `reports/agents/agent-209-labeled-control-20260427T223251Z/**`
- `reports/runs/agent-209-labeled-control-20260427T223251Z/**`

Forbidden files:

- `crates/runtime-abi/**`
- GC issue files and GC runtime allocation work
- typed IR dump issue files

Expected validation commands:

- `cargo nextest run -E 'test(break|continue|label)'`
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

- `PARENT_EVENT: DONE issue=209 branch=agent/209-labeled-control-20260427T223251Z commit=<hash> merge_request=yes`
- `PARENT_EVENT: PROGRESS issue=209 branch=agent/209-labeled-control-20260427T223251Z commit=<hash-or-none> merge_request=no`
- `PARENT_EVENT: BLOCKED issue=209 branch=agent/209-labeled-control-20260427T223251Z commit=<hash-or-none> reason=<short-reason>`
- `PARENT_EVENT: FAILED issue=209 branch=agent/209-labeled-control-20260427T223251Z reason=<short-reason>`
