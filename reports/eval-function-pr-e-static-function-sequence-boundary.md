# eval/function PR E report: static Function sequence boundary

Run ID: `eval-function-pr-e-static-function-sequence-boundary`
Date: 2026-05-19

## Slice

Tightened the static `Function` source classifier boundary for sequence expressions.

## Change

- `ResolvedExpr::Sequence` source folding now requires every discarded prefix operand to be statically classifiable before folding to the final operand.
- Effectful prefixes such as assignments no longer disappear during AOT source classification.
- Added an IR unit test covering both static-prefix AOT classification and effectful-prefix host-compile fallback.

## Evidence

- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-parent-fn-sequence-target cargo test -p ts2wasm-ir function_constructor_sequence_sources_require_static_prefixes -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-parent-fn-sequence-target cargo test -p ts2wasm-cli --test node_shim_host static_function_constructor_sequence_source_uses_aot_lane -- --nocapture`

## Remaining

This keeps the focused sequence classifier honest, but a general JavaScript constant-expression evaluator with explicit side-effect modeling remains out of scope until the first-class `FunctionConstructorPlan` migration.
