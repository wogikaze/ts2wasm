# Architecture Improvement — Requirements

**Run ID**: `arch-design-review-2026-05-11`
**Date**: 2026-05-11

## Goal

Reduce coupling and improve LLM-context-friendliness of the ts2wasm codebase by enforcing mechanical boundaries, splitting oversized files/modules, and replacing String-typed runtime references with typed alternatives.

## Non-Goals

- Complete crate reorganization (new crate trees)
- HIR/MIR full separation (will be scoped to specific improvements)
- Full WAT encoder replacement

## Scope (8 items, priority order)

### P1: Extract Span/DiagCode/Diagnostic from frontend into shared
- Move `Span`, `DiagCode`, `Diagnostic` to `crates/shared`
- Update all import paths
- Remove `frontend` dependency from `backend-wasm` and `ir`

### P2: Introduce `Validated<LoweredProgram>` wrapper
- Struct that wraps `LoweredProgram`, validates on construction
- Backend API accepts only `&Validated<LoweredProgram>`

### P3: Replace `RuntimeCall { runtime_fn: String }` with typed alternative
- Create `RuntimeIntrinsic` enum in IR layer
- Map `RuntimeIntrinsic → RuntimeFn` in backend

### P4: Domain-split `runtime_fn_impl.rs` spec tables
- Split `spec()` into domain-specific files: `core`, `array`, `object`, `string`, `bigint`, `host`

### P5: Domain-dispatch `runtime_builder.rs`
- Replace single giant match with domain-dispatch pattern

### P6: Extract pipeline stages from `compiler/src/lib.rs`
- Create `compile_source()` pure function
- Separate I/O from compilation logic

### P7: Decompose `Resolver` context
- Struct-group fields in Resolver (SymbolEnv, ClassEnv, StaticFacts, etc.)

### P8: Strengthen architecture checks
- Ban new files > 2000 lines
- Ban functions > 300 lines
- Ban new `RuntimeCall` string usage
- Ban new backend→frontend dependency
- Reduce line limit from 4100 → 3000

## Acceptance

Each P-item:
- [ ] Implementation committed
- [ ] `cargo fmt --all --check` passes
- [ ] `cargo nextest run` passes
- [ ] Architecture check passes with no regressions

## Evidence

- commit hashes per P-item
- Test results for full gate
