# eval/function PR F report: optional host object facts

Run ID: `eval-function-pr-f-optional-host-object-facts`
Date: 2026-05-19

## Slice

Preserved host external object facts through optional property/index access.

## Change

- `resolved_expr_returns_host_external_object` now treats `OptionalPropertyAccess` like `PropertyAccess`.
- It also treats `OptionalComputedIndex` like `ComputedIndex`.
- Added `function-constructor-dynamic-optional-nested-method-node-shim.ts`.
- The fixture verifies a nested object read through `obj?.child` from a dynamic Function host result keeps enough host external facts for `child.cb(...)` to lower through `host.function.callMethod` with the correct receiver.
- Added exact manifest coverage for `host.function.compile`, `host.function.call`, and `host.function.callMethod`.

## Evidence

- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-host-optional-nested-target cargo test -p ts2wasm-cli --test node_shim_host dynamic_function_handle_preserves_optional_nested_object_method_through_node_shim_host_imports -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-host-optional-nested-target cargo test -p ts2wasm-cli --test m11_host_deny dynamic_function_optional_nested_method_declares_exact_host_capabilities -- --nocapture`
- PASS: `cargo fmt --all --check`

## Remaining

This advances fact propagation for focused host external object flows. Full PR F still needs the production-wide `HostExternalObject` / `HostExternalFunction` ABI and handle-table contract beyond local lowering facts and the Node test shim.
