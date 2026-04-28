# Assignment: issue 047 super keyword

Child id: agent-047-super-close-20260428T011445Z
Worktree: /home/wogikaze/wgkz/ts2wasm-047-super-close-20260428T011445Z
Branch: agent/047-super-close-20260428T011445Z

Assigned issue list:
- 047: Implement super keyword

Issue order:
1. Verify current parser/IR/backend behavior against issue 047 acceptance criteria. The repo already has `super(...)`, `super.method(...)`, and class-super fixtures, so first determine whether this is stale or only missing close evidence.
2. If all 047 criteria are satisfied, move the issue to done with validation evidence.
3. If one narrow residual gap remains, implement only that gap and validate it.
4. Do not expand into static/private fields or unrelated class semantics.

Allowed files:
- issues/open/047-implement-super-keyword.md
- issues/done/047-implement-super-keyword.md
- issues/index.md
- crates/frontend/src/**
- crates/ir/src/**
- crates/backend-wasm/src/**
- crates/cli/tests/**
- fixtures/classes-and-inheritance/**
- fixtures/core-semantics/*instanceof*
- reports/agents/agent-047-super-close-20260428T011445Z/**
- reports/runs/**047*super*/**

Forbidden files:
- scripts/**
- artifacts/coverage/**
- docs/**
- issues/open/049-implement-map-set.md
- issues/open/224-implement-annexb-html-comments.md

Expected validation commands:
- cargo fmt --all --check
- cargo nextest run -p ts2wasm-cli super
- cargo nextest run -p ts2wasm-cli class
- cargo nextest run
- scripts/manager update-issue-index --check
- scripts/manager check-issue-health

Node differential requirement:
- Required for semantic close. Use existing fixture differential tests or add a minimal fixture if missing.

Webhook/reporting requirement:
- Send webhook only if configured and safe. If unavailable, write a deferred payload under this agent report directory and continue.

Merge request protocol:
- Commit validated work on the assigned branch.
- End with exactly one PARENT_EVENT line. Use merge_request=yes for DONE or useful PROGRESS that should be integrated.

Parent event protocol:
- PARENT_EVENT: DONE issue=047 branch=agent/047-super-close-20260428T011445Z commit=<hash> merge_request=yes
- PARENT_EVENT: PROGRESS issue=047 branch=agent/047-super-close-20260428T011445Z commit=<hash-or-none> merge_request=<yes-or-no>
- PARENT_EVENT: BLOCKED issue=047 branch=agent/047-super-close-20260428T011445Z commit=<hash-or-none> reason=<short-reason>
- PARENT_EVENT: FAILED issue=047 branch=agent/047-super-close-20260428T011445Z reason=<short-reason>

Coordination:
- You are not alone in the codebase. Other agents are working on separate branches; do not revert changes made by others, and keep your edits inside this assignment.
