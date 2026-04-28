# Child assignment: issue 022 test262 coverage expansion

Child label: agent-022-test262-coverage-20260428T092000Z
Worktree: /home/wogikaze/wgkz/arukellt-022-test262-coverage-20260428T092000Z
Branch: agent/022-test262-coverage-20260428T092000Z

## Assigned issues

1. issues/open/022-expand-test262-differential-coverage.md

## Scope

Make validated coverage-harness progress for test262 without editing compiler implementation files. Prefer increasing or documenting the next safe limit ramp, updating coverage artifacts/matrix, and generating reference-backed follow-up issues if the issue cannot close in one cycle.

Do not act as a search engine. This is a concrete coverage execution, artifact, matrix, and issue-management task.

## Allowed files

- issues/open/022-expand-test262-differential-coverage.md
- issues/done/022-expand-test262-differential-coverage.md
- issues/open/**
- issues/done/**
- issues/index.md
- artifacts/coverage/**
- docs/15-coverage-matrix.md
- docs/06-testing-and-coverage.md
- scripts/manager.py
- scripts/manager
- scripts/lib/**
- reports/agents/agent-022-test262-coverage-20260428T092000Z/**
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

- Ensure `reference/test262` exists in this worktree; if missing, clone a shallow checkout from the official test262 repository.
- `cargo fmt --all --check` if scripts/Rust-adjacent files are touched
- `python scripts/manager.py reference-coverage test262 --limit 100 --detail`
- `scripts/manager update-coverage-matrix`
- `scripts/manager update-coverage-matrix --check`
- `scripts/manager update-issue-index --check`
- `scripts/manager check-issue-health`

If Gate D/E cannot close, commit validated PROGRESS with the new artifact/matrix state and a clear next ramp recommendation.

## Reporting

Save reports under `reports/agents/agent-022-test262-coverage-20260428T092000Z/` and `reports/runs/`.
If webhook delivery is unavailable, save/defer payload locally and continue.

End with exactly one parent event line:

```text
PARENT_EVENT: DONE issue=022 branch=agent/022-test262-coverage-20260428T092000Z commit=<hash> merge_request=yes
PARENT_EVENT: PROGRESS issue=022 branch=agent/022-test262-coverage-20260428T092000Z commit=<hash-or-none> merge_request=no
PARENT_EVENT: BLOCKED issue=022 branch=agent/022-test262-coverage-20260428T092000Z commit=<hash-or-none> reason=<short-reason>
PARENT_EVENT: FAILED issue=022 branch=agent/022-test262-coverage-20260428T092000Z reason=<short-reason>
```
