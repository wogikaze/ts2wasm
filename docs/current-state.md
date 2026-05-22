# Current State

Last audited: 2026-05-21.

This page records what is true now. Final-state design contracts live in the other `docs/` files. Do not turn this page into a roadmap or issue backlog.

## Current Gate

Use the manager entrypoints first; `mise run <task>` aliases are acceptable when available.

```bash
python scripts/manager.py check
python scripts/manager.py gate-fast
python scripts/manager.py nextest
python scripts/manager.py coverage-report --format markdown
```

Generated coverage artifacts must name their generator command. Do not hand-edit generated tables to make a gate look fresh.

## Repository Shape

- Rust workspace crates: 15 (`backend-core, backend-wasm, cli, compiler, diagnostic, frontend, ir, iwasm-runner, resolve, runtime-abi, runtime-catalog, semantics, shared, source, syntax`).
- Runtime catalog variants detected from source: approximately 490 `RuntimeFn` entries, including pseudo-intrinsics.
- Fixture catalog directories: 36.
- Issues present: 466 Markdown files under `issues/`.

## Issue Inventory Snapshot

This is an inventory snapshot, not the issue source of truth.

| Status | Count |
|---|---:|
| `done` | 402 |
| `open` | 62 |
| `doing` | 2 |

| Priority | Count |
|---|---:|
| `P2` | 212 |
| `P1` | 189 |
| `P0` | 43 |
| `P3` | 22 |

## Fixture Snapshot

| Fixture directory status | Count |
|---|---:|
| `pass` | 29 |
| `unknown` | 7 |

| Feature matrix status | Count |
|---|---:|
| `pass` | 29 |
| `partial` | 5 |

Build success is not semantic evidence. A feature is semantically supported only when a fixture, differential run, or reference record proves it.

## Implemented Execution Targets

- `wasm32-wasi-p1` through aliases `wasm32-wasi`, `wasm32-wasi-p1`.
- `wasm32-wasi-p1+node-shim` through aliases `wasm32-wasi+node-host`, `wasm32-wasi-p1+node-host`, `wasm32-wasi-p1+node-shim`.

Known but unimplemented targets: `wasm32-wasi-gc`, `wasm32-component`.

## Current Implementation Notes

- The default compiler path still lowers through legacy `LoweredProgram`, but build writes backend-provided wasm bytes and no longer uses a `wat2wasm` CLI fallback.
- `backend-wasm` has an explicit native `LoweredProgram -> WasmModule -> wasm bytes` subset API for simple numeric/string console output, locals, arithmetic, `if`, `while`, and direct user-function calls. It is not yet the default build backend and does not cover the full runtime helper catalog.
- `--experimental-hir-mir` and `--experimental-hir-mir-compat-fallback` are migration modes and must not be described as default parity.
- Static named ES module import/export has a narrow local differential slice; broader module semantics are not complete.
- Class, Node API, and broader module fixtures include build-smoke coverage that must not be reported as semantic parity.
- TypeScript syntax support is a mix of erase, preserve, lower, and reject; parser acceptance alone does not imply runtime support.
- Reference coverage artifacts may be sampled; always inspect the evidence command before quoting numbers.

## Next Useful Docs

- Compiler pipeline: `docs/04-compiler-architecture-and-runtime.md`
- IR boundaries: `docs/13-ir-contracts.md`
- Runtime ABI: `docs/14-runtime-abi.md`
- Test status schema: `docs/11-shared-definitions.md`
- Coverage matrix: `docs/15-coverage-matrix.md`
- Fixture matrix: `docs/26-semantic-feature-matrix.md`
