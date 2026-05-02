# Plan: Issue 5009 — ImportNamespace slice

## Objective

Implement `import * as ns from "./mod"` (ImportNamespace) in the entry module. This binds all named exports from a dependency module into a namespace object accessible as `ns.exportName`.

## Steps

1. In `crates/compiler/src/lib.rs`, add `Stmt::ImportNamespace` handler in `lower_static_named_import_bindings_for_build`
2. Collect all named exports from the dependency via `collect_literal_named_exports`
3. Create an object literal expression from the exports and bind it as a `Stmt::Let`
4. Add fixture pair (source module with exports, entry with `import * as ns`)
5. Add build smoke test

## Validation

- `cargo nextest run -E 'test(module)'`
- `cargo nextest run -p ts2wasm-compiler`
