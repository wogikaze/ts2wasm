# Plan: Issue 5009 — Remaining static ES module export forms

## Objective

Implement `export { x, y }` (ExportNamed with specifiers) in the entry module. This is the next simplest slice after 5008.

## Steps

1. In `crates/compiler/src/lib.rs`, extend the `ExportNamed` handler in `lower_static_named_import_bindings_for_build` to process non-empty specifiers
2. Record each specifier as a `ModuleExport`, matching the local binding name in lowered statements
3. Verify `export { x, y }` builds to WASM without issue-055
4. Add fixture and build smoke test

## Validation

- `cargo nextest run -E 'test(module) or test(export)'`
- `cargo nextest run -p ts2wasm-compiler`
