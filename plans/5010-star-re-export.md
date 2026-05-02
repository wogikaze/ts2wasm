# Plan: Star re-export (ExportAllFrom)

## Problem

`export * from "./mod"` is rejected by `issue-055` in the builtin resolver because the compiler's rewrite pass has no handler for `Stmt::ExportAllFrom`.

## Design

For `export * from "./mod"`, all named exports from the dependency become exports of the entry module. The approach mirrors ImportNamed's mechanism:

1. **Resolve the dependency** from `module_graph.entry().dependencies()`
2. **Read literal exports** from the dependency via `collect_literal_named_exports`
3. **For each export** (name + value):
   - Create a generated local variable `__ts2wasm_re_<name>` with a `Stmt::Let` using the initial value
   - Create a `StaticNamedImportBinding` so `lower_static_named_import_reads_for_build` rewrites it to `ModuleLoad(dep_id).<name>`
   - Create a `ModuleExport { name, lowered_statement_index }` so `populate_static_module_exports_for_build` generates the entry module's `Export` statement

## Changes

### `crates/compiler/src/lib.rs`

Add a new match arm in `lower_static_named_import_bindings_for_build`:

```rust
Stmt::ExportAllFrom { source, .. } => {
    let dependency = module_graph
        .entry()
        .dependencies()
        .iter()
        .find(|dependency| dependency.specifier() == source.value)
        .ok_or_else(|| Diagnostic {
            code: DiagCode::InvariantViolation,
            message: format!(
                "module graph has no dependency for re-export `{}`",
                source.value
            ),
            span: Some(source.span),
        })?;
    let exports = collect_literal_named_exports(dependency.resolved_path())?;
    for (export_name, expr) in &exports {
        let local_name = format!("__ts2wasm_re_{export_name}");
        rewritten.push(Stmt::Let {
            name: local_name.clone(),
            expr: expr.clone(),
            span: source.span,
        });
        local_name_to_index.insert(local_name.clone(), lowered_statement_index);
        named_imports.push(StaticNamedImportBinding {
            source_specifier: source.value.clone(),
            source_module_id: dependency.resolved_module_id(),
            source_path: dependency.resolved_path().to_path_buf(),
            imported_name: export_name.clone(),
            local_name: local_name.clone(),
            lowered_statement_index,
            initializer: expr.clone(),
        });
        module_exports.push(ModuleExport {
            name: export_name.clone(),
            lowered_statement_index,
        });
        lowered_statement_index += 1;
    }
}
```

### `crates/cli/tests/m9_modules.rs`

Add build smoke test:

```rust
#[test]
fn static_star_re_export_entry_build_smoke() {
    assert_fixture_build_smoke("module-system/static-star-re-export-entry.ts");
}
```

### Fixtures

- `fixtures/module-system/static-star-re-export-source.ts`: `export const x = 1; export const y = 2;`
- `fixtures/module-system/static-star-re-export-entry.ts`:

  ```
  export * from "./static-star-re-export-source";
  ```

## Validation

```
cargo fmt --all --check
cargo nextest run -E 'test(module)'
```
