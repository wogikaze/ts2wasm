# Assignment: issue 224 Annex B HTML-like comments

Child id: agent-224-html-comments-20260428T011445Z
Worktree: /home/wogikaze/wgkz/ts2wasm-224-html-comments-20260428T011445Z
Branch: agent/224-html-comments-20260428T011445Z

Assigned issue list:
- 224: Implement Annex B HTML-like comments

Issue order:
1. Reproduce at least one listed test262 `html-comment` case or a minimal local fixture.
2. Implement the smallest lexer/parser support for Annex B HTML-like comments while preserving normal operator parsing for `<`, `!`, and `-`.
3. Add regression fixtures/tests for accepted `<!--`, accepted/allowed `-->`, and non-comment operator cases.
4. Close 224 only if the listed classified window no longer reports `html-comment`; otherwise commit validated PROGRESS with exact remaining cases.

Allowed files:
- issues/open/224-implement-annexb-html-comments.md
- issues/done/224-implement-annexb-html-comments.md
- issues/index.md
- crates/frontend/src/**
- crates/cli/tests/**
- fixtures/**
- artifacts/coverage/**
- reports/agents/agent-224-html-comments-20260428T011445Z/**
- reports/runs/**224*html*/**

Forbidden files:
- crates/backend-wasm/**
- crates/runtime-abi/**
- docs/**
- issues/open/047-implement-super-keyword.md
- issues/open/049-implement-map-set.md

Expected validation commands:
- cargo fmt --all --check
- cargo nextest run -p ts2wasm-frontend html
- cargo nextest run -p ts2wasm-cli html
- TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 300
- scripts/manager update-coverage-matrix --check
- scripts/manager update-issue-index --check
- scripts/manager check-issue-health

Node differential requirement:
- Required for accepted executable fixture semantics if runtime execution is affected.

Webhook/reporting requirement:
- Send webhook only if configured and safe. If unavailable, write a deferred payload under this agent report directory and continue.

Merge request protocol:
- Commit validated work on the assigned branch.
- End with exactly one PARENT_EVENT line. Use merge_request=yes for DONE or useful PROGRESS that should be integrated.

Parent event protocol:
- PARENT_EVENT: DONE issue=224 branch=agent/224-html-comments-20260428T011445Z commit=<hash> merge_request=yes
- PARENT_EVENT: PROGRESS issue=224 branch=agent/224-html-comments-20260428T011445Z commit=<hash-or-none> merge_request=<yes-or-no>
- PARENT_EVENT: BLOCKED issue=224 branch=agent/224-html-comments-20260428T011445Z commit=<hash-or-none> reason=<short-reason>
- PARENT_EVENT: FAILED issue=224 branch=agent/224-html-comments-20260428T011445Z reason=<short-reason>

Coordination:
- You are not alone in the codebase. Other agents are working on separate branches; do not revert changes made by others, and keep your edits inside this assignment.
