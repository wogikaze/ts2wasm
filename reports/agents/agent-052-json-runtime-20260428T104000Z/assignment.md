# Child assignment: issue 052 JSON runtime builtins

Child label: agent-052-json-runtime-20260428T104000Z
Worktree: /home/wogikaze/wgkz/arukellt-052-json-runtime-20260428T104000Z
Branch: agent/052-json-runtime-20260428T104000Z

## Assigned issues

1. issues/open/052-implement-json.md

## Scope

Implement or complete the smallest safe JSON.parse / JSON.stringify runtime slice. Existing JSON fixtures may already cover part of this behavior; verify first, then either close the issue with evidence or commit validated progress with a precise remaining JSON gap.

## Allowed files

- issues/open/052-implement-json.md
- issues/done/052-implement-json.md
- issues/index.md
- crates/backend-wasm/src/**
- crates/runtime-abi/src/**
- crates/shared/src/**
- crates/cli/tests/**
- fixtures/**/json*
- fixtures/**/*json*
- reports/agents/agent-052-json-runtime-20260428T104000Z/**
- reports/runs/**052*json*/**

## Forbidden files

- scripts/**
- artifacts/coverage/**
- docs/15-coverage-matrix.md
- issues/open/060-investigate-unknown-unsupported-cases.md
- reports/agents/agent-060-unsupported-classification-20260428T102000Z/**

## Expected validation

- `cargo fmt --all --check`
- focused JSON/builtin tests
- Node differential fixture tests for semantic claims
- `scripts/manager update-issue-index --check`
- `scripts/manager check-issue-health`
- `scripts/manager check-repo-smoke`

Run full `cargo nextest run` before DONE. For PROGRESS, run narrow validation plus fmt and issue checks.

## Reporting

Save reports under `reports/agents/agent-052-json-runtime-20260428T104000Z/` and `reports/runs/`.
If webhook delivery is unavailable, save/defer payload locally and continue.

End with exactly one parent event line:

```text
PARENT_EVENT: DONE issue=052 branch=agent/052-json-runtime-20260428T104000Z commit=<hash> merge_request=yes
PARENT_EVENT: PROGRESS issue=052 branch=agent/052-json-runtime-20260428T104000Z commit=<hash-or-none> merge_request=no
PARENT_EVENT: BLOCKED issue=052 branch=agent/052-json-runtime-20260428T104000Z commit=<hash-or-none> reason=<short-reason>
PARENT_EVENT: FAILED issue=052 branch=agent/052-json-runtime-20260428T104000Z reason=<short-reason>
```
