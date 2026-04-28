# Assignment: issue 060 unsupported classification expansion

Child id: agent-060-unsupported-classification-20260428T005343Z
Worktree: /home/wogikaze/wgkz/ts2wasm-060-unsupported-classification-20260428T005343Z
Branch: agent/060-unsupported-classification-20260428T005343Z

Assigned issue list:
- 060: Investigate and classify unknown-unsupported diagnostic cases

Issue order:
1. Continue from the existing 2026-04-28 PROGRESS evidence in issue 060.
2. Run a broader but bounded classification window using the existing local reference checkout via `TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference`.
3. If new `unknown-unsupported` cases appear, classify them with stable path/message labels and update both `scripts/lib/feature-labels.sh` and the Python classifier if needed.
4. Create or update feature issues only for newly identified, reference-backed buckets. Do not generate duplicate noise.
5. Keep 060 open unless all required coverage windows are exhausted and unknown count is zero or explicitly justified.

Allowed files:
- scripts/lib/feature-labels.sh
- scripts/run/reference-coverage.py
- scripts/gen/issues-from-coverage.py
- scripts/data/**
- artifacts/coverage/**
- issues/open/060-investigate-unknown-unsupported-cases.md
- issues/open/**
- issues/index.md
- current-state.md
- reports/agents/agent-060-unsupported-classification-20260428T005343Z/**
- reports/runs/**060*unsupported*/**

Forbidden files:
- crates/**
- fixtures/**
- docs/**
- issues/open/051-implement-regexp.md
- issues/open/045-implement-class-syntax.md

Expected validation commands:
- cargo fmt --all --check
- TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 300
- TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage tsc --limit 150
- TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage tsgo --limit 100
- scripts/manager update-coverage-matrix
- scripts/manager update-issue-index --check
- scripts/manager check-issue-health

Webhook/reporting requirement:
- Send webhook only if configured and safe. If unavailable, write a deferred payload under this agent report directory and continue.

Merge request protocol:
- Commit validated classification/artifact/issue progress on the assigned branch.
- End with exactly one PARENT_EVENT line. Use merge_request=yes for DONE or useful PROGRESS that should be integrated.

Parent event protocol:
- PARENT_EVENT: DONE issue=060 branch=agent/060-unsupported-classification-20260428T005343Z commit=<hash> merge_request=yes
- PARENT_EVENT: PROGRESS issue=060 branch=agent/060-unsupported-classification-20260428T005343Z commit=<hash-or-none> merge_request=<yes-or-no>
- PARENT_EVENT: BLOCKED issue=060 branch=agent/060-unsupported-classification-20260428T005343Z commit=<hash-or-none> reason=<short-reason>
- PARENT_EVENT: FAILED issue=060 branch=agent/060-unsupported-classification-20260428T005343Z reason=<short-reason>

Coordination:
- You are not alone in the codebase. Other agents are working on separate branches; do not revert changes made by others, and keep your edits inside this assignment.
