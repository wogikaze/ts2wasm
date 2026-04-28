# Assignment: issue 060 reference coverage ramp

Parent branch: `master`
Worktree: `/home/wogikaze/wgkz/ts2wasm-060-coverage-ramp-20260428T015517Z`
Branch: `agent/060-coverage-ramp-20260428T015517Z`
Issue: `issues/open/060-investigate-unknown-unsupported-cases.md`
Base: `466a4bd`

## Scope

Run a reference-backed coverage/classification continuation and generate follow-up issues when new actionable classes appear.

Primary goal:

- Increase at least one reference-coverage window beyond the current recorded limits.
- Classify newly surfaced `unknown-unsupported` diagnostics into specific issue-linked classes where evidence supports it.
- Generate follow-up issues from coverage evidence when the queue needs new implementation-ready/design-ready work.

Expected paths:

- `artifacts/coverage/`
- `issues/open/`
- `issues/index.md`
- `issues/open/060-investigate-unknown-unsupported-cases.md`
- `reports/agents/agent-060-coverage-ramp-20260428T015517Z/`
- `reports/runs/<timestamp>-060-coverage-ramp/`

Do not edit compiler code in this worktree unless a tiny classification bug blocks the coverage run.

## Suggested commands

Use repository scripts and keep outputs as artifacts/reports:

- `scripts/manager reference-coverage test262 --limit <higher> --detail`
- `scripts/manager reference-coverage tsc --limit <higher> --detail`
- `scripts/manager reference-coverage tsgo --limit <higher> --detail`
- pipe suitable detail output to `scripts/manager gen-issues-from-coverage -- --suite <suite>` or the mise equivalent
- `scripts/manager update-issue-index`
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`

If a ramp is too expensive or blocked by missing references, commit the reproduced blocker and choose another suite/window.

## Completion contract

Commit generated artifacts/issues/reports on this branch. If no safe issues can be generated, commit a clean stop report with exact evidence.

Final response must include exactly one parent event line:

`PARENT_EVENT: PROGRESS issue=060 branch=agent/060-coverage-ramp-20260428T015517Z commit=<sha> merge_request=<yes|no>`

Use `merge_request=yes` when issue/index/artifact updates are internally consistent and gates pass.
