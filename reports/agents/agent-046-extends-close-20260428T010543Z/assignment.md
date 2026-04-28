# Assignment: issue 046 extends inheritance

Child id: agent-046-extends-close-20260428T010543Z
Worktree: /home/wogikaze/wgkz/ts2wasm-046-extends-close-20260428T010543Z
Branch: agent/046-extends-close-20260428T010543Z

Assigned issue list:
- 046: Implement extends inheritance

Issue order:
1. Verify current parser/IR/backend behavior against issue 046 acceptance criteria. The codebase already has `extends`, class parent maps, prototype references, and class inheritance fixtures, so first determine whether the issue is stale or only missing coverage/close evidence.
2. If all 046 acceptance criteria are satisfied, move the issue to done with concrete validation evidence.
3. If one narrow residual gap remains, implement only that gap and validate it.
4. Do not implement `super` beyond what is already required to prove inheritance; issue 047 owns `super`.

Allowed files:
- issues/open/046-implement-extends-inheritance.md
- issues/done/046-implement-extends-inheritance.md
- issues/index.md
- crates/frontend/src/**
- crates/ir/src/**
- crates/backend-wasm/src/**
- crates/cli/tests/**
- fixtures/classes-and-inheritance/**
- fixtures/core-semantics/*prototype*
- fixtures/core-semantics/*instanceof*
- reports/agents/agent-046-extends-close-20260428T010543Z/**
- reports/runs/**046*extends*/**

Forbidden files:
- scripts/**
- artifacts/coverage/**
- docs/**
- issues/open/047-implement-super-keyword.md
- issues/open/054-implement-error-types.md
- issues/open/060-investigate-unknown-unsupported-cases.md

Expected validation commands:
- cargo fmt --all --check
- cargo nextest run -p ts2wasm-cli class_extends
- cargo nextest run -p ts2wasm-cli prototype
- cargo nextest run -p ts2wasm-cli class
- cargo nextest run

Node differential requirement:
- Required for semantic close. Use existing fixture differential tests or add a minimal fixture if missing.

Webhook/reporting requirement:
- Send webhook only if configured and safe. If unavailable, write a deferred payload under this agent report directory and continue.

Merge request protocol:
- Commit validated work on the assigned branch.
- End with exactly one PARENT_EVENT line. Use merge_request=yes for DONE or useful PROGRESS that should be integrated.

Parent event protocol:
- PARENT_EVENT: DONE issue=046 branch=agent/046-extends-close-20260428T010543Z commit=<hash> merge_request=yes
- PARENT_EVENT: PROGRESS issue=046 branch=agent/046-extends-close-20260428T010543Z commit=<hash-or-none> merge_request=<yes-or-no>
- PARENT_EVENT: BLOCKED issue=046 branch=agent/046-extends-close-20260428T010543Z commit=<hash-or-none> reason=<short-reason>
- PARENT_EVENT: FAILED issue=046 branch=agent/046-extends-close-20260428T010543Z reason=<short-reason>

Coordination:
- You are not alone in the codebase. Other agents are working on separate branches; do not revert changes made by others, and keep your edits inside this assignment.
