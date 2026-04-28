# Child assignment: issue 050 Date runtime builtins

Child label: agent-050-date-runtime-20260428T102000Z
Worktree: /home/wogikaze/wgkz/arukellt-050-date-runtime-20260428T102000Z
Branch: agent/050-date-runtime-20260428T102000Z

## Assigned issues

1. issues/open/050-implement-date.md

## Scope

Implement the smallest safe Date runtime slice. Close issue 050 only if the scoped constructor and basic methods in the issue are implemented with fixture coverage. Otherwise commit validated PROGRESS with a clear remaining Date API gap.

Prefer deterministic, auditable behavior over untracked host time access. If `Date.now()` or `new Date()` needs a capability/time policy decision, record the blocker or create a follow-up instead of inventing unaudited host imports.

## Allowed files

- issues/open/050-implement-date.md
- issues/done/050-implement-date.md
- issues/index.md
- crates/backend-wasm/src/**
- crates/runtime-abi/src/**
- crates/shared/src/**
- crates/cli/tests/**
- fixtures/**/date*
- fixtures/**/*date*
- reports/agents/agent-050-date-runtime-20260428T102000Z/**
- reports/runs/**050*date*/**

## Forbidden files

- scripts/**
- artifacts/coverage/**
- docs/15-coverage-matrix.md
- issues/open/060-investigate-unknown-unsupported-cases.md
- reports/agents/agent-060-unsupported-classification-20260428T102000Z/**

## Expected validation

- `cargo fmt --all --check`
- focused Date/builtin tests
- Node differential fixture tests for semantic claims
- `scripts/manager update-issue-index --check`
- `scripts/manager check-issue-health`
- `scripts/manager check-repo-smoke`

Run full `cargo nextest run` before DONE. For PROGRESS, run narrow validation plus fmt and issue checks.

## Reporting

Save reports under `reports/agents/agent-050-date-runtime-20260428T102000Z/` and `reports/runs/`.
If webhook delivery is unavailable, save/defer payload locally and continue.

End with exactly one parent event line:

```text
PARENT_EVENT: DONE issue=050 branch=agent/050-date-runtime-20260428T102000Z commit=<hash> merge_request=yes
PARENT_EVENT: PROGRESS issue=050 branch=agent/050-date-runtime-20260428T102000Z commit=<hash-or-none> merge_request=no
PARENT_EVENT: BLOCKED issue=050 branch=agent/050-date-runtime-20260428T102000Z commit=<hash-or-none> reason=<short-reason>
PARENT_EVENT: FAILED issue=050 branch=agent/050-date-runtime-20260428T102000Z reason=<short-reason>
```
