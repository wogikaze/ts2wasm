# Assignment: issue 060 classification ramp

Child id: agent-060-classification-ramp-20260428T012931Z
Worktree: /home/wogikaze/wgkz/ts2wasm-060-classification-ramp-20260428T012931Z
Branch: agent/060-classification-ramp-20260428T012931Z

Assigned issue list:
- 060: Investigate and classify unknown-unsupported diagnostic cases

Issue order:
1. Continue from issue 060 validated windows: test262 limit 300, tsc limit 150, tsgo limit 100.
2. Run the next bounded reference window using `TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference` where available, and use the existing `/tmp/ts2wasm-issue060-reference` only for tsc if the parent reference root still lacks TypeScript.
3. Classify any new `unknown-unsupported` buckets with stable labels in both shell and Python classifiers.
4. Create/update feature issues only for new, reference-backed buckets; avoid duplicates.
5. Keep 060 open unless the broader acceptance is actually exhausted.

Allowed files:
- scripts/lib/feature-labels.sh
- scripts/run/reference-coverage.py
- scripts/data/**
- artifacts/coverage/**
- issues/open/060-investigate-unknown-unsupported-cases.md
- issues/open/**
- issues/index.md
- current-state.md
- reports/agents/agent-060-classification-ramp-20260428T012931Z/**
- reports/runs/**060*classification*/**

Forbidden files:
- crates/**
- fixtures/**
- docs/**
- issues/open/049-implement-map-set.md
- issues/open/052-implement-json.md

Expected validation commands:
- cargo fmt --all --check
- TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 500
- TS2WASM_REFERENCE_ROOT=/tmp/ts2wasm-issue060-reference scripts/manager reference-coverage tsc --limit 200
- TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage tsgo --limit 120
- scripts/manager update-coverage-matrix --check
- scripts/manager update-issue-index --check
- scripts/manager check-issue-health

Webhook/reporting requirement:
- Send webhook only if configured and safe. If unavailable, write a deferred payload under this agent report directory and continue.

Merge request protocol:
- Commit validated classification/artifact/issue progress on the assigned branch.
- End with exactly one PARENT_EVENT line. Use merge_request=yes for useful PROGRESS.

Parent event protocol:
- PARENT_EVENT: PROGRESS issue=060 branch=agent/060-classification-ramp-20260428T012931Z commit=<hash-or-none> merge_request=<yes-or-no>
- PARENT_EVENT: BLOCKED issue=060 branch=agent/060-classification-ramp-20260428T012931Z commit=<hash-or-none> reason=<short-reason>
- PARENT_EVENT: FAILED issue=060 branch=agent/060-classification-ramp-20260428T012931Z reason=<short-reason>

Coordination:
- You are not alone in the codebase. Other agents are working on separate branches; do not revert changes made by others, and keep your edits inside this assignment.
