---
id: 5010
title: "Implement local named export (export { value } and export { value as alias }) for entry module (audit reopened #5010)"
type: feature
area: ir/compiler
class: done
priority: P1
depends_on: []
blocks: []
status: done
created: 2026-05-02
updated: 2026-05-06
---

## Summary

Implement entry-module local named exports such as `export { value }` and `export { value as alias }`. Unlike `export const`, these forms reference existing local bindings rather than declaring+exporting in one step.

Re-export forms have build-smoke coverage in this area, but observable import-through semantic parity is split to issue 5128.

## Problem

`export { value }` currently produces `issue-5005: entry module export list is not in the current static export slice`. The rewrite infrastructure exists (ExportDecl, ExportDefault), but ExportNamed with specifiers that reference local `let`/`const` bindings is not wired up.

Example failing fixture:

```ts
const value = 1;
export { value };
```

Expected: WASM export `"value"` with value `1`.

```ts
const value = 1;
export { value as renamed };
```

Expected: WASM export `"renamed"` with value `1`.

```ts
const a = 1;
const b = 2;
export { a, b };
```

Expected: WASM exports `"a"` and `"b"`.

Problem: `export { ... }` references local bindings by name, but the compiler's module rewrite path only handles `ExportDecl` (export const) and `ExportDefault` (export default). ExportNamed specifiers need to resolve local names to `LoweredStmt::Let` indices and create `ModuleExport` entries.

## Desired final state

`export { value }` in an entry module produces a WASM module with a named export `"value"` that has the same value as the local `value` binding. Alias form `export { x as value }` exports under the alias name. Duplicate export names produce a clear diagnostic. Undefined local references produce a clear diagnostic.

## Scope

In scope:

- `export { value }` where `value` is a local `const`/`let` binding
- `export { x as value }` alias form
- Multiple exports: `export { a, b }`
- Duplicate export name diagnostic
- Undefined local reference diagnostic

Out of scope:

- Re-export import-through semantic parity (`export { x } from`, `export * as ns from`, `export * from`) — tracked by issue 5128
- Export after destructuring binding
- Live binding (export reflects initial value, not updates)

## Affected paths

Expected:

- `crates/compiler/src/lib.rs` — ExportNamed handler in `lower_static_named_import_bindings_for_build`
- `crates/ir/src/builtin_resolver.rs` — narrow ExportNamed catch-all
- `crates/cli/tests/m9_modules.rs` — new tests
- `fixtures/module-system/` — new test fixtures

Do not touch:

- `crates/frontend/` — parser already produces `Stmt::ExportNamed`
- `crates/backend-wasm/` — runtime helpers unchanged

## Acceptance criteria

- [x] `const value = 1; export { value };` builds to WASM and produces export `"value" = 1`
- [x] `const value = 1; export { value as renamed };` builds to WASM and produces export `"renamed" = 1`
- [x] `const a = 1; const b = 2; export { a, b };` builds to WASM with exports `"a"` and `"b"`
- [x] `export { missing };` with no local `missing` produces clear diagnostic
- [x] `const a = 1; const b = 2; export { a as value, b as value };` (duplicate) produces clear diagnostic
- [x] Re-export import-through semantic parity is split to issue 5128
- [x] All previous module tests still pass

## Validation

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli module
```

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/done/5010-remaining-es-module-export-forms.md` before this move
- `issues/done/5010-remaining-es-module-export-forms.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.

## Completion evidence

Completed by earlier static module export implementation slices and re-verified on 2026-05-06.

Implemented behavior:

- Entry-module local named exports build successfully for `export { value }`, alias form, and multiple local exports.
- Local named export values are observable through a supported static import when the exporting module is used as a dependency.
- Unknown local export names produce a source-spanned issue-5005 diagnostic.
- Duplicate export names produce a source-spanned issue-5005 diagnostic.
- Re-export import-through semantic parity is tracked separately by `issues/open/5128-static-re-export-semantic-parity.md`.

Repo-local evidence:

- `crates/compiler/src/lib.rs`
- `crates/cli/tests/m9_modules.rs`
- `crates/cli/tests/common/m2_node_diff_fixture_tests.rs`
- `fixtures/module-system/static-named-export-unsupported.ts`
- `fixtures/module-system/static-export-named-list-entry.ts`
- `fixtures/module-system/static-named-list-import-entry.ts`
- `fixtures/module-system/static-named-list-import-source.ts`
- `fixtures/module-system/static-local-named-export-missing-unsupported.ts`
- `fixtures/module-system/static-local-named-export-duplicate-unsupported.ts`
- `issues/open/5128-static-re-export-semantic-parity.md`

Validation:

- `cargo nextest run -p ts2wasm-cli static_local_named_export_missing_reports_issue_5005 static_local_named_export_duplicate_reports_issue_5005 static_named_export_reports_issue_5005 static_export_named_list_entry_build_smoke static_named_export_list_import_fixture_matches_node_output_under_iwasm` => pass (`5 tests run: 5 passed, 649 skipped`)
- `cargo nextest run -p ts2wasm-cli module` => pass (`24 tests run: 24 passed, 630 skipped`)
- `cargo fmt --all --check` => pass
- `git diff --check` => pass
