# Plan: Issue 5009 — ImportDefault slice

## Objective

Implement `import x from "./mod"` (ImportDefault) in the entry module. This reads the "default" export from the dependency module and binds it locally.

## Steps

1. In `crates/compiler/src/lib.rs`, add `Stmt::ImportDefault` handler in `lower_static_named_import_bindings_for_build`
2. Look up "default" export from the dependency module via `collect_literal_named_exports`
3. Push a `Stmt::Let` for the binding, record in `named_imports` and `local_name_to_index`
4. Add fixture pair (source with `export default`, entry with `import x from`)
5. Add build smoke test
6. Update any existing issue-055/fixture tests that now succeed

## Validation

- `cargo nextest run -E 'test(module)'`
- `cargo nextest run -p ts2wasm-compiler`
