# eval/function PR E report: static Function source drift

Run ID: `eval-function-pr-e-static-function-source-drift`
Date: 2026-05-19

## Slice

Closed a plan drift gap in static `Function` / `new Function` AOT source metadata.

## Change

- `FunctionConstructorPlan::static_source_is_consistent()` now recomputes the expected static source from `args`.
- `FunctionConstructorPlan::expected_host_policy()` now derives from the same recomputed source instead of trusting stored `static_source`.
- The plan is rejected if `static_source` does not match the recomputed source.
- The same guard rejects plans that hide static args behind `static_source: None` and a host-compile policy.
- Added IR unit coverage for body drift and missing static-source metadata.

## Evidence

- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-function-source-drift-target cargo test -p ts2wasm-ir function_constructor_plan_validates_static_source_metadata -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-function-source-drift-target cargo test -p ts2wasm-ir function_constructor_plan -- --nocapture`

## Remaining

This closes a static source/args consistency gap, but the broader PR E migration away from synthetic `FunctionExpr` expansion is still open.
