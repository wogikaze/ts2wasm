# Assignment: issue 226 TypeScript parameter properties

Child id: agent-226-parameter-properties-20260428T014053Z
Worktree: /home/wogikaze/wgkz/ts2wasm-226-parameter-properties-20260428T014053Z
Branch: agent/226-parameter-properties-20260428T014053Z

Assigned issue list:
- 226: Implement TypeScript parameter properties

Issue order:
1. Reproduce the two reference-backed tsgo parameter-property cases or a minimal local fixture.
2. Implement the smallest safe supported subset for constructor parameter properties with default values.
3. If full implementation is too broad, add precise issue-linked diagnostics and validated classification progress; do not silently leave `parameter-property` as unknown.
4. Close 226 only if the classified tsgo cases no longer report `parameter-property` and required full gates pass.

Allowed files:
- issues/open/226-implement-parameter-properties.md
- issues/done/226-implement-parameter-properties.md
- issues/index.md
- crates/frontend/src/**
- crates/ir/src/**
- crates/backend-wasm/src/**
- crates/cli/tests/**
- fixtures/**
- reports/agents/agent-226-parameter-properties-20260428T014053Z/**
- reports/runs/**226*parameter*/**

Forbidden files:
- scripts/**
- artifacts/coverage/**
- docs/**
- issues/open/051-implement-regexp.md
- issues/open/054-implement-error-types.md

Expected validation commands:
- cargo fmt --all --check
- cargo nextest run -E 'test(parameter) or test(class)'
- TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage tsgo --limit 120
- scripts/manager update-issue-index --check
- scripts/manager check-issue-health
- cargo nextest run

Node/reference differential requirement:
- Use Node/iwasm differential for executable behavior when possible; use reference-coverage evidence for tsgo classification acceptance.

Webhook/reporting requirement:
- Send webhook only if configured and safe. If unavailable, write a deferred payload under this report directory.

Merge request protocol:
- Commit validated work and end with exactly one PARENT_EVENT line.

Parent event protocol:
- PARENT_EVENT: DONE issue=226 branch=agent/226-parameter-properties-20260428T014053Z commit=<hash> merge_request=yes
- PARENT_EVENT: PROGRESS issue=226 branch=agent/226-parameter-properties-20260428T014053Z commit=<hash-or-none> merge_request=<yes-or-no>
- PARENT_EVENT: BLOCKED issue=226 branch=agent/226-parameter-properties-20260428T014053Z commit=<hash-or-none> reason=<short-reason>
- PARENT_EVENT: FAILED issue=226 branch=agent/226-parameter-properties-20260428T014053Z reason=<short-reason>

Coordination:
- You are not alone in the codebase. Other agents are working on separate branches; do not revert changes made by others, and keep your edits inside this assignment.
