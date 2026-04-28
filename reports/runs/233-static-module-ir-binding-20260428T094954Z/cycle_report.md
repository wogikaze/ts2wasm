# Cycle Report: 233 static module IR binding

Run id: `233-static-module-ir-binding-20260428T094954Z`
Branch: `agent/233-static-module-ir-binding-20260428T094954Z`
Issue: `issues/open/233-emit-static-es-module-bindings.md`
Implementation commit: `43c0de6bfa4b309399f39b43f1b7daefa19512ed`
Outcome: PROGRESS

## Scope

Implemented a narrow issue-233 progress slice. The temporary static named import build rewrite now flows through `lower_static_named_import_bindings_for_build`, which records explicit `StaticNamedImportBinding` entries before producing the current buildable local binding rewrite.

The binding record captures:

- source specifier
- resolved source module ID
- resolved source module path
- imported export name
- importer local binding name
- source initializer used by the current literal-only build slice

## Changes

- Added `StaticModuleBindingLowering` and `StaticNamedImportBinding` in `crates/compiler/src/lib.rs`.
- Kept `rewrite_static_named_imports_for_build` as the build pipeline entry point while moving the graph-backed binding decision into a clearly named lowering helper.
- Added compiler regression coverage proving an importer local named `value` does not supply the imported value when importing `value as importedValue`; the initializer is taken from the resolved source module export.
- Added `fixtures/module-system/static-entry-shadow.ts` and CLI module build-smoke coverage.
- Updated issue 233 progress evidence.

## Acceptance Evidence

- Simple named export/import programs still build to WASM: `static-entry.ts`, `static-entry-alias.ts`, and `static-entry-shadow.ts` all built successfully.
- Imported values source resolution: compiler regression asserts `importedValue` receives source module export `1` while importer local `value` remains `99`.
- Module initialization once semantics: not completed in this slice.
- Runtime link plan module helper gating: existing backend package test remains passing; no runtime helper behavior changed.
- Existing CommonJS module-cache fixtures: covered by full `scripts/manager nextest` and CLI module shard.

Issue 233 remains open because dependency-order initialization, once-only module execution, and broader module runtime semantics are not complete.

## Validation

```text
cargo nextest run -p ts2wasm-cli static_module_named_import_alias_build_smoke
PASS: 1 test passed

cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry.ts -o /tmp/ts2wasm-esm-233-ir-pre.wasm
PASS

cargo nextest run -p ts2wasm-compiler static_named_import_binding_lowering_uses_source_export_when_importer_shadows_name
PASS: 1 test passed

cargo nextest run -p ts2wasm-cli static_module_named_import_shadowed_local_build_smoke
PASS: 1 test passed

cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-shadow.ts -o /tmp/ts2wasm-esm-233-shadow-ir.wasm
PASS

cargo fmt --all --check
PASS

cargo nextest run -p ts2wasm-ir
PASS: 16 tests passed

cargo nextest run -p ts2wasm-backend-wasm
PASS: 16 tests passed

cargo nextest run -p ts2wasm-cli module
PASS: 15 tests passed, 219 skipped

cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry.ts -o /tmp/ts2wasm-esm-233-ir.wasm
PASS

cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-alias.ts -o /tmp/ts2wasm-esm-233-alias-ir.wasm
PASS

scripts/manager check-issue-health
PASS

scripts/manager check-agent-state
PASS

scripts/manager nextest
PASS: 369 tests passed, 4 skipped

scripts/manager check-repo-smoke
PASS

scripts/manager discord-report --run-id 233-static-module-ir-binding-20260428T094954Z
DEFERRED: DISCORD_WEBHOOK_URL is not configured; saved reports/runs/233-static-module-ir-binding-20260428T094954Z/discord-report-deferred.md
```

## Remaining Work

- Lower named exports/imports into durable resolved/lowered module binding IR instead of the current literal-only build rewrite.
- Emit dependency-order module initialization and once-only module execution.
- Extend runtime/link-plan coverage for ES module helper inclusion once module runtime helpers are introduced.
- Leave semantic execution parity to issue 234 after issue 233's dependency gate is actually complete.
