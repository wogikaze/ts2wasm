# eval/function PR E report: static Function comparison source

Run ID: `eval-function-pr-e-static-function-comparison-source`
Date: 2026-05-19

## Slice

Extended static `Function` / `new Function` source classification for focused compile-time comparison expressions.

## Change

- Added static folding for relational comparisons (`<`, `<=`, `>`, `>=`) over string-string lexicographic cases and finite number-convertible primitive operands.
- Added static folding for strict and loose equality/inequality across focused primitive/nullish cases.
- Preserved the existing dynamic/unsupported boundary for identity-sensitive objects such as arrays.
- Added `function-constructor-static-comparison-source.ts` to Node-shim stdout, fixture catalog, and standalone host-deny guards.

## Evidence

- RED: `cargo test -p ts2wasm-cli --test m11_host_deny standalone_fixtures_pass_host_deny -- --nocapture` rejected the new fixture with `host-deny mode rejects Node host imports` before the classifier change.
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-fn-comparison-source-target cargo test -p ts2wasm-cli --test node_shim_host static_function_constructor_comparison_source_uses_aot_lane -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-fn-comparison-source-target cargo test -p ts2wasm-cli --test m11_host_deny standalone_fixtures_pass_host_deny -- --nocapture`

## Remaining

The first-class `FunctionConstructorPlan` migration remains open, and broader comparison coverage should be expanded only when the static evaluator can model object ToPrimitive, BigInt/Number equality, `NaN`, and UTF-16 string ordering precisely.
