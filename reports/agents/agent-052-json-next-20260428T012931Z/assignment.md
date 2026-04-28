# Assignment: issue 052 JSON next runtime slice

Child id: agent-052-json-next-20260428T012931Z
Worktree: /home/wogikaze/wgkz/ts2wasm-052-json-next-20260428T012931Z
Branch: agent/052-json-next-20260428T012931Z

Assigned issue list:
- 052: Implement JSON

Issue order:
1. Continue from the existing first JSON runtime slice documented in issue 052.
2. Implement one narrow, safe JSON gap such as escaped ASCII strings, arrays in `JSON.parse`, or nested object/array handling. Pick the smallest slice that can be proven with Node/iwasm differential coverage.
3. Do not attempt full replacer/space or throw-compatible diagnostic parity in this slice unless it is already trivial.
4. Close 052 only if all acceptance criteria are satisfied; otherwise commit validated PROGRESS with exact remaining criteria.

Allowed files:
- issues/open/052-implement-json.md
- issues/done/052-implement-json.md
- issues/index.md
- crates/ir/src/**
- crates/backend-wasm/src/**
- crates/cli/tests/**
- fixtures/builtins-and-io/**
- reports/agents/agent-052-json-next-20260428T012931Z/**
- reports/runs/**052*json*/**

Forbidden files:
- scripts/**
- artifacts/coverage/**
- docs/**
- issues/open/049-implement-map-set.md
- issues/open/060-investigate-unknown-unsupported-cases.md

Expected validation commands:
- cargo fmt --all --check
- cargo nextest run -p ts2wasm-cli json
- cargo nextest run -E 'test(json)'
- cargo nextest run
- scripts/manager check-issue-health

Node differential requirement:
- Required for any JSON semantics. Add or update a fixture that compares Node stdout with iwasm stdout.

Webhook/reporting requirement:
- Send webhook only if configured and safe. If unavailable, write a deferred payload under this agent report directory and continue.

Merge request protocol:
- Commit validated work on the assigned branch.
- End with exactly one PARENT_EVENT line. Use merge_request=yes for DONE or useful PROGRESS that should be integrated.

Parent event protocol:
- PARENT_EVENT: DONE issue=052 branch=agent/052-json-next-20260428T012931Z commit=<hash> merge_request=yes
- PARENT_EVENT: PROGRESS issue=052 branch=agent/052-json-next-20260428T012931Z commit=<hash-or-none> merge_request=<yes-or-no>
- PARENT_EVENT: BLOCKED issue=052 branch=agent/052-json-next-20260428T012931Z commit=<hash-or-none> reason=<short-reason>
- PARENT_EVENT: FAILED issue=052 branch=agent/052-json-next-20260428T012931Z reason=<short-reason>

Coordination:
- You are not alone in the codebase. Other agents are working on separate branches; do not revert changes made by others, and keep your edits inside this assignment.
