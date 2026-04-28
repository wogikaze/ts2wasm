# Child assignment: issue 022 coverage evidence rows

Child label: agent-022-coverage-evidence-20260428T100000Z
Worktree: /home/wogikaze/wgkz/arukellt-022-coverage-evidence-20260428T100000Z
Branch: agent/022-coverage-evidence-20260428T100000Z

## Assigned issues

1. issues/open/022-expand-test262-differential-coverage.md

## Scope

Issue 022 now has two valid evidence modes:

- sorted ramp: `test262 --limit 100` proves Gate D executed-count tracking
- selected seeds: `test262 --paths-file scripts/data/test262-semantic-core-seeds.txt` proves Gate E build/semantic pass capability

Make the smallest safe scripts/docs/artifact change so both evidence modes can be represented without replacing one another. Prefer an additive coverage result/artifact convention or matrix row shape that keeps the existing sorted ramp row and records selected-seed evidence clearly.

Do not edit compiler implementation files.

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
- reports/agents/agent-022-coverage-evidence-20260428T100000Z/**
- reports/runs/**022*coverage*/**

## Forbidden files

- crates/frontend/src/**
- crates/ir/src/**
- crates/backend-wasm/src/**
- crates/runtime-abi/src/**
- crates/cli/src/**
- crates/cli/tests/**
- fixtures/**
- current-state.md unless coverage facts change and the change is required
- docs/language-reference/javascript-features.md
- issues/open/221-implement-gc-call-frame-roots-for-closure-escape.md
- reports/agents/agent-221-gc-call-frame-roots-20260428T095000Z/**

## Expected validation

- Ensure `reference/test262` exists; if missing, clone a shallow official checkout.
- `python scripts/manager.py reference-coverage test262 --limit 100 --detail`
- `python scripts/manager.py reference-coverage test262 --paths-file scripts/data/test262-semantic-core-seeds.txt --detail`
- Any new artifact/matrix generation command needed by your change
- `scripts/manager update-coverage-matrix --check`
- `scripts/manager update-issue-index --check`
- `scripts/manager check-issue-health`
- `scripts/manager check-repo-smoke`
- `scripts/manager check-agent-state`
- `cargo fmt --all --check`

If a clean artifact design is larger than one cycle, commit PROGRESS with a precise proposal and validated no-op or partial artifact change.

## Reporting

Save reports under `reports/agents/agent-022-coverage-evidence-20260428T100000Z/` and `reports/runs/`.
If webhook delivery is unavailable, save/defer payload locally and continue.

End with exactly one parent event line:

```text
PARENT_EVENT: DONE issue=022 branch=agent/022-coverage-evidence-20260428T100000Z commit=<hash> merge_request=yes
PARENT_EVENT: PROGRESS issue=022 branch=agent/022-coverage-evidence-20260428T100000Z commit=<hash-or-none> merge_request=no
PARENT_EVENT: BLOCKED issue=022 branch=agent/022-coverage-evidence-20260428T100000Z commit=<hash-or-none> reason=<short-reason>
PARENT_EVENT: FAILED issue=022 branch=agent/022-coverage-evidence-20260428T100000Z reason=<short-reason>
```
