# Cycle Report: 233 module lowered IR

Run ID: `233-module-lowered-ir-20260428T103829Z`
Issue: `issues/open/233-emit-static-es-module-bindings.md`
Branch: `agent/233-module-lowered-ir-20260428T103829Z`
Outcome: PROGRESS

## Scope

Implemented the assigned narrow progress slice for explicit lowered module IR. This does not close issue 233.

## Changes

- Added compiler build-path population of `LoweredProgram.modules` for reachable local source modules that contain literal `export const` declarations.
- Represented those source exports as explicit `LoweredStmt::Export` statements keyed by the issue-232 module graph IDs.
- Hardened `validate_lowered` so module statements are validated with each module's `locals_count`.
- Added a compiler regression proving the explicit module export metadata is populated, validates, remains buildable as WAT, selects `$module_exports_set`, and does not select `$module_require`.

## Acceptance Evidence

- Simple named export/import programs still build to WASM for the existing static entry, alias, and shadow fixtures.
- Imported values continue to use the existing temporary graph-backed static named import rewrite; no runtime semantic parity claim was added.
- The new explicit lowered module metadata carries source-side literal export statements and selects the runtime export helper through the link plan.
- Runtime link-plan tests from the previous slice remain green through `cargo nextest run -p ts2wasm-backend-wasm`.
- Existing CommonJS module-cache/module tests remain green through `cargo nextest run -p ts2wasm-cli module`.

## Validation

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

## Remaining Work

- Lower named imports to reads from emitted module exports instead of the temporary build rewrite.
- Emit dependency-order module initialization and once-only runtime execution.
- Add runtime execution/differential coverage under issue 234 before claiming semantic parity.
- Issue 233 remains open.
