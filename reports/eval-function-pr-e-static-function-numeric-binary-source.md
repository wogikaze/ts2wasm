# eval/function PR E report: static Function numeric binary source

Run ID: `eval-function-pr-e-static-function-numeric-binary-source`
Date: 2026-05-19

## Slice

Extended static `Function` / `new Function` source classification for finite numeric binary expressions.

## Change

- Added compile-time folding for `-`, `*`, `/`, `%`, and `**` when both operands are statically convertible to finite JavaScript numbers.
- Reused the Function-constructor `ToString` source path so numeric results become generated AOT function bodies instead of dynamic `host.function.compile` inputs.
- Covered focused `ToNumber` operands for strings, booleans, and `null`, while keeping non-finite and unsupported values on the existing dynamic/unsupported boundary.
- Added `function-constructor-static-numeric-binary-source.ts` to Node-shim stdout, fixture catalog, and standalone host-deny guards.

## Evidence

- RED: `cargo test -p ts2wasm-cli --test m11_host_deny standalone_fixtures_pass_host_deny -- --nocapture` rejected the new fixture with `host-deny mode rejects Node host imports` before the classifier change.
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-fn-numeric-binary-target cargo test -p ts2wasm-cli --test node_shim_host static_function_constructor_numeric_binary_source_uses_aot_lane -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-fn-numeric-binary-target cargo test -p ts2wasm-cli --test m11_host_deny standalone_fixtures_pass_host_deny -- --nocapture`

## Remaining

The static Function constructor still needs the broader first-class `FunctionConstructorPlan` migration and full ECMAScript numeric edge audit for `NaN`, infinities, BigInt numeric operations, and source text representation.
