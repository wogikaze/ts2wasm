# Plan: Combined default + namespace import + issue-055 narrowing

## Problem

`import x, * as ns from "./mod"` hit `issue-055`. It's the last static module form without a compiler handler. Also, the `issue-055` catch-all still lists all import/export forms even though all entry-module forms are now handled.

## Design

### ImportDefaultNamespace handler

Combine ImportDefault + ImportNamespace patterns:
1. Resolve dependency
2. Read literal exports via `collect_literal_named_exports`
3. Default import: create `Stmt::Let` + `StaticNamedImportBinding` (like ImportDefault)
4. Namespace import: create `Stmt::Let` with `Expr::Object` (like ImportNamespace)

### issue-055 narrowing

Remove all handled forms from the catch-all match, leaving only `ImportDefaultNamespace` (soon to be removed) and `ImportDefaultNamespace` as a safety net. After this slice adds the handler, remove the catch-all arm entirely.

## Changes

### `crates/compiler/src/lib.rs`

Add handler in `lower_static_named_import_bindings_for_build`:

```rust
Stmt::ImportDefaultNamespace {
    default, namespace, source, span, ..
} => {
    let dependency = module_graph
        .entry()
        .dependencies()
        .iter()
        .find(|dependency| dependency.specifier() == source.value)
        .ok_or_else(|| Diagnostic { ... })?;
    let exports = collect_literal_named_exports(dependency.resolved_path())?;

    // Default import: `x` from `import x, * as ns from "./mod"`
    let default_expr = exports.get("default").ok_or_else(|| Diagnostic { ... })?;
    let default_binding = StaticNamedImportBinding {
        source_specifier: source.value.clone(),
        source_module_id: dependency.resolved_module_id(),
        source_path: dependency.resolved_path().to_path_buf(),
        imported_name: "default".to_owned(),
        local_name: default.local.clone(),
        lowered_statement_index,
        initializer: default_expr.clone(),
    };
    rewritten.push(Stmt::Let { name: default.local.clone(), expr: default_expr.clone(), span: default.local_span });
    local_name_to_index.insert(default.local.clone(), lowered_statement_index);
    named_imports.push(default_binding);
    lowered_statement_index += 1;

    // Namespace import: `* as ns` from `import x, * as ns from "./mod"`
    let props: Vec<(String, Expr)> = exports.iter()
        .filter(|(k, _)| k != "default")
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    rewritten.push(Stmt::Let {
        name: namespace.local.clone(),
        expr: Expr::Object { props, span: *span },
        span: namespace.span,
    });
    local_name_to_index.insert(namespace.local.clone(), lowered_statement_index);
    lowered_statement_index += 1;
}
```

### `crates/ir/src/builtin_resolver.rs`

Narrow the issue-055 catch-all to only list forms not yet handled. Since all entry-module forms are now handled, remove the catch-all arm entirely.

### `crates/cli/tests/m9_modules.rs`

Add build smoke test.

### Fixtures

- `fixtures/module-system/static-default-namespace-import-source.ts`: `export const x = 1; export default 42;`
- `fixtures/module-system/static-default-namespace-import-entry.ts`: `import value, * as ns from "./static-default-namespace-import-source";`

## Validation

```
cargo fmt --all --check
cargo nextest run -E 'test(module)'
```
