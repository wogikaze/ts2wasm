# Cycle Report: 233 Module Init Once

Run id: `233-module-init-once-20260428T100229Z`
Branch: `agent/233-module-init-once-20260428T100229Z`
Issue: `issues/open/233-emit-static-es-module-bindings.md`
Progress commit: `a765a7b3164f`
Outcome: PROGRESS

## Scope

Implemented the assignment's preferred narrow source-backed initialization representation slice. Issue 233 remains open because backend runtime initialization, export storage, live bindings, and execution parity are still not implemented.

## Changes

- Added `ModuleInitializationStep`.
- Added `ModuleGraph::dependency_first_initialization_steps()` to derive dependency-first initialization steps from the resolved issue-232 module graph.
- Deduplicated direct dependency module IDs so repeated imports of the same source module contribute one initialization dependency.
- Re-exported `ModuleInitializationStep` from `ts2wasm-compiler`.
- Added compiler regression coverage using real local files: entry imports the same source module twice, source imports a nested module, and the resulting step order is nested -> source -> entry.
- Recorded progress evidence in issue 233 and updated `current-state.md` with the new compiler API fact.

## Acceptance Progress

- Simple named export/import programs still build to WASM: preserved existing build smokes for `static-entry.ts`, `static-entry-alias.ts`, and `static-entry-shadow.ts`.
- Imported values are read from the resolved source module: unchanged from previous source-backed literal rewrite slice; preserved by compiler and CLI coverage.
- Module initialization runs once per module for repeated imports: added compiler-level representation coverage only. Runtime execution is not claimed.
- Runtime link plan includes module helpers only for programs that use modules: unchanged; backend package validation still passes.
- Existing CommonJS module-cache fixtures still build: covered by `cargo nextest run -p ts2wasm-cli module`.

## Validation

```text
cargo nextest run -p ts2wasm-compiler builds_dependency_first_once_only_initialization_steps_from_static_graph
result: FAIL before final patch; assertion used an untyped empty slice and failed to compile with E0282/E0283.

cargo nextest run -p ts2wasm-compiler builds_dependency_first_once_only_initialization_steps_from_static_graph
result: PASS (1 test, 36 skipped)

cargo fmt --all --check
result: PASS

cargo nextest run -p ts2wasm-compiler
result: PASS (37 tests)

cargo nextest run -p ts2wasm-ir
result: PASS (16 tests)

cargo nextest run -p ts2wasm-backend-wasm
result: PASS (16 tests)

cargo nextest run -p ts2wasm-cli module
result: PASS (15 tests, 219 skipped)

cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry.ts -o /tmp/ts2wasm-233-init-entry.wasm
result: PASS

cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-alias.ts -o /tmp/ts2wasm-233-init-alias.wasm
result: PASS

cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-shadow.ts -o /tmp/ts2wasm-233-init-shadow.wasm
result: PASS

scripts/manager check-issue-health
result: PASS

scripts/manager check-agent-state
result: PASS
```

## Remaining Work

- Lower named exports/imports into explicit resolved/lowered module binding IR instead of the temporary build rewrite.
- Emit module initialization and once guards in backend/runtime code using the graph-derived order.
- Add export storage reads for named imports.
- Add issue-234 runtime/differential execution coverage before making semantic parity claims.
