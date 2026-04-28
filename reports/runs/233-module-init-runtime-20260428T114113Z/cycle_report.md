# Cycle Report: 233-module-init-runtime-20260428T114113Z

## Outcome

Status: PROGRESS

Issue 233 remains open. This slice wires explicit lowered module metadata into dependency-ordered backend initialization, but it does not claim full ES module execution parity, live bindings, cycle semantics, or issue 234 runtime differential coverage.

## Changes

- Ordered compiler-populated `LoweredProgram.modules` with `ModuleGraph::dependency_first_initialization_steps()`, excluding the entry module.
- Collected strings from lowered module statements in the WAT backend.
- Emitted `$module_init_<id>` helpers for modules with explicit lowered statements and called them before top-level import reads.
- Added compiler and backend regression coverage for dependency-first metadata order and backend initializer call order.
- Preserved existing static module build smokes for entry, alias, and shadow fixtures.

## Validation

```text
cargo nextest run -p ts2wasm-backend-wasm explicit_module_export_statement_selects_es_module_export_helpers: PASS
cargo nextest run -p ts2wasm-compiler static_module_export_lowering_orders_module_metadata_dependency_first: PASS
cargo nextest run -p ts2wasm-backend-wasm module_initializers_are_emitted_and_called_in_metadata_order: PASS
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

## Acceptance Evidence

- Simple named export/import programs still build to WASM: verified with `static-entry.ts`, alias, and shadow build commands.
- Imported values remain lowered as reads from `ModuleLoad { module_id }` plus `PropertyGet`, not lexical importer globals: preserved by existing compiler coverage and backend initializer test.
- Dependency-first initialization progressed: compiler metadata now follows dependency-first graph order and backend calls module init helpers in metadata order.
- Runtime helpers remain selected through the runtime/link-plan catalog: backend tests still prove non-module IR avoids module helpers and explicit module exports select `ModuleExportsSet`.

## Remaining Work

- Full module initialization once-only semantics still need broader runtime execution coverage.
- Live binding updates, cyclic execution semantics, and semantic parity remain out of scope here and tracked by issues 233/234.

## Reporting

Discord reporting was attempted after local validation and commit preparation. If unavailable, payload/error artifacts are stored in this run directory.
