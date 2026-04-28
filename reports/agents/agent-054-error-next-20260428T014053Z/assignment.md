# Assignment: issue 054 Error types next slice

Child id: agent-054-error-next-20260428T014053Z
Worktree: /home/wogikaze/wgkz/ts2wasm-054-error-next-20260428T014053Z
Branch: agent/054-error-next-20260428T014053Z

Assigned issue list:
- 054: Implement Error types

Issue order:
1. Continue from issue 054's Error constructor + `.message` progress.
2. Implement one narrow safe slice: prefer non-string message coercion or prototype/`instanceof Error` if existing object/class support makes it small. If `.stack` requires policy/design, record a blocker/progress note instead of stalling.
3. Close 054 only if constructors, message, stack, fixtures, and no regression criteria are complete; otherwise commit validated PROGRESS.

Allowed files:
- issues/open/054-implement-error-types.md
- issues/done/054-implement-error-types.md
- issues/index.md
- crates/ir/src/**
- crates/backend-wasm/src/**
- crates/cli/tests/**
- fixtures/builtins-and-io/**
- reports/agents/agent-054-error-next-20260428T014053Z/**
- reports/runs/**054*error*/**

Forbidden files:
- scripts/**
- artifacts/coverage/**
- docs/**
- issues/open/051-implement-regexp.md
- issues/open/226-implement-parameter-properties.md

Expected validation commands:
- cargo fmt --all --check
- cargo nextest run -E 'test(error)'
- cargo nextest run -p ts2wasm-cli error
- cargo nextest run
- scripts/manager check-issue-health

Node differential requirement:
- Required for every accepted Error behavior.

Webhook/reporting requirement:
- Send webhook only if configured and safe. If unavailable, write a deferred payload under this report directory.

Merge request protocol:
- Commit validated work and end with exactly one PARENT_EVENT line.

Parent event protocol:
- PARENT_EVENT: DONE issue=054 branch=agent/054-error-next-20260428T014053Z commit=<hash> merge_request=yes
- PARENT_EVENT: PROGRESS issue=054 branch=agent/054-error-next-20260428T014053Z commit=<hash-or-none> merge_request=<yes-or-no>
- PARENT_EVENT: BLOCKED issue=054 branch=agent/054-error-next-20260428T014053Z commit=<hash-or-none> reason=<short-reason>
- PARENT_EVENT: FAILED issue=054 branch=agent/054-error-next-20260428T014053Z reason=<short-reason>

Coordination:
- You are not alone in the codebase. Other agents are working on separate branches; do not revert changes made by others, and keep your edits inside this assignment.
