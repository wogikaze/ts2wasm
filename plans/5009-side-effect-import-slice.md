# Plan: Issue 5009 — ImportSideEffect slice

## Objective

Implement `import "./mod"` (ImportSideEffect) in the entry module. Side-effect imports execute the dependency module's initialization but don't bind any local names.

## Steps

1. In `crates/compiler/src/lib.rs`, add `Stmt::ImportSideEffect` handler in `lower_static_named_import_bindings_for_build`
2. Verify the dependency exists in the module graph, then skip (no binding needed)
3. Add fixture pair and build smoke test

## Validation

- `cargo nextest run -E 'test(module)'`
- `cargo nextest run -p ts2wasm-compiler`
