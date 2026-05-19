# eval / Function PR C-D: static eval var destructuring computed/rest guards

Run ID: `eval-function-pr-cd-static-eval-var-destructuring-computed-rest`
Date: 2026-05-19

## Summary

- Added direct eval fixtures for static `var { [key]: removed, ...rest } = ...` destructuring declarations, both top-level and in traditional `for` init.
- Added indirect eval fixtures for the same patterns, verifying caller lexical bindings stay untouched while eval-created `var` bindings land on `globalThis`.
- Added host-deny coverage so these static AOT eval paths remain standalone and do not request `host.eval.*`.

## Verification

- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-eval-next-target cargo test -p ts2wasm-cli --test node_shim_host static_direct_eval_var_destructuring_computed_rest_lands_in_caller_scope -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-eval-next-target cargo test -p ts2wasm-cli --test node_shim_host static_direct_eval_for_init_var_destructuring_computed_rest_lands_in_caller_scope -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-eval-next-target cargo test -p ts2wasm-cli --test node_shim_host static_indirect_eval_var_destructuring_computed_rest_lands_on_global_object -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-eval-next-target cargo test -p ts2wasm-cli --test node_shim_host static_indirect_eval_for_init_var_destructuring_computed_rest_lands_on_global_object -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-eval-next-target cargo test -p ts2wasm-cli --test m11_host_deny static_direct_eval_for_head_var_declares_no_node_host_eval_capability -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-eval-next-target cargo test -p ts2wasm-cli --test m11_host_deny static_direct_eval_for_head_var_destructuring_declares_no_node_host_eval_capability -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-eval-next-target cargo test -p ts2wasm-cli --test m11_host_deny static_indirect_eval_var_global_declares_no_node_host_eval_capability -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-eval-next-target cargo test -p ts2wasm-cli --test m11_host_deny static_indirect_eval_for_head_var_destructuring_global_declares_no_node_host_eval_capability -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-eval-next-target cargo check -p ts2wasm-ir -p ts2wasm-compiler -p ts2wasm-cli`
- PASS: `python scripts/issue-lint.py`
- FAIL (existing baseline): `python scripts/manager.py check fixtures`
  - Existing uncataloged fixtures remain in `builtins-and-io/` and `classes-and-inheritance/`; the new `core-semantics` fixtures are cataloged.
- FAIL (existing baseline): `CARGO_TARGET_DIR=/tmp/ts2wasm-eval-next-target cargo fmt --all --check`
  - Existing rustfmt diffs remain in `crates/backend-wasm/src/runtime/host/emit.rs` and `crates/backend-wasm/src/runtime_typed_arrays.rs`.
