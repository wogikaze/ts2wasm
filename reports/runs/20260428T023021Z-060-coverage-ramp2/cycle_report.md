# Issue 060 Coverage Ramp2 Report

Date: 2026-04-28
Branch: `agent/060-coverage-ramp2-20260428T022516Z`
Worktree: `/home/wogikaze/wgkz/ts2wasm-060-coverage-ramp2-20260428T022516Z`

## Outcome

PROGRESS. The stored test262 reference window was raised from limit 750 to limit 1000.

The limit-1000 stored artifact has:

- `executed=1000`
- `unsupported=1000`
- `blocked=0`
- `fail=0`
- `unknown-unsupported=0`

No classifier labels or follow-up issues were needed in this slice. Newly visible failures were already covered by existing labels: `parser-syntax`, `function`, `name-resolution`, `arguments-object`, `switch`, and existing builtin labels.

## Commands

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 1000 --detail
result: pass; unknown-unsupported=0; first detail run observed one timeout-blocked Array.from case

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --path-filter annexB/built-ins/Array/from/iterator-method-emulates-undefined.js --detail
result: pass; unsupported_features=array-builtin:1; blocked=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 1000 --json > artifacts/coverage/results/test262.json
result: pass; stored JSON artifact refreshed

scripts/manager update-coverage-matrix
result: pass; coverage matrix refreshed
```

## Validation

```text
scripts/manager update-issue-index
result: pass; issues/index.md regenerated

scripts/manager update-issue-index --check
result: pass; issues/index.md OK (up to date)

scripts/manager update-coverage-matrix --check
result: pass; coverage matrix OK (up to date)

scripts/manager check-issue-health
result: pass; check_issue_health OK and issues/index.md queue OK

scripts/manager check-agent-state
result: pass; agent state files validated

python -m py_compile scripts/run/reference-coverage.py scripts/gen/issues-from-coverage.py
result: pass
```

## Remaining Work

Issue 060 remains open. Full acceptance still requires exhausting broader reference windows, and the assigned `/home/wogikaze/wgkz/ts2wasm/reference` root still lacks the `TypeScript` checkout needed for exact-root tsc validation.
