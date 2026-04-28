# Assignment: issue 049 Map/Set delete completion slice

Child id: agent-049-map-set-delete-20260428T012931Z
Worktree: /home/wogikaze/wgkz/ts2wasm-049-map-set-delete-20260428T012931Z
Branch: agent/049-map-set-delete-20260428T012931Z

Assigned issue list:
- 049: Implement Map and Set

Issue order:
1. Continue from the 2026-04-28 PROGRESS notes in issue 049.
2. Fix the narrow parser/frontend gap for keyword property names after `.`, specifically `map.delete(...)` and `set.delete(...)`, without broad keyword semantics.
3. Wire/validate the existing `MapDelete` and `SetDelete` runtime helpers with Node/iwasm differential fixture coverage.
4. If all issue 049 acceptance criteria are met, move issue 049 to done; otherwise commit validated PROGRESS with exact remaining criteria.

Allowed files:
- issues/open/049-implement-map-set.md
- issues/done/049-implement-map-set.md
- issues/index.md
- crates/frontend/src/**
- crates/ir/src/**
- crates/backend-wasm/src/**
- crates/cli/tests/**
- fixtures/builtins-and-io/**
- reports/agents/agent-049-map-set-delete-20260428T012931Z/**
- reports/runs/**049*map*set*/**

Forbidden files:
- scripts/**
- artifacts/coverage/**
- docs/**
- issues/open/052-implement-json.md
- issues/open/060-investigate-unknown-unsupported-cases.md

Expected validation commands:
- cargo fmt --all --check
- cargo nextest run -E 'test(map) or test(set)'
- cargo nextest run -p ts2wasm-cli map
- cargo nextest run -p ts2wasm-cli set
- cargo nextest run
- scripts/manager update-issue-index --check
- scripts/manager check-issue-health

Node differential requirement:
- Required. Include `delete` behavior in a Node/iwasm differential fixture.

Webhook/reporting requirement:
- Send webhook only if configured and safe. If unavailable, write a deferred payload under this agent report directory and continue.

Merge request protocol:
- Commit validated work on the assigned branch.
- End with exactly one PARENT_EVENT line. Use merge_request=yes for DONE or useful PROGRESS that should be integrated.

Parent event protocol:
- PARENT_EVENT: DONE issue=049 branch=agent/049-map-set-delete-20260428T012931Z commit=<hash> merge_request=yes
- PARENT_EVENT: PROGRESS issue=049 branch=agent/049-map-set-delete-20260428T012931Z commit=<hash-or-none> merge_request=<yes-or-no>
- PARENT_EVENT: BLOCKED issue=049 branch=agent/049-map-set-delete-20260428T012931Z commit=<hash-or-none> reason=<short-reason>
- PARENT_EVENT: FAILED issue=049 branch=agent/049-map-set-delete-20260428T012931Z reason=<short-reason>

Coordination:
- You are not alone in the codebase. Other agents are working on separate branches; do not revert changes made by others, and keep your edits inside this assignment.
