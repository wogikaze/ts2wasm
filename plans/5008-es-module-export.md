# Plan: Issue 5008 — Implement static ES module export forms

## Objective

Implement the simplest slice: fix `export const x = 1` (ExportDecl) when the file has no `import` from another module.

Currently the compiler's module rewrite path only triggers for files with named imports. `export const x = 1` in a file without imports hits `issue-055`.

## Steps

1. In `crates/compiler/src/lib.rs`, find where module rewrite is gated on `has_named_imports` and add a condition for `has_exports` (or detect any export statement)
2. Verify `export const x = 1` builds to WASM without issue-055
3. Add/update Node/iwasm differential fixtures for export forms

## Validation

- `cargo nextest run -E 'test(module) or test(export)'`
- `cargo nextest run -p ts2wasm-compiler`
