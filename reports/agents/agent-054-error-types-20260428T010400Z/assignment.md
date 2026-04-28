# Assignment: issue 054 Error types

Child id: agent-054-error-types-20260428T010400Z
Worktree: /home/wogikaze/wgkz/ts2wasm-054-error-types-20260428T010400Z
Branch: agent/054-error-types-20260428T010400Z

Assigned issue list:
- 054: Implement Error types

Issue order:
1. Reproduce the current behavior for `new Error("msg")`, `new TypeError("msg")`, `new ReferenceError("msg")`, and `new SyntaxError("msg")`.
2. Implement the smallest safe Error-object slice. Prefer constructor plus `.message` if that is feasible; if `.stack` requires a design decision or host support, record PROGRESS and leave 054 open.
3. Add Node/iwasm differential fixture coverage for the implemented slice.
4. Close 054 only if all acceptance criteria are met. Otherwise commit validated PROGRESS with exact remaining criteria.

Allowed files:
- issues/open/054-implement-error-types.md
- issues/done/054-implement-error-types.md
- issues/index.md
- crates/frontend/src/**
- crates/ir/src/**
- crates/backend-wasm/src/**
- crates/runtime-abi/src/**
- crates/shared/src/**
- crates/cli/tests/**
- fixtures/**
- reports/agents/agent-054-error-types-20260428T010400Z/**
- reports/runs/**054*error*/**

Forbidden files:
- scripts/**
- artifacts/coverage/**
- docs/**
- issues/open/051-implement-regexp.md
- issues/open/060-investigate-unknown-unsupported-cases.md

Expected validation commands:
- cargo fmt --all --check
- cargo nextest run -E 'test(error)'
- cargo nextest run -p ts2wasm-cli error
- cargo nextest run

Node differential requirement:
- Required for any runtime semantics. Add or reuse a fixture that compares Node stdout with iwasm stdout.

Webhook/reporting requirement:
- Send webhook only if configured and safe. If unavailable, write a deferred payload under this agent report directory and continue.

Merge request protocol:
- Commit validated work on the assigned branch.
- End with exactly one PARENT_EVENT line. Use merge_request=yes for DONE or useful PROGRESS that should be integrated.

Parent event protocol:
- PARENT_EVENT: DONE issue=054 branch=agent/054-error-types-20260428T010400Z commit=<hash> merge_request=yes
- PARENT_EVENT: PROGRESS issue=054 branch=agent/054-error-types-20260428T010400Z commit=<hash-or-none> merge_request=<yes-or-no>
- PARENT_EVENT: BLOCKED issue=054 branch=agent/054-error-types-20260428T010400Z commit=<hash-or-none> reason=<short-reason>
- PARENT_EVENT: FAILED issue=054 branch=agent/054-error-types-20260428T010400Z reason=<short-reason>

Coordination:
- You are not alone in the codebase. Other agents are working on separate branches; do not revert changes made by others, and keep your edits inside this assignment.
