# Cycle Report: 060 coverage ramp 9000

Run id: `060-coverage-ramp9000-20260428T062802Z`
Issue: `issues/open/060-investigate-unknown-unsupported-cases.md`
Branch: `agent/060-coverage-ramp9000-20260428T062802Z`
Status: PROGRESS

## Summary

Expanded the stored test262 reference coverage window from limit 8000 to limit 9000. The refreshed artifact has `executed=9000`, `build_pass=1`, `semantic_pass=0`, `unsupported=8999`, `blocked=0`, and no `unknown-unsupported` bucket.

No classifier script changes and no follow-up issues were required. The newly visible `object-builtin` bucket was already classified by existing rules.

## Files changed

- `artifacts/coverage/results/test262.json`
- `artifacts/coverage/reference-coverage-matrix.md`
- `current-state.md`
- `issues/open/060-investigate-unknown-unsupported-cases.md`
- `reports/runs/060-coverage-ramp9000-20260428T062802Z/`

## Coverage evidence

Detail command:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 9000 --detail
```

Result summary:

```text
suite=test262
denominator=53445
executed=9000
build_pass=1
semantic_pass=0
fail=0
unsupported=8998
blocked=1
unsupported_features=name-resolution:2546,builtin-api:2399,array-builtin:2166,function:542,eval:461,date:421,parser-syntax:188,object-builtin:102,string-builtin:63,regexp-literal:51,duplicate-local:31,legacy-global-builtin:16,declaration-emit:4,destructuring:2,object-literal:2,arguments-object:1,async-iteration:1,class:1,switch:1
unknown-unsupported=0
```

JSON artifact command:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 9000 --json > artifacts/coverage/results/test262.json
```

Stored artifact summary:

```text
executed=9000
build_pass=1
semantic_pass=0
fail=0
unsupported=8999
blocked=0
unknown-unsupported=0
```

The detail run reported the known transient blocked timeout for `annexB/built-ins/Array/from/iterator-method-emulates-undefined.js`; the JSON rerun completed with `blocked=0` and classified that case as `array-builtin`.

## Validation

```text
scripts/manager update-coverage-matrix
result: pass

scripts/manager update-coverage-matrix --check
result: pass; coverage matrix OK (up to date)

scripts/manager check-issue-health
result: pass; check_issue_health: OK

scripts/manager check-agent-state
result: pass; OK: agent state files validated
```

## Outcome

Issue 060 remains open as validated PROGRESS. Full DONE still requires exhausting broader reference windows beyond this assigned test262 limit-9000 slice and the existing tsc root caveat remains unchanged.
