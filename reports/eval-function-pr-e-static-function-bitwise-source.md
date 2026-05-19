# eval/function PR E report: static Function bitwise source

Run ID: `eval-function-pr-e-static-function-bitwise-source`
Date: 2026-05-19

## Slice

Extended static `Function` / `new Function` source classification for focused bitwise, shift, and unary `~` expressions.

## Change

- Added static folding for `&`, `|`, `^`, `<<`, `>>`, `>>>`, and unary `~` over primitive operands that can be converted through the focused compile-time numeric path.
- Added `ToInt32`, `ToUint32`, and shift-count helpers for Function-constructor source classification so generated bodies match JavaScript bitwise result strings for the guarded subset.
- Added `function-constructor-static-bitwise-source.ts` to Node-shim stdout, fixture catalog, and standalone host-deny guards.

## Evidence

- RED: `cargo test -p ts2wasm-cli --test m11_host_deny standalone_fixtures_pass_host_deny -- --nocapture` rejected the new fixture before the classifier change with `binary operator LeftShift not yet supported`.
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-fn-bitwise-source-target cargo test -p ts2wasm-cli --test node_shim_host static_function_constructor_bitwise_source_uses_aot_lane -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-fn-bitwise-source-target cargo test -p ts2wasm-cli --test m11_host_deny standalone_fixtures_pass_host_deny -- --nocapture`

## Remaining

This is still a focused static source classifier slice. The first-class `FunctionConstructorPlan` migration and full JavaScript numeric edge audit remain open.
