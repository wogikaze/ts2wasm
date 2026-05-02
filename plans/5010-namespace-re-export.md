# Plan: Namespace re-export (ExportNamespaceFrom)

## Problem

`export * as ns from "./mod"` is rejected by `issue-055` because the compiler has no handler for `Stmt::ExportNamespaceFrom`.

## Design

`export * as ns from "./mod"` re-exports all named exports from a dependency under a single namespace object. The approach mirrors ImportNamespace but also creates a ModuleExport entry:

1. Resolve dependency
2. Read literal exports via `collect_literal_named_exports`
3. Create an `Expr::Object { props: exports }` literal (self-contained, like ImportNamespace)
4. Create a `Stmt::Let` with a generated local variable
5. Create a `ModuleExport { name: <namespace_name>, lowered_statement_index }`

No `StaticNamedImportBinding` needed — the Expr::Object is self-contained.

## Changes

### `crates/compiler/src/lib.rs`

Add match arm in `lower_static_named_import_bindings_for_build`:

```rust
Stmt::ExportNamespaceFrom {
    namespace, source, span, ..
} => {
    let dependency = module_graph
        .entry()
        .dependencies()
        .iter()
        .find(|dependency| dependency.specifier() == source.value)
        .ok_or_else(|| Diagnostic { ... })?;
    let exports = collect_literal_named_exports(dependency.resolved_path())?;
    let props: Vec<(String, Expr)> = exports.into_iter().collect();
    let local_name = format!("__ts2wasm_ns_{}", namespace.exported);
    rewritten.push(Stmt::Let {
        name: local_name.clone(),
        expr: Expr::Object { props, span: *span },
        span: namespace.span,
    });
    local_name_to_index.insert(local_name.clone(), lowered_statement_index);
    module_exports.push(ModuleExport {
        name: namespace.exported.clone(),
        lowered_statement_index,
    });
    lowered_statement_index += 1;
}
```

### `crates/cli/tests/m9_modules.rs`

```rust
#[test]
fn static_namespace_re_export_from_entry_build_smoke() {
    assert_fixture_build_smoke("module-system/static-namespace-re-export-from-entry.ts");
}
```

### Fixtures

- `fixtures/module-system/static-namespace-re-export-from-source.ts`: `export const x = 1; export const y = 2;`
- `fixtures/module-system/static-namespace-re-export-from-entry.ts`: `export * as ns from "./static-namespace-re-export-from-source";`

## Validation

```
cargo fmt --all --check
cargo nextest run -E 'test(module)'
```
