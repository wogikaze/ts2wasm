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
---

## Summary

Lower resolved ES module imports/exports into module initialization, export storage, and import reads in the existing WASM runtime model.

## Problem

The backend has CommonJS-oriented module cache helpers, but static ES module bindings are not represented through lowering and emission. Importing code needs to read exported bindings without weakening the existing runtime/capability boundaries.

## Desired final state

Named static exports are emitted into a module export object and named imports read those bindings through the resolved module graph. Modules initialize once in dependency order.

## Scope

In scope:

- [ ] Lower named exports from declarations and export lists into resolved IR
- [ ] Lower named imports to reads from resolved module exports
- [ ] Emit module initialization in dependency order using the module graph IDs
- [ ] Reuse or extend existing module cache/export runtime helpers without unconditional host imports
- [ ] Add backend/link-plan tests showing ES module helpers are included only when needed

Out of scope:

- [ ] Live binding updates beyond simple top-level values
- [ ] `export default`, namespace objects, package resolution, and dynamic import
- [ ] Broad module semantic parity claims without execution fixtures

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

- [ ] Simple named export/import programs build to WASM
- [ ] Imported values are read from the resolved source module, not from lexical globals in the importer
- [ ] Module initialization runs once per module for repeated imports
- [ ] Runtime link plan includes module helpers only for programs that use modules
- [ ] Existing CommonJS module-cache fixtures still build

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

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none

## Notes

Follow `docs/12-coding-standard.md`: runtime helpers, imports, capabilities, and manifest behavior must be derived through the runtime/link-plan catalog, not hard-coded in the emitter.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none

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
