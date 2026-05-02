# Plan: Combined default + named import (ImportDefaultNamed)

## Problem

`import x, { y } from "./mod"` (combined default + named import) is rejected by `issue-055` in the builtin resolver because the compiler's rewrite pass (`lower_static_named_import_bindings_for_build`) has no handler for `Stmt::ImportDefaultNamed`.

## Design

The handler combines the ImportDefault and ImportNamed patterns:

1. **Resolve the dependency** from `module_graph.entry().dependencies()` by matching `source.value`
2. **Read literal exports** from the dependency via `collect_literal_named_exports`
3. **Default import** (`x` from `import x, { y } from "./mod"`):
   - Look up `"default"` in the exports map (same as ImportDefault handler)
   - Create a `StaticNamedImportBinding` with `imported_name: "default"` for later read-rewriting
   - Push a `Stmt::Let` with the default export's initializer
4. **Named imports** (`{ y }` from `import x, { y } from "./mod"`):
   - For each specifier: look up its `imported` name in exports map (same as ImportNamed handler)
   - Create a `StaticNamedImportBinding` for each
   - Push a `Stmt::Let` for each
5. Track in `local_name_to_index` for ExportNamed list references

## Changes

### `crates/compiler/src/lib.rs`

Add a new match arm in `lower_static_named_import_bindings_for_build`:

```rust
Stmt::ImportDefaultNamed {
    default,
    specifiers,
    source,
    ..
} => {
    let dependency = module_graph
        .entry()
        .dependencies()
        .iter()
        .find(|dependency| dependency.specifier() == source.value)
        .ok_or_else(|| Diagnostic {
            code: DiagCode::InvariantViolation,
            message: format!(
                "module graph has no dependency for combined import `{}`",
                source.value
            ),
            span: Some(source.span),
        })?;
    let exports = collect_literal_named_exports(dependency.resolved_path())?;

    // Default import: x from import x, { y } from "./mod"
    let default_expr = exports.get("default").ok_or_else(|| Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!(
            "issue-233: module `{}` does not have a default export",
            source.value
        ),
        span: Some(source.span),
    })?;
    let default_binding = StaticNamedImportBinding {
        source_specifier: source.value.clone(),
        source_module_id: dependency.resolved_module_id(),
        source_path: dependency.resolved_path().to_path_buf(),
        imported_name: "default".to_owned(),
        local_name: default.local.clone(),
        lowered_statement_index,
        initializer: default_expr.clone(),
    };
    rewritten.push(Stmt::Let {
        name: default.local.clone(),
        expr: default_expr.clone(),
        span: default.local_span,
    });
    local_name_to_index.insert(default.local.clone(), lowered_statement_index);
    named_imports.push(default_binding);
    lowered_statement_index += 1;

    // Named imports: { y } from import x, { y } from "./mod"
    for specifier in specifiers {
        let expr = exports.get(&specifier.imported).ok_or_else(|| Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!(
                "issue-233: module `{}` does not export named binding `{}`",
                source.value, specifier.imported
            ),
            span: Some(specifier.imported_span),
        })?;
        let binding = StaticNamedImportBinding {
            source_specifier: source.value.clone(),
            source_module_id: dependency.resolved_module_id(),
            source_path: dependency.resolved_path().to_path_buf(),
            imported_name: specifier.imported.clone(),
            local_name: specifier.local.clone(),
            lowered_statement_index,
            initializer: expr.clone(),
        };
        rewritten.push(Stmt::Let {
            name: binding.local_name.clone(),
            expr: binding.initializer.clone(),
            span: specifier.local_span,
        });
        local_name_to_index.insert(binding.local_name.clone(), lowered_statement_index);
        named_imports.push(binding);
        lowered_statement_index += 1;
    }
}
```

### `crates/cli/tests/m9_modules.rs`

Add build smoke test for combined import:

```rust
#[test]
fn static_combined_named_import_entry_build_smoke() {
    assert_fixture_build_smoke("module-system/static-combined-named-import-entry.ts");
}
```

### Fixtures

- `fixtures/module-system/static-combined-named-import-source.ts`: `export const x = 1; export default 42;`
- `fixtures/module-system/static-combined-named-import-entry.ts`: `import value, { x } from "./static-combined-named-import-source"; console.log(value, x);`

## Validation

```
cargo fmt --all --check
cargo nextest run -E 'test(module)'
```
