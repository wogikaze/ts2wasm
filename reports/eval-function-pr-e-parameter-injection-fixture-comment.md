# eval/function PR E report: parameter injection fixture comment

Run ID: `eval-function-pr-e-parameter-injection-fixture-comment`
Date: 2026-05-19

## Slice

Clarified the remaining static Function constructor parameter-injection boundary fixture.

## Change

- Added an expected-diagnostic comment to `function-constructor-parameter-injection-unsupported.ts`.
- The fixture now states that this is a FormalParameters parse-goal rejection before AOT expansion, not a generic unsupported Function constructor case.
- Added an issue note tying the fixture comment to the existing PR E parse-goal validation slice.

## Evidence

- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-param-injection-comment-target cargo test -p ts2wasm-cli --test node_shim_host static_function_constructor_rejects_parameter_wrapper_injection -- --nocapture`
- PASS: `python scripts/issue-lint.py`
- PASS: scoped `git diff --check`

## Remaining

This is a PR E tracking/fixture clarity slice. It does not replace the broader first-class `FunctionConstructorPlan` migration.
