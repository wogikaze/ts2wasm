# eval/function PR E report: dynamic Function sequence manifest

Run ID: `eval-function-pr-e-dynamic-function-sequence-manifest`
Date: 2026-05-19

## Slice

Added exact manifest coverage for effectful sequence expressions used as dynamic Function-constructor source arguments.

## Change

- Added a manifest assertion for `function-constructor-dynamic-sequence-prefix-node-shim.ts`.
- The test requires `standalone: false`, `node_host.required: true`, and exact imports `host.function.compile` + `host.function.call`.
- Capability reasons for both imports must be present and auditable.

## Evidence

- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-fn-sequence-manifest-target cargo test -p ts2wasm-cli --test m11_host_deny dynamic_function_sequence_prefix_declares_exact_host_capabilities -- --nocapture`

## Remaining

This closes the manifest side of the effectful sequence boundary. Broader PR E still needs first-class generated function object construction instead of synthetic `FunctionExpr` expansion.
