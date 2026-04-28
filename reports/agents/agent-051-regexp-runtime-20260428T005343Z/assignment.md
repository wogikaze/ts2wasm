# Assignment: issue 051 RegExp runtime

Child id: agent-051-regexp-runtime-20260428T005343Z
Worktree: /home/wogikaze/wgkz/ts2wasm-051-regexp-runtime-20260428T005343Z
Branch: agent/051-regexp-runtime-20260428T005343Z

Assigned issue list:
- 051: Implement RegExp

Issue order:
1. Verify current RegExp literal status from issue 202/done and existing fixtures.
2. Implement the smallest safe runtime slice for issue 051. Prefer `RegExp.prototype.test` with literal-backed simple patterns if that is the narrowest feasible path. If constructor/exec/match require a broader object model change, commit validated progress and record the blocker instead of stalling.
3. Close 051 only if every acceptance criterion is met; otherwise record PROGRESS with exact remaining criteria.

Allowed files:
- issues/open/051-implement-regexp.md
- issues/done/051-implement-regexp.md
- issues/index.md
- crates/frontend/src/**
- crates/ir/src/**
- crates/backend-wasm/src/**
- crates/runtime-abi/src/**
- crates/shared/src/**
- crates/cli/tests/**
- fixtures/**
- reports/agents/agent-051-regexp-runtime-20260428T005343Z/**
- reports/runs/**051*regexp*/**

Forbidden files:
- scripts/**
- artifacts/coverage/**
- docs/**
- issues/open/060-investigate-unknown-unsupported-cases.md
- issues/open/045-implement-class-syntax.md

Expected validation commands:
- cargo fmt --all --check
- cargo nextest run -E 'test(regexp)'
- cargo nextest run -p ts2wasm-cli regexp
- cargo nextest run

Node differential requirement:
- Required for any semantic behavior added. Add or reuse a fixture that compares Node stdout with iwasm stdout.

Webhook/reporting requirement:
- Send webhook only if configured and safe. If unavailable, write a deferred payload under this agent report directory and continue.

Merge request protocol:
- Commit validated work on the assigned branch.
- End with exactly one PARENT_EVENT line. Use merge_request=yes for DONE or useful PROGRESS that should be integrated.

Parent event protocol:
- PARENT_EVENT: DONE issue=051 branch=agent/051-regexp-runtime-20260428T005343Z commit=<hash> merge_request=yes
- PARENT_EVENT: PROGRESS issue=051 branch=agent/051-regexp-runtime-20260428T005343Z commit=<hash-or-none> merge_request=<yes-or-no>
- PARENT_EVENT: BLOCKED issue=051 branch=agent/051-regexp-runtime-20260428T005343Z commit=<hash-or-none> reason=<short-reason>
- PARENT_EVENT: FAILED issue=051 branch=agent/051-regexp-runtime-20260428T005343Z reason=<short-reason>

Coordination:
- You are not alone in the codebase. Other agents are working on separate branches; do not revert changes made by others, and keep your edits inside this assignment.
