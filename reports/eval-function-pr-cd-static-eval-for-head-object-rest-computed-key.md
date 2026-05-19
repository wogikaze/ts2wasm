# eval/function PR C/D: static eval for-head object rest computed keys

## Summary

- Extended static direct eval `for (var ... of ...)` object-rest destructuring to computed excluded keys such as `{ [key]: removed, ...rest }`, reading the computed key from the caller scope.
- Extended the same path for static indirect eval, resolving computed keys through `globalThis.key` while leaving caller locals untouched and writing loop-head `var` results to the global object.
- Fixed the runtime root cause exposed by the indirect fixture: `$object_create` now allocates append headroom like object literals, so `globalThis` properties created before later eval/object-rest allocations are not overwritten by subsequent heap allocation.
- Added Node-shim stdout and host-deny standalone coverage for both fixtures.

## Verification

- PASS: `cargo test -p ts2wasm-cli --test node_shim_host static_direct_eval_for_head_var_object_rest_computed_key -- --nocapture`.
- PASS: `cargo test -p ts2wasm-cli --test node_shim_host static_indirect_eval_for_head_var_object_rest_computed_lands_on_global_object -- --nocapture`.
- PASS: `cargo test -p ts2wasm-cli --test node_shim_host static_direct_eval_for_head -- --nocapture`.
- PASS: `cargo test -p ts2wasm-cli --test node_shim_host static_indirect_eval_for_head -- --nocapture`.
- PASS: `cargo test -p ts2wasm-cli --test m11_host_deny static_direct_eval_for_head_var_destructuring_declares_no_node_host_eval_capability -- --nocapture`.
- PASS: `cargo test -p ts2wasm-cli --test m11_host_deny static_indirect_eval_for_head_var_destructuring_global_declares_no_node_host_eval_capability -- --nocapture`.
- PASS: `cargo check -p ts2wasm-ir -p ts2wasm-compiler -p ts2wasm-cli`.
- PASS: `python scripts/issue-lint.py`.
- FAIL (unrelated dirty files): `cargo fmt --all --check` reports formatting diffs in existing dirty runtime math/typed-array/catalog files outside this eval slice.
- BASELINE FAIL: `python scripts/manager.py check fixtures` still reports existing uncataloged builtins/classes fixtures unrelated to this eval slice.
