# Plan: Named re-export from (ExportNamedFrom)

## Problem

`export { x } from "./mod"` and `export { x as y } from "./mod"` are rejected by `issue-055` because the compiler has no handler for `Stmt::ExportNamedFrom`.

## Design

For each specifier in the export list:
1. Look up the `imported` name in the dependency's exports
2. Create a generated local variable `__ts2wasm_re_<name>` with a `Stmt::Let` using the initial value
3. Create a `StaticNamedImportBinding` for read-rewriting to `ModuleLoad(dep_id).<imported>`
4. Create a `ModuleExport { name: <exported>, lowered_statement_index }`

This is almost identical to ExportAllFrom but uses the specifier's imported name (not iterating all exports).

## Changes

### `crates/compiler/src/lib.rs`

Add match arm in `lower_static_named_import_bindings_for_build`:

```rust
Stmt::ExportNamedFrom { specifiers, source, .. } => {
    let dependency = module_graph
        .entry()
        .dependencies()
        .iter()
        .find(|dependency| dependency.specifier() == source.value)
        .ok_or_else(|| Diagnostic { ... })?;
    let exports = collect_literal_named_exports(dependency.resolved_path())?;
    for specifier in specifiers {
        let expr = exports.get(&specifier.imported).ok_or_else(|| Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!("issue-233: module `{}` does not export named binding `{}`", ...),
            span: Some(specifier.imported_span),
        })?;
        let local_name = format!("__ts2wasm_re_{}", specifier.exported);
        rewritten.push(Stmt::Let { name: local_name.clone(), expr: expr.clone(), span: specifier.span });
        local_name_to_index.insert(local_name.clone(), lowered_statement_index);
        named_imports.push(StaticNamedImportBinding {
            source_specifier: source.value.clone(),
            source_module_id: dependency.resolved_module_id(),
            source_path: dependency.resolved_path().to_path_buf(),
            imported_name: specifier.imported.clone(),
            local_name: local_name.clone(),
            lowered_statement_index,
            initializer: expr.clone(),
        });
        module_exports.push(ModuleExport {
            name: specifier.exported.clone(),
            lowered_statement_index,
        });
        lowered_statement_index += 1;
    }
}
```

### `crates/cli/tests/m9_modules.rs`

```rust
#[test]
fn static_named_re_export_from_entry_build_smoke() {
    assert_fixture_build_smoke("module-system/static-named-re-export-from-entry.ts");
}
```

### Fixtures

- `fixtures/module-system/static-named-re-export-from-source.ts`: `export const x = 1; export const y = 2;`
- `fixtures/module-system/static-named-re-export-from-entry.ts`: `export { x } from "./static-named-re-export-from-source";`

## Validation

```
cargo fmt --all --check
cargo nextest run -E 'test(module)'
```
