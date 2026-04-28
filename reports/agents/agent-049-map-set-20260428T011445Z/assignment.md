# Assignment: issue 049 Map and Set

Child id: agent-049-map-set-20260428T011445Z
Worktree: /home/wogikaze/wgkz/ts2wasm-049-map-set-20260428T011445Z
Branch: agent/049-map-set-20260428T011445Z

Assigned issue list:
- 049: Implement Map and Set

Issue order:
1. Reproduce current behavior for `new Map()`, `map.set/get/has/delete`, `new Set()`, and `set.add/has/delete`.
2. Implement the smallest safe collection slice. Prefer a narrow Map-only or Set-only subset if full issue closure is too broad.
3. Add Node/iwasm differential fixture coverage for implemented behavior.
4. Close 049 only if all Map and Set acceptance criteria are met; otherwise commit validated PROGRESS with exact remaining criteria.

Allowed files:
- issues/open/049-implement-map-set.md
- issues/done/049-implement-map-set.md
- issues/index.md
- crates/ir/src/**
- crates/backend-wasm/src/**
- crates/runtime-abi/src/**
- crates/shared/src/**
- crates/cli/tests/**
- fixtures/**
- reports/agents/agent-049-map-set-20260428T011445Z/**
- reports/runs/**049*map*set*/**

Forbidden files:
- scripts/**
- artifacts/coverage/**
- docs/**
- issues/open/047-implement-super-keyword.md
- issues/open/224-implement-annexb-html-comments.md

Expected validation commands:
- cargo fmt --all --check
- cargo nextest run -E 'test(map) or test(set)'
- cargo nextest run -p ts2wasm-cli map
- cargo nextest run -p ts2wasm-cli set
- cargo nextest run
- scripts/manager check-issue-health

Node differential requirement:
- Required for runtime semantics. Add or reuse a fixture that compares Node stdout with iwasm stdout.

Webhook/reporting requirement:
- Send webhook only if configured and safe. If unavailable, write a deferred payload under this agent report directory and continue.

Merge request protocol:
- Commit validated work on the assigned branch.
- End with exactly one PARENT_EVENT line. Use merge_request=yes for DONE or useful PROGRESS that should be integrated.

Parent event protocol:
- PARENT_EVENT: DONE issue=049 branch=agent/049-map-set-20260428T011445Z commit=<hash> merge_request=yes
- PARENT_EVENT: PROGRESS issue=049 branch=agent/049-map-set-20260428T011445Z commit=<hash-or-none> merge_request=<yes-or-no>
- PARENT_EVENT: BLOCKED issue=049 branch=agent/049-map-set-20260428T011445Z commit=<hash-or-none> reason=<short-reason>
- PARENT_EVENT: FAILED issue=049 branch=agent/049-map-set-20260428T011445Z reason=<short-reason>

Coordination:
- You are not alone in the codebase. Other agents are working on separate branches; do not revert changes made by others, and keep your edits inside this assignment.
