# Assignment: issue 051 RegExp next slice

Child id: agent-051-regexp-next-20260428T014053Z
Worktree: /home/wogikaze/wgkz/ts2wasm-051-regexp-next-20260428T014053Z
Branch: agent/051-regexp-next-20260428T014053Z

Assigned issue list:
- 051: Implement RegExp

Issue order:
1. Continue from issue 051's literal-backed `RegExp.prototype.test` progress.
2. Implement one narrow safe remaining slice: prefer `String.prototype.match` for literal-backed plain byte RegExp or `new RegExp("plain").test(...)` if that is simpler and validated.
3. Do not broaden into full RegExp syntax or captures. Unsupported cases must keep issue-linked diagnostics instead of wrong semantics.
4. Close 051 only if constructor, test, exec, match, literal parsing, and fixture criteria are all complete; otherwise commit validated PROGRESS.

Allowed files:
- issues/open/051-implement-regexp.md
- issues/done/051-implement-regexp.md
- issues/index.md
- crates/frontend/src/**
- crates/ir/src/**
- crates/backend-wasm/src/**
- crates/cli/tests/**
- fixtures/core-semantics/**
- fixtures/builtins-and-io/**
- reports/agents/agent-051-regexp-next-20260428T014053Z/**
- reports/runs/**051*regexp*/**

Forbidden files:
- scripts/**
- artifacts/coverage/**
- docs/**
- issues/open/054-implement-error-types.md
- issues/open/226-implement-parameter-properties.md

Expected validation commands:
- cargo fmt --all --check
- cargo nextest run -E 'test(regexp)'
- cargo nextest run -p ts2wasm-cli regexp
- cargo nextest run
- scripts/manager check-issue-health

Node differential requirement:
- Required for every accepted RegExp behavior.

Webhook/reporting requirement:
- Send webhook only if configured and safe. If unavailable, write a deferred payload under this report directory.

Merge request protocol:
- Commit validated work and end with exactly one PARENT_EVENT line.

Parent event protocol:
- PARENT_EVENT: DONE issue=051 branch=agent/051-regexp-next-20260428T014053Z commit=<hash> merge_request=yes
- PARENT_EVENT: PROGRESS issue=051 branch=agent/051-regexp-next-20260428T014053Z commit=<hash-or-none> merge_request=<yes-or-no>
- PARENT_EVENT: BLOCKED issue=051 branch=agent/051-regexp-next-20260428T014053Z commit=<hash-or-none> reason=<short-reason>
- PARENT_EVENT: FAILED issue=051 branch=agent/051-regexp-next-20260428T014053Z reason=<short-reason>

Coordination:
- You are not alone in the codebase. Other agents are working on separate branches; do not revert changes made by others, and keep your edits inside this assignment.
