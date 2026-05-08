---
id: 5009
title: "Remaining static ES module export forms (named list, default import, namespace, re-export, side-effect) (audit reopened #5009)"
type: feature
area: ir/compiler
class: done
priority: P1
depends_on: []
blocks: [5010]
status: done
created: 2026-05-02
updated: 2026-05-06
---

## Summary

Issue 5008 implemented `export const x = 1` (ExportDecl) and `export default <expr>` for entry modules without imports. The remaining static ES module export/import forms still hit `issue-055` unsupported diagnostics.

## Remaining forms

- `export { x, y }` / `export { x as y }` (named export lists in entry module)
- `import x from "./mod"` (default import from another module)
- `import x, { y } from "./mod"` (combined default + named import)
- `import * as ns from "./mod"` (namespace import)
- `import "./mod"` (side-effect import)
- `export * from "./mod"` (star re-export)
- `export { x } from "./mod"` (named re-export from)
- `export * as ns from "./mod"` (namespace re-export)

## Scope

- [x] Rewrite `ExportNamed` in `lower_static_named_import_bindings_for_build` for `export { x, y }`
- [x] Rewrite `ImportDefault` for `import x from "./mod"`
- [x] Rewrite `ImportNamespace` for `import * as ns from "./mod"`
- [x] Rewrite `ImportSideEffect` for `import "./mod"`
- Remaining forms (ImportDefaultNamed, re-exports, differential tests, issue-055 narrowing) tracked in issue 5010

## Acceptance criteria

- [x] `export { x, y }` builds to WASM and both names are accessible
- [x] `import x from "./mod"` builds to WASM and reads the default export
- [x] `import * as ns from "./mod"` builds to WASM and `ns.x` accesses named exports
- [x] `import "./side-effect"` triggers module initialization
- Remaining acceptance criteria tracked in issue 5010

## Validation

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli module
cargo nextest run -p ts2wasm-compiler
```

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/5009-remaining-es-module-export-forms.md` before this move
- `issues/open/5009-remaining-es-module-export-forms.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.

## Completion evidence

Completed by the static module import/export lowering slices and re-verified on 2026-05-06.

Implemented behavior:

- Local named export lists are preserved in dependency module export metadata and are readable through static named imports.
- Static default imports read source-module `"default"` exports.
- Static namespace imports materialize an object with named exports.
- Static side-effect imports trigger dependency module initialization, including supported top-level side-effect statements such as `console.log(...)`.
- Combined default + named imports and existing module smoke coverage remain green; broader re-export and diagnostic cleanup remains tracked by `5010`.

Repo-local evidence:

- `crates/compiler/src/lib.rs`
- `crates/cli/tests/common/m2_node_diff_fixture_tests.rs`
- `fixtures/module-system/static-named-list-import-entry.ts`
- `fixtures/module-system/static-named-list-import-source.ts`
- `fixtures/module-system/static-default-import-entry.ts`
- `fixtures/module-system/static-default-import-source.ts`
- `fixtures/module-system/static-namespace-import-entry.ts`
- `fixtures/module-system/static-namespace-import-source.ts`
- `fixtures/module-system/static-combined-named-import-entry.ts`
- `fixtures/module-system/static-combined-named-import-source.ts`
- `fixtures/module-system/static-side-effect-import-entry.ts`
- `fixtures/module-system/static-side-effect-source.ts`

Validation:

- `cargo nextest run -p ts2wasm-cli static_named_export_list_import_fixture_matches_node_output_under_iwasm static_namespace_module_import_fixture_matches_node_output_under_iwasm static_combined_named_module_import_fixture_matches_node_output_under_iwasm static_side_effect_module_import_fixture_matches_node_output_under_iwasm` => pass (`4 tests run: 4 passed, 648 skipped`)
- `cargo nextest run -p ts2wasm-cli module` => pass (`24 tests run: 24 passed, 628 skipped`)
- `cargo nextest run -p ts2wasm-compiler` => pass (`58 tests run: 58 passed, 0 skipped`)
- `cargo fmt --all --check` => pass
- `git diff --check` => pass
