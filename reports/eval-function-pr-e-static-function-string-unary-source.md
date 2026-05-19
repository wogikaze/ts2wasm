# eval / Function PR E: static Function string unary source

Run ID: `eval-function-pr-e-static-function-string-unary-source`
Date: 2026-05-19

## Summary

- Extended `StaticFunctionConstructorSource` ToString classification for focused string numeric unary coercion.
- `Function(+"2.5")`, `Function(-"1.5")`, and `Function(+"")` now fold through compile-time ToNumber + ToString and stay in the static AOT lane.
- Added Node differential and host-deny coverage to guard stdout, generated source text, and standalone manifest behavior.

## Verification

- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-fn-unary-string-target cargo test -p ts2wasm-cli --test node_shim_host static_function_constructor_string_unary_source_uses_aot_lane -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-fn-unary-string-target cargo test -p ts2wasm-cli --test m11_host_deny standalone_fixtures_pass_host_deny -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-fn-unary-string-target cargo check -p ts2wasm-ir -p ts2wasm-compiler -p ts2wasm-cli`
- PASS: `python scripts/issue-lint.py`
- FAIL (existing baseline): `python scripts/manager.py check fixtures`
  - Existing uncataloged fixtures remain in `builtins-and-io/` and `classes-and-inheritance/`; the new `core-semantics` fixture is cataloged.
- FAIL (existing baseline): `CARGO_TARGET_DIR=/tmp/ts2wasm-fn-unary-string-target cargo fmt --all --check`
  - Existing rustfmt diffs remain in `crates/backend-wasm/src/runtime/host/emit.rs` and `crates/backend-wasm/src/runtime_typed_arrays.rs`.
