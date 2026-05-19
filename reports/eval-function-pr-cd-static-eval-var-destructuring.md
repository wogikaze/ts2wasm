# eval / Function PR C-D: static eval var destructuring landing

Run ID: `eval-function-pr-cd-static-eval-var-destructuring`
Date: 2026-05-19

## Summary

- Extended static eval `var` declaration collection so object and array destructuring patterns contribute their bound names instead of treating the whole pattern text as one declaration.
- Added a plan-owned `DestructureVarLet` completion step carrying caller/global landing metadata for static eval `var` destructuring declarations.
- Lowered destructuring `var` declarations through a single-evaluation temp plus the existing pattern write path, preserving caller-scope landing for direct eval and `globalThis` landing for indirect eval.
- Added Node differential and host-deny fixtures for top-level `var` destructuring and traditional `for` init `var` destructuring in both direct and indirect static eval.

## Verification

- PASS: `cargo test -p ts2wasm-cli --test node_shim_host static_direct_eval_var_destructuring_lands_in_caller_scope -- --nocapture`
- PASS: `cargo test -p ts2wasm-cli --test node_shim_host static_direct_eval_for_init_var_destructuring_lands_in_caller_scope -- --nocapture`
- PASS: `cargo test -p ts2wasm-cli --test node_shim_host static_indirect_eval_var_destructuring_lands_on_global_object -- --nocapture`
- PASS: `cargo test -p ts2wasm-cli --test node_shim_host static_indirect_eval_for_init_var_destructuring_lands_on_global_object -- --nocapture`
- PASS: `cargo test -p ts2wasm-cli --test m11_host_deny static_direct_eval_for_head_var_declares_no_node_host_eval_capability -- --nocapture`
- PASS: `cargo test -p ts2wasm-cli --test m11_host_deny static_direct_eval_for_head_var_destructuring_declares_no_node_host_eval_capability -- --nocapture`
- PASS: `cargo test -p ts2wasm-cli --test m11_host_deny static_indirect_eval_var_global_declares_no_node_host_eval_capability -- --nocapture`
- PASS: `cargo test -p ts2wasm-cli --test m11_host_deny static_indirect_eval_for_head_var_destructuring_global_declares_no_node_host_eval_capability -- --nocapture`
- PASS: `cargo check -p ts2wasm-ir -p ts2wasm-compiler -p ts2wasm-cli`
- PASS: `python scripts/issue-lint.py`
- FAIL (existing baseline): `python scripts/manager.py check fixtures`
  - Existing uncataloged fixtures remain in `builtins-and-io/` and `classes-and-inheritance/`; the new `core-semantics` fixtures are cataloged.
- FAIL (existing baseline): `cargo fmt --all --check`
  - Existing rustfmt diffs remain in `crates/backend-wasm/src/runtime/host/emit.rs`, `crates/backend-wasm/src/runtime_typed_arrays.rs`, and `crates/ir/src/lowered/resolver/expr/property.rs`.
