# eval/function PR E report: dynamic Function sequence prefix

Run ID: `eval-function-pr-e-dynamic-function-sequence-prefix`
Date: 2026-05-19

## Slice

Added host-lane coverage for effectful sequence expressions used as Function-constructor source arguments.

## Change

- Added `function-constructor-dynamic-sequence-prefix-node-shim.ts`.
- The fixture verifies `Function((side = 1, "return 7"))` preserves the prefix assignment and executes the generated function through the dynamic host handle path.
- Added fixture catalog and host-deny rejection coverage so this boundary cannot regress into the static AOT lane.

## Evidence

- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-fn-sequence-host-target cargo test -p ts2wasm-cli --test node_shim_host dynamic_function_sequence_prefix_preserves_side_effect_through_node_shim_host_imports -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-fn-sequence-host-target cargo test -p ts2wasm-cli --test m11_host_deny host_deny_rejects_dynamic_function_constructor_host_lane -- --nocapture`

## Remaining

Exact manifest assertions for this specific fixture can be added alongside the broader host function manifest audit, but the current guard already fixes the static/dynamic lane boundary and host-deny behavior.
