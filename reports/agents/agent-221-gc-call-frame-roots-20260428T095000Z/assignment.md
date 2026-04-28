# Child assignment: issue 221 GC call-frame roots

Child label: agent-221-gc-call-frame-roots-20260428T095000Z
Worktree: /home/wogikaze/wgkz/arukellt-221-gc-call-frame-roots-20260428T095000Z
Branch: agent/221-gc-call-frame-roots-20260428T095000Z

## Assigned issues

1. issues/open/221-implement-gc-call-frame-roots-for-closure-escape.md

## Scope

Implement the smallest safe GC call-frame/closure-root slice now that issue 210 has landed local arrow closure support. Close issue 221 only if activation-frame registration/unregistration and closure/call-frame escape fixtures are covered. Otherwise commit validated PROGRESS with a precise remaining root-safety gap.

## Allowed files

- issues/open/221-implement-gc-call-frame-roots-for-closure-escape.md
- issues/done/221-implement-gc-call-frame-roots-for-closure-escape.md
- issues/index.md
- crates/backend-wasm/src/**
- crates/runtime-abi/src/**
- crates/cli/tests/**
- fixtures/**/gc*
- fixtures/**/closure*
- fixtures/**/arrow*
- current-state.md
- docs/04-compiler-architecture-and-runtime.md
- docs/14-runtime-abi.md
- docs/language-reference/javascript-features.md
- reports/agents/agent-221-gc-call-frame-roots-20260428T095000Z/**
- reports/runs/**221*gc*/**

## Forbidden files

- scripts/**
- artifacts/coverage/**
- docs/15-coverage-matrix.md
- issues/open/022-expand-test262-differential-coverage.md
- reports/agents/agent-022-test262-selection-20260428T093500Z/**

## Expected validation

- `cargo fmt --all --check`
- targeted GC/root/closure tests, for example `cargo nextest run -E 'test(gc|root|closure|arrow)'` if it selects tests, or exact test names otherwise
- Node differential fixture tests for any added closure/call-frame GC fixture
- `scripts/manager update-issue-index --check`
- `scripts/manager check-issue-health`
- `scripts/manager check-repo-smoke`

Run full `cargo nextest run` before DONE. For PROGRESS, run narrow validation plus fmt and issue checks.

## Reporting

Save reports under `reports/agents/agent-221-gc-call-frame-roots-20260428T095000Z/` and `reports/runs/`.
If webhook delivery is unavailable, save/defer payload locally and continue.

End with exactly one parent event line:

```text
PARENT_EVENT: DONE issue=221 branch=agent/221-gc-call-frame-roots-20260428T095000Z commit=<hash> merge_request=yes
PARENT_EVENT: PROGRESS issue=221 branch=agent/221-gc-call-frame-roots-20260428T095000Z commit=<hash-or-none> merge_request=no
PARENT_EVENT: BLOCKED issue=221 branch=agent/221-gc-call-frame-roots-20260428T095000Z commit=<hash-or-none> reason=<short-reason>
PARENT_EVENT: FAILED issue=221 branch=agent/221-gc-call-frame-roots-20260428T095000Z reason=<short-reason>
```
