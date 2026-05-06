---
id: 5228
title: "W0: wasm binary backend — primary emission path"
type: feature
area: backend
class: design-ready
priority: P1
depends_on: [5225]
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Replace the WAT-dominant emission pipeline with direct wasm binary emission as the primary path. `binary_mvp.rs` exists as a hello-world MVP; this issue extends it to cover the entire lowered program. The typed WAT writer (W0-001/5225) becomes the fallback for debugging (`--emit-wat`).

## Problem

Current-state.md: "full wasm backend は未実装（現状は WAT 中心）"

The pipeline is:
```
LoweredProgram → emit WAT string → wat2wasm subprocess → .wasm binary
```

This creates:
- Build-time dependency on `wat2wasm` CLI tool
- Slow compilation: WAT → wasm binary requires an extra process invocation
- Opaque debugging: WAT output and actual wasm binary can diverge
- Limited optimization: no way to apply wasm-level optimizations without round-tripping through text format

Problem: The WAT→wasm subprocess pipeline is fragile, slow, and adds an external toolchain dependency for every build.

## Desired final state

The primary emission path is direct wasm binary output:
```
LoweredProgram → RuntimeLinkPlan → wasm binary writer → .wasm binary
```

The WAT path becomes the `--emit-wat` debug-only fallback, using the typed `WatWriter` (from W0-001/5225) for those who need human-readable output.

The binary writer covers the full lowered program:
- Type section (function signatures)
- Import section (WASI imports, host imports)
- Function section (type indices)
- Memory section (initial/max pages)
- Export section (_start, memory)
- Code section (function bodies with local declarations)
- Data section (interned strings, runtime strings)

## Scope

In scope:

- [ ] Extend `binary_mvp.rs` from hello-world MVP to full lowered-program emission
- [ ] Cover all `LoweredStmt` and `LoweredExpr` variants in binary code emission
- [ ] Support WASI import emission
- [ ] Support data segment emission (interned strings)
- [ ] Support memory section (initial/max from Layout constants)
- [ ] Validate output with `wasm-tools validate` in CI
- [ ] Keep `--emit-wat` as debug fallback using typed WatWriter
- [ ] `docs/04-compiler-architecture-and-runtime.md` update to document wasm binary flow
- [ ] `current-state.md` update

Out of scope:

- `wasm-encoder` crate dependency evaluation (may use custom encoder or wasm-encoder)
- Wasm GC/Component Model binary targets
- Binary size optimization
- Streaming or multi-module compilation

## Affected paths

Expected:

- `crates/backend-wasm/src/binary_mvp.rs` — extend to full lowering coverage
- `crates/backend-wasm/src/emitter.rs` — restructure to support dual WAT/binary path
- `crates/backend-wasm/src/lib.rs` — pipeline selection
- `crates/compiler/src/` — driver integration
- `docs/04-compiler-architecture-and-runtime.md`
- `current-state.md`

Do not touch:

- `crates/frontend/`, `crates/ir/`, `crates/runtime-abi/`
- Any WAT runtime helper templates (runtime_*.rs)
- Test fixtures

## Acceptance criteria

- [ ] All build-smoke fixtures produce valid `.wasm` via direct binary emission
- [ ] `wasm-tools validate` passes on all generated `.wasm` files
- [ ] Differential tests (Node/iwasm) pass with binary-emitted output (no wat2wasm subprocess)
- [ ] `--emit-wat` still produces human-readable WAT output
- [ ] `cargo test` and `cargo nextest run` all pass

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
# Verify wasm-tools validation passes
wasm-tools validate target/debug/*.wasm  || echo "check implement"
# Verify differential tests still pass
cargo nextest run --test differential_jsonl
```

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected
- [ ] updated: `docs/04-compiler-architecture-and-runtime.md`

Current state:

- [ ] not affected
- [ ] updated: `current-state.md`

Follow-up issues:

- [ ] none
- [ ] created/updated: none
