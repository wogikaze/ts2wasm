# eval/function PR F report: optional computed host object facts

Run ID: `eval-function-pr-f-optional-computed-host-object-facts`
Date: 2026-05-19

## Slice

Added focused coverage for optional computed host external object fact propagation.

## Change

- Added `function-constructor-dynamic-optional-computed-nested-method-node-shim.ts`.
- Registered the fixture in `fixtures/catalog.yaml`.
- The fixture verifies `obj?.[key]` from a dynamic Function host result preserves the nested host external object fact so `child.cb(...)` uses the receiver-preserving host function method bridge.
- Extended host-deny coverage for both optional nested host object fixtures.

## Evidence

- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-host-optional-computed-nested-target cargo test -p ts2wasm-cli --test node_shim_host dynamic_function_optional_computed_nested_method_uses_host_call_method -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-host-optional-computed-nested-target cargo test -p ts2wasm-cli --test m11_host_deny host_deny_rejects_dynamic_function_constructor_host_lane -- --nocapture`
- PASS: `cargo fmt --all --check`

## Remaining

This locks the optional computed propagation slice. Full PR F still needs a production-wide host external ABI and handle-table contract beyond localized lowering facts.
