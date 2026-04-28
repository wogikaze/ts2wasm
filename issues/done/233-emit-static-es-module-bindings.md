---
id: 233
title: "Emit static ES module bindings"
type: feature
area: ir/backend
class: implementation-ready
priority: P1
depends_on: [231, 232]
blocks: [234]
created: 2026-04-28
updated: 2026-04-28
completed: 2026-04-28
status: done
---

## Summary

Lower resolved ES module imports/exports into module initialization, export storage, and import reads in the existing WASM runtime model.

## Problem

The backend has CommonJS-oriented module cache helpers, but static ES module bindings are not represented through lowering and emission. Importing code needs to read exported bindings without weakening the existing runtime/capability boundaries.

## Desired final state

Named static exports are emitted into a module export object and named imports read those bindings through the resolved module graph. Modules initialize once in dependency order.

## Scope

In scope:

- [x] Lower named exports from declarations and export lists into resolved IR
- [x] Lower named imports to reads from resolved module exports
- [x] Emit module initialization in dependency order using the module graph IDs
- [x] Reuse or extend existing module cache/export runtime helpers without unconditional host imports
- [x] Add backend/link-plan tests showing ES module helpers are included only when needed

Out of scope:

- [x] Live binding updates beyond simple top-level values
- [x] `export default`, namespace objects, package resolution, and dynamic import
- [x] Broad module semantic parity claims without execution fixtures

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/compiler/src/`
- `crates/cli/tests/`
- `fixtures/module-system/`

Do not touch:

- `crates/runtime-abi/` unless a layout constant is strictly required and reviewed in this issue
- `docs/`

## Acceptance criteria

- [x] Simple named export/import programs build to WASM
- [x] Imported values are read from the resolved source module, not from lexical globals in the importer
- [x] Module initialization runs once per module for repeated imports
- [x] Runtime link plan includes module helpers only for programs that use modules
- [x] Existing CommonJS module-cache fixtures still build

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir
cargo nextest run -p ts2wasm-backend-wasm
cargo nextest run -p ts2wasm-cli module
```

Impacted commands:

```sh
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry.ts -o /tmp/ts2wasm-esm.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

Follow `docs/12-coding-standard.md`: runtime helpers, imports, capabilities, and manifest behavior must be derived through the runtime/link-plan catalog, not hard-coded in the emitter.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `8ef5f5584941f7c5c3069b388c449da3eea1db71` issue-233: add static module runtime semantic coverage

Validation result:

```text
cargo fmt --all --check: PASS
cargo nextest run -p ts2wasm-ir: PASS (18 tests)
cargo nextest run -p ts2wasm-backend-wasm: PASS (19 tests)
cargo nextest run -p ts2wasm-compiler: PASS (39 tests)
cargo nextest run -p ts2wasm-cli module: PASS (17 tests, 220 skipped)
cargo nextest run -p ts2wasm-cli static_named_module_import_fixtures_match_node_output_under_iwasm: PASS (1 test, 236 skipped)
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry.ts -o /tmp/ts2wasm-233-semantic-entry.wasm: PASS
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-alias.ts -o /tmp/ts2wasm-233-semantic-alias.wasm: PASS
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-shadow.ts -o /tmp/ts2wasm-233-semantic-shadow.wasm: PASS
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-repeated.ts -o /tmp/ts2wasm-233-semantic-repeated.wasm: PASS
iwasm /tmp/ts2wasm-233-semantic-entry.wasm && iwasm /tmp/ts2wasm-233-semantic-alias.wasm && iwasm /tmp/ts2wasm-233-semantic-shadow.wasm && iwasm /tmp/ts2wasm-233-semantic-repeated.wasm: PASS (stdout 1, 1, 1, 2)
scripts/manager check-issue-index: PASS
scripts/manager check-issue-health: PASS
scripts/manager check-agent-state: PASS
scripts/manager check-repo-smoke: PASS
cargo nextest run: PASS (382 tests, 4 skipped)
date: 2026-04-28
```

Remaining risks:

- Static named import/export coverage is intentionally limited to local relative modules with source-side literal `export const` bindings. Live binding updates, default/namespace/dynamic imports, package resolution, and broader module body semantics remain out of scope.

## Progress evidence

2026-04-28 child worker `233-static-named-import-build-20260428T092830Z` completed the first static named import build slice:

- Reused the issue-232 compiler module graph during build and rewrote only resolved local `ImportNamed` declarations whose source module exports literal `export const` bindings.
- `fixtures/module-system/static-entry.ts` now builds to WASM with `import { value } from "./static-entry-source";` backed by `fixtures/module-system/static-entry-source.ts`.
- Unsupported static module forms remain on their existing issue-055/issue-232 diagnostics; default imports/exports, namespace imports/re-exports, declaration/class/default export unsupported fixtures were not broadened.
- Added CLI build-smoke coverage for the static named import entry fixture.
- Added backend coverage proving module runtime helpers are not emitted for plain non-module IR.

Validation:

```text
cargo fmt --all --check: PASS
cargo nextest run -p ts2wasm-ir: PASS (16 tests)
cargo nextest run -p ts2wasm-backend-wasm: PASS (16 tests)
cargo nextest run -p ts2wasm-cli module: PASS (12 tests, 219 skipped)
cargo nextest run -p ts2wasm-cli static_named_import_build_smoke: PASS (1 test, 230 skipped)
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry.ts -o /tmp/ts2wasm-esm.wasm: PASS
```

Remaining work before close:

- Lower named exports/imports into explicit resolved/lowered module binding IR instead of the current literal export build rewrite.
- Emit dependency-order module initialization and once-only execution semantics.
- Add runtime execution/differential coverage under issue 234 before making semantic parity claims.

2026-04-28 child worker `233-named-import-alias-diagnostics-20260428T093927Z` hardened the current graph-backed static named import build rewrite:

- Added `fixtures/module-system/static-entry-alias.ts` for `import { value as renamed } from "./static-entry-source";` backed by literal `export const value = 1;`.
- Added CLI build-smoke coverage proving the alias form emits WASM through the existing imported-name lookup plus local-binding rewrite.
- Added `fixtures/module-system/static-missing-named-export.ts` and CLI diagnostic coverage proving an existing local module with no requested export reports `issue-233` at the imported name span.
- No broader module binding IR, dependency-order initialization, live binding, namespace/default, or execution parity claims were added.

Validation:

```text
cargo fmt --all --check: PASS
cargo nextest run -p ts2wasm-compiler: PASS (35 tests)
cargo nextest run -p ts2wasm-cli module: PASS (14 tests, 219 skipped)
cargo nextest run -p ts2wasm-cli static_module_named_import_alias_build_smoke: PASS (1 test, 232 skipped)
cargo nextest run -p ts2wasm-cli static_module_named_import_missing_export_reports_issue_233_at_imported_name: PASS (1 test, 232 skipped)
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-alias.ts -o /tmp/ts2wasm-esm-alias.wasm: PASS
```

2026-04-28 child worker `233-static-module-ir-binding-20260428T094954Z` made a narrow explicit binding-lowering progress slice:

- Moved the temporary graph-backed named import build rewrite behind `lower_static_named_import_bindings_for_build`, which records `StaticNamedImportBinding` entries with source specifier, resolved source module ID/path, imported name, local name, and source initializer before generating the current buildable local binding.
- Added compiler regression coverage for an importer that has a same-named local `value` while importing `value as importedValue`; the binding initializer is asserted to come from the resolved source module export (`1`) and the importer lexical local remains `99`.
- Added `fixtures/module-system/static-entry-shadow.ts` plus CLI module build-smoke coverage, preserving the existing `static-entry.ts` and `static-entry-alias.ts` build behavior.
- No dependency-order module initialization, once-only execution semantics, live binding behavior, or runtime semantic parity claims were added; issue 233 remains open.

Validation:

```text
cargo nextest run -p ts2wasm-cli static_module_named_import_alias_build_smoke: PASS (pre-change reproduction, 1 test)
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry.ts -o /tmp/ts2wasm-esm-233-ir-pre.wasm: PASS (pre-change reproduction)
cargo nextest run -p ts2wasm-compiler static_named_import_binding_lowering_uses_source_export_when_importer_shadows_name: PASS (1 test)
cargo nextest run -p ts2wasm-cli static_module_named_import_shadowed_local_build_smoke: PASS (1 test)
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-shadow.ts -o /tmp/ts2wasm-esm-233-shadow-ir.wasm: PASS
cargo fmt --all --check: PASS
cargo nextest run -p ts2wasm-ir: PASS (16 tests)
cargo nextest run -p ts2wasm-backend-wasm: PASS (16 tests)
cargo nextest run -p ts2wasm-cli module: PASS (15 tests, 219 skipped)
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry.ts -o /tmp/ts2wasm-esm-233-ir.wasm: PASS
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-alias.ts -o /tmp/ts2wasm-esm-233-alias-ir.wasm: PASS
scripts/manager check-issue-health: PASS
scripts/manager check-agent-state: PASS
scripts/manager nextest: PASS (369 tests, 4 skipped)
scripts/manager check-repo-smoke: PASS
```

2026-04-28 child worker `233-module-init-once-20260428T100229Z` added a narrow source-backed module initialization contract slice:

- Added `ModuleInitializationStep` plus `ModuleGraph::dependency_first_initialization_steps()` so the issue-232 graph can produce dependency-first module initialization steps with each resolved module ID scheduled once.
- Added compiler regression coverage using a real local graph where an entry imports the same source module twice and that source imports a nested module; the asserted initialization step order is nested module, source module, entry module, and the repeated source import contributes one dependency edge in the entry step.
- Preserved the current temporary static named import build rewrite and did not add backend runtime initialization, export storage, live bindings, or runtime semantic parity claims. Issue 233 remains open.

Validation:

```text
cargo nextest run -p ts2wasm-compiler builds_dependency_first_once_only_initialization_steps_from_static_graph: PASS (1 test, 36 skipped; first run found an assertion type inference compile error, fixed before validation)
cargo fmt --all --check: PASS
cargo nextest run -p ts2wasm-compiler: PASS (37 tests)
cargo nextest run -p ts2wasm-ir: PASS (16 tests)
cargo nextest run -p ts2wasm-backend-wasm: PASS (16 tests)
cargo nextest run -p ts2wasm-cli module: PASS (15 tests, 219 skipped)
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry.ts -o /tmp/ts2wasm-233-init-entry.wasm: PASS
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-alias.ts -o /tmp/ts2wasm-233-init-alias.wasm: PASS
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-shadow.ts -o /tmp/ts2wasm-233-init-shadow.wasm: PASS
```

2026-04-28 child worker `233-module-runtime-plan-20260428T102446Z` added a narrow backend runtime-link contract slice:

- `RuntimeLinkPlan` now scans explicit lowered `ModuleInfo.statements`, so future module export statements select module export helpers through the runtime catalog.
- Added backend link-plan coverage proving empty module metadata does not select ES module export helpers, while an explicit lowered module `Export` statement selects `ModuleExportsSet` without selecting `ModuleRequire`.
- Preserved current static module build smokes and CommonJS module-cache behavior; no dependency-order runtime execution, export storage wiring, live binding behavior, or runtime semantic parity claims were added. Issue 233 remains open.

Validation:

```text
cargo fmt --all --check: PASS
cargo nextest run -p ts2wasm-ir: PASS (16 tests)
cargo nextest run -p ts2wasm-backend-wasm: PASS (18 tests)
cargo nextest run -p ts2wasm-compiler: PASS (37 tests)
cargo nextest run -p ts2wasm-cli module: PASS (15 tests, 219 skipped)
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry.ts -o /tmp/ts2wasm-233-runtime-entry.wasm: PASS
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-alias.ts -o /tmp/ts2wasm-233-runtime-alias.wasm: PASS
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-shadow.ts -o /tmp/ts2wasm-233-runtime-shadow.wasm: PASS
scripts/manager check-issue-health: PASS
scripts/manager check-agent-state: PASS
```

2026-04-28 child worker `233-module-import-exports-20260428T105318Z` added a narrow named-import lowered read slice:

- The compiler build path now keeps the source-level static named import rewrite only as declaration scaffolding for name resolution, then replaces the lowered import local initializer with `PropertyGet(ModuleLoad { module_id }, export_name)`.
- `StaticNamedImportBinding` records the lowered top-level statement index so the importer read is connected back to the resolved issue-232 module ID and imported export name after lowering.
- Extended compiler regression coverage to assert `import { value } from "./source"` lowers the importer local as a module export read while the resolved source module still appears in `LoweredProgram.modules` as `LoweredStmt::Export`.
- The resulting WAT includes `$module_require`, `$property_get`, and `$module_exports_set` through existing lowered IR and runtime-link planning. No dependency-order runtime execution, module initialization body emission, live binding behavior, or runtime semantic parity claim was added; issue 233 remains open.

Validation:

```text
cargo nextest run -p ts2wasm-compiler static_module_export_lowering_populates_explicit_lowered_module_statements: PASS (1 test)
cargo fmt --all --check: PASS
cargo nextest run -p ts2wasm-ir: PASS (18 tests)
cargo nextest run -p ts2wasm-backend-wasm: PASS (18 tests)
cargo nextest run -p ts2wasm-compiler: PASS (38 tests)
cargo nextest run -p ts2wasm-cli module: PASS (15 tests, 220 skipped)
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry.ts -o /tmp/ts2wasm-233-import-exports-entry.wasm: PASS
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-alias.ts -o /tmp/ts2wasm-233-import-exports-alias.wasm: PASS
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-shadow.ts -o /tmp/ts2wasm-233-import-exports-shadow.wasm: PASS
scripts/manager check-issue-health: PASS
scripts/manager check-agent-state: PASS
```

2026-04-28 child worker `233-module-init-runtime-20260428T114113Z` added a narrow dependency-order module initialization/runtime wiring slice:

- `populate_static_module_exports_for_build` now emits lowered module metadata in the existing `ModuleGraph::dependency_first_initialization_steps()` order, excluding the entry module, so downstream backend initialization receives dependency-first module bodies.
- The WAT backend now collects strings from lowered module statements, emits `$module_init_<id>` helpers for modules with explicit lowered statements, sets `$current_module_id` while running each helper, and calls those helpers before top-level import reads.
- Added compiler coverage proving a nested static graph populates `LoweredProgram.modules` as nested module then source module, and backend coverage proving module init helper calls preserve metadata order and run before the top-level `ModuleLoad` import read.
- Preserved existing static module build smokes for `static-entry.ts`, alias, and shadow. No live binding behavior, cycle semantics, broader module body lowering, or Node/iwasm semantic parity claim was added; issue 233 remains open.

Validation:

```text
cargo nextest run -p ts2wasm-backend-wasm explicit_module_export_statement_selects_es_module_export_helpers: PASS (pre-change reproduction, 1 test)
cargo nextest run -p ts2wasm-compiler static_module_export_lowering_orders_module_metadata_dependency_first: PASS (1 test)
cargo nextest run -p ts2wasm-backend-wasm module_initializers_are_emitted_and_called_in_metadata_order: PASS (1 test)
cargo fmt --all --check: PASS
cargo nextest run -p ts2wasm-ir: PASS (18 tests)
cargo nextest run -p ts2wasm-backend-wasm: PASS (19 tests)
cargo nextest run -p ts2wasm-compiler: PASS (39 tests)
cargo nextest run -p ts2wasm-cli module: PASS (15 tests, 220 skipped)
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry.ts -o /tmp/ts2wasm-233-init-runtime-entry.wasm: PASS
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-alias.ts -o /tmp/ts2wasm-233-init-runtime-alias.wasm: PASS
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-shadow.ts -o /tmp/ts2wasm-233-init-runtime-shadow.wasm: PASS
scripts/manager check-issue-health: PASS
scripts/manager check-agent-state: PASS
```

2026-04-28 child worker `233-module-lowered-ir-20260428T103829Z` added a narrow explicit lowered-module population slice:

- The compiler build path now attaches source-backed literal `export const` declarations from reachable local static modules into `LoweredProgram.modules` as `LoweredStmt::Export` statements, keyed by the issue-232 module graph IDs.
- Added compiler regression coverage proving `import { value } from "./source"` plus `export const value = 1` produces explicit lowered module metadata for module ID 1, validates as lowered IR, and selects `$module_exports_set` without selecting `$module_require`.
- Hardened `validate_lowered` to validate each module's explicit lowered statements using the module's `locals_count`, so future module metadata is checked before backend emission.
- Preserved the temporary static named import build rewrite and existing `static-entry.ts`, alias, and shadow build smokes. No dependency-order runtime execution, import reads from emitted module exports, live bindings, or runtime semantic parity claims were added; issue 233 remains open.

Validation:

```text
cargo nextest run -p ts2wasm-compiler static_module_export_lowering_populates_explicit_lowered_module_statements: PASS (1 test)
cargo fmt --all --check: PASS
cargo nextest run -p ts2wasm-ir: PASS (16 tests)
cargo nextest run -p ts2wasm-compiler: PASS (38 tests)
cargo nextest run -p ts2wasm-backend-wasm: PASS (18 tests)
cargo nextest run -p ts2wasm-cli module: PASS (15 tests, 219 skipped)
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry.ts -o /tmp/ts2wasm-233-lowered-entry.wasm: PASS
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-alias.ts -o /tmp/ts2wasm-233-lowered-alias.wasm: PASS
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-shadow.ts -o /tmp/ts2wasm-233-lowered-shadow.wasm: PASS
scripts/manager check-issue-health: PASS
scripts/manager check-agent-state: PASS
```
