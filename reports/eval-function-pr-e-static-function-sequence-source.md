# eval/function PR E report: static Function sequence source

Run ID: `eval-function-pr-e-static-function-sequence-source`
Date: 2026-05-19

## Slice

Extended static `Function` / `new Function` source classification for side-effect-free sequence expressions.

## Change

- Added `ResolvedExpr::Sequence` handling to the Function-constructor static source evaluator.
- Static comma expressions now classify using their final static operand before Function-constructor `ToString`.
- Added `function-constructor-static-sequence-source.ts` to Node-shim stdout, fixture catalog, and standalone host-deny guards.

## Evidence

- RED: `cargo test -p ts2wasm-cli --test m11_host_deny standalone_fixtures_pass_host_deny -- --nocapture` rejected the new fixture with `host-deny mode rejects Node host imports` before the classifier change.
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-fn-sequence-source-target cargo test -p ts2wasm-cli --test node_shim_host static_function_constructor_sequence_source_uses_aot_lane -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-fn-sequence-source-target cargo test -p ts2wasm-cli --test m11_host_deny standalone_fixtures_pass_host_deny -- --nocapture`

## Remaining

The side-effect boundary is intentionally narrow: only already-resolved static sequence operands are folded. The broader `FunctionConstructorPlan` migration and general constant-expression evaluator remain open.
