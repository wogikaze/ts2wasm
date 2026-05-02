# Issue 5037: Complete entry module export lowering for local references

## Summary

Current `populate_static_module_exports_for_build` rejects exports referencing locals
(`contains_local_ref`). We need to instead emit proper module-export statements that
reference the local's value.

## Design

### Problem overview

In the current pipeline:
1. `export { value }` → `ModuleExport { name: "value", lowered_statement_index: 0 }`
2. `top_level_statements[0]` = `LoweredStmt::Let("value", LoweredExpr::Number(1))`
3. `contains_local_ref` returns false for `Number(1)` → OK

But for:
1. `let x = 1; export { x };` → the `export { x }` creates `ModuleExport { name: "x", lowered_statement_index: 0 }`
2. But the `Let` at index 0 holds `LoweredExpr::Local(LocalId(0))` — the **name** `x` resolved to local slot 0
3. `contains_local_ref` returns true → rejected

### Fix: Allow local refs through by emitting Let + Export

In `populate_static_module_exports_for_build`, when an export references a local:
1. Keep the `Let` statement in place (already there)
2. Clone the export expression into the `Export` statement (allow `Local` references)
3. The module export emit code in the backend reads the local's value at initialization time

### Changes needed

1. **`crates/compiler/src/lib.rs`** — Remove the `contains_local_ref` rejection in `populate_static_module_exports_for_build`. Allow `Local` references to pass through to `Export` statements.

2. **Fixtures** — Update `static-named-export-unsupported.ts` if needed. The test `static_named_export_reports_issue_5005` may need to change from `assert_fixture_build_smoke` to a proper assertion.

3. **Tests** — Update tests if behavior changes.

### Verification

```sh
cargo fmt --all --check
cargo nextest run
```

## Acceptance criteria

- [x] Local binding exports are no longer rejected
- [x] Existing module fixtures maintain backward compatibility
- [x] fmt + nextest pass
