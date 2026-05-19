# eval / Function PR C-D: static eval for-init var declaration landing

Run ID: `eval-function-pr-cd-static-eval-for-init-var`
Date: 2026-05-19

## Summary

- Extended the static eval declaration collectors to include traditional `for` init statements, not only loop bodies and `for-in` / `for-of` heads.
- Added direct eval coverage for `eval("for (var x = 1; false;) {} x")`, verifying the `var` lands in the caller scope and remains readable by later normal code.
- Added indirect eval coverage for the same construct, verifying the binding lands on `globalThis` without capturing the caller lexical binding.
- Added both fixtures to catalog and standalone host-deny coverage so the static AOT lane remains free of `host.eval.*` imports.

## Verification

- PASS: `cargo test -p ts2wasm-cli --test node_shim_host static_direct_eval_for_init_var_lands_in_caller_scope -- --nocapture`
- PASS: `cargo test -p ts2wasm-cli --test node_shim_host static_indirect_eval_for_init_var_lands_on_global_object -- --nocapture`
- PASS: `cargo test -p ts2wasm-cli --test m11_host_deny static_direct_eval_for_head_var_destructuring_declares_no_node_host_eval_capability -- --nocapture`
- PASS: `cargo test -p ts2wasm-cli --test m11_host_deny static_indirect_eval_for_head_var_destructuring_global_declares_no_node_host_eval_capability -- --nocapture`
- PASS: `cargo check -p ts2wasm-ir -p ts2wasm-compiler -p ts2wasm-cli`
- PASS: `python scripts/issue-lint.py`
- FAIL (existing baseline): `python scripts/manager.py check fixtures`
  - Existing uncataloged fixtures remain in `builtins-and-io/` and `classes-and-inheritance/`; the new `core-semantics` fixtures are cataloged.
- FAIL (existing baseline): `cargo fmt --all --check`
  - Existing rustfmt diffs remain in `crates/backend-wasm/src/runtime/host/emit.rs` and `crates/backend-wasm/src/runtime_typed_arrays.rs`.

## Notes

- Stale unrelated runtime dirty files were restored before the final focused verification; this slice only changes eval expansion, tests, catalog, issue notes, fixtures, and this report.
