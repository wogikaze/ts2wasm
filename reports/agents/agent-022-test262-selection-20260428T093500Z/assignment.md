# Child assignment: issue 022 test262 selection mode

Child label: agent-022-test262-selection-20260428T093500Z
Worktree: /home/wogikaze/wgkz/arukellt-022-test262-selection-20260428T093500Z
Branch: agent/022-test262-selection-20260428T093500Z

## Assigned issues

1. issues/open/022-expand-test262-differential-coverage.md

## Scope

Implement a concrete harness selection mode that helps issue 022 make Gate E progress without waiting for the sorted first-100 test262 ramp to pass unsupported clusters.

Smallest acceptable slice:

- Add an option to `scripts/run/reference-coverage.py` that can run a curated, deterministic test262 subset file list or path filter.
- Include a source-backed seed list or documented filter for currently runnable semantic-core test262 candidates.
- Run the new mode against test262 and record build/semantic pass evidence.
- Update issue 022 and coverage artifacts/matrix only if measured facts change.

If implementation proves unsafe, commit a validated PROGRESS note with the exact blocker and keep issue 022 open.

## Allowed files

- scripts/run/reference-coverage.py
- scripts/manager.py
- scripts/manager
- scripts/run/**
- scripts/data/**
- artifacts/coverage/**
- docs/15-coverage-matrix.md
- docs/06-testing-and-coverage.md
- issues/open/022-expand-test262-differential-coverage.md
- issues/done/022-expand-test262-differential-coverage.md
- issues/index.md
- reports/agents/agent-022-test262-selection-20260428T093500Z/**
- reports/runs/**022*test262*/**

## Forbidden files

- crates/frontend/src/**
- crates/ir/src/**
- crates/backend-wasm/src/**
- crates/cli/src/**
- crates/cli/tests/**
- fixtures/**
- current-state.md unless coverage facts change and the change is required
- docs/language-reference/javascript-features.md
- issues/open/210-implement-arrow-function-closure-lexical-this.md
- reports/agents/agent-210-arrow-closure-20260428T092000Z/**

## Expected validation

- Ensure `reference/test262` exists in this worktree; if missing, clone a shallow official checkout.
- `python scripts/manager.py reference-coverage test262 --limit 0`
- New selection/filter mode command with `--detail`
- `scripts/manager update-coverage-matrix --check` if artifacts are not changed, or `scripts/manager update-coverage-matrix` then `--check` if artifacts are changed.
- `scripts/manager update-issue-index --check`
- `scripts/manager check-issue-health`
- `scripts/manager check-repo-smoke`
- `scripts/manager check-agent-state`
- `cargo fmt --all --check`

Do not run or alter issue 210 work. Do not edit compiler implementation files.

## Reporting

Save reports under `reports/agents/agent-022-test262-selection-20260428T093500Z/` and `reports/runs/`.
If webhook delivery is unavailable, save/defer payload locally and continue.

End with exactly one parent event line:

```text
PARENT_EVENT: DONE issue=022 branch=agent/022-test262-selection-20260428T093500Z commit=<hash> merge_request=yes
PARENT_EVENT: PROGRESS issue=022 branch=agent/022-test262-selection-20260428T093500Z commit=<hash-or-none> merge_request=no
PARENT_EVENT: BLOCKED issue=022 branch=agent/022-test262-selection-20260428T093500Z commit=<hash-or-none> reason=<short-reason>
PARENT_EVENT: FAILED issue=022 branch=agent/022-test262-selection-20260428T093500Z reason=<short-reason>
```
