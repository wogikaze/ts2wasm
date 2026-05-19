# eval/function PR E report: FunctionConstructorPlan host-policy validation

Run ID: `eval-function-pr-e-function-host-policy-validation`
Date: 2026-05-19

## Slice

Moved Function-constructor host-lane consistency checking into `FunctionConstructorPlan`.

## Change

- Added `FunctionConstructorPlan::expected_host_policy()` and `host_policy_is_consistent()`.
- AOT expansion now rejects plans whose `host_policy` does not match their `static_source` classification.
- Lowering now rejects inconsistent Function-constructor plans and also rejects static AOT Function constructors that reach the dynamic host-compile lane without expansion.
- Added an IR unit test covering static AOT, dynamic host compile, and an intentionally inconsistent plan.

## Evidence

- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-parent-fn-policy-target cargo test -p ts2wasm-ir function_constructor_plan_derives_expected_host_policy -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-parent-fn-policy-target cargo check -p ts2wasm-ir -p ts2wasm-compiler -p ts2wasm-cli`

## Remaining

This is a validation slice on the way to PR E. The broader first-class `FunctionConstructorPlan` migration still needs generated function object creation to stop relying on synthetic `FunctionExpr` expansion as the main path.
