# eval/function PR E report: static Function typeof source

Run ID: `eval-function-pr-e-static-function-typeof-source`
Date: 2026-05-19

## Slice

Extended static `Function` / `new Function` source classification for focused `typeof` expressions used inside source-building expressions.

## Change

- Added compile-time `typeof` classification for static primitive, null, array, and BigInt values.
- Kept the fold inside the existing Function-constructor source evaluator so `typeof` composes with static string concatenation before Function-constructor `ToString`.
- Added `function-constructor-static-typeof-source.ts` to Node-shim stdout, fixture catalog, and standalone host-deny guards.

## Evidence

- RED: `cargo test -p ts2wasm-cli --test m11_host_deny standalone_fixtures_pass_host_deny -- --nocapture` rejected the new fixture with `host-deny mode rejects Node host imports` before the classifier change.
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-fn-typeof-source-target cargo test -p ts2wasm-cli --test node_shim_host static_function_constructor_typeof_source_uses_aot_lane -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-fn-typeof-source-target cargo test -p ts2wasm-cli --test m11_host_deny standalone_fixtures_pass_host_deny -- --nocapture`

## Remaining

This is still a focused PR E source-classifier slice. The first-class `FunctionConstructorPlan` migration and broader `typeof` object/function identity model remain open.
