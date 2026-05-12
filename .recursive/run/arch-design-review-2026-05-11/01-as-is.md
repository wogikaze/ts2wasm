# Architecture Design Review — As-Is Analysis

**Date**: 2026-05-11
**Source**: External architecture review of ts2wasm project

## Summary

This project's design philosophy is sound — especially `docs/12-coding-standard.md` and `docs/13-ir-contracts.md` which establish good principles (Parser = syntax only, Backend = validated IR only, RuntimeLinkPlan = single source of truth for imports/capabilities/runtime deps).

**Problem**: Those principles have not yet been mechanically enforced at the boundary level. The crate structure exists but the code within exhibits:

- Giant functions and enums
- Frontend type leakage into backend/IR
- String-based runtime calls in IR
- `include!` pseudo-splitting without real module boundaries
- Overly permissive line limits (4100)

## Current Coupling Points

| Symptom | Evidence | Risk |
|---------|----------|------|
| `backend-wasm` depends on `frontend` | `Cargo.toml:10`, `lib.rs:24` | Backend dragged by syntax-layer types and diagnostic types |
| `ir` depends on `frontend` | `Cargo.toml:11`, `types.rs:8-10` | IR not independent from parser/syntax representations |
| `compiler/src/lib.rs` holds driver + I/O + module rewrite + validation + emit | `lib.rs:84-167` | One change requires reading broad context |
| `Resolver` state is huge | `resolver.rs:5-50` | Array/class/module/capture/private field tightly coupled in one state object |
| `lower_expr` ~2700 lines | `resolver_expr.rs:4` | One feature addition requires context of all expression lowering |
| `RuntimeFn` ~288 variants | `runtime_fn.rs:43+` | Runtime catalog additions ripple through giant match / giant spec |
| `runtime_builder.rs` 300+ line dispatch | `runtime_builder.rs:5-319` | Low independence per runtime domain |
| `LoweredExpr::RuntimeCall` uses `String` | `types.rs:358-361` | Violates docs/13 "no string literals for runtime function names" |
| `include!` for physical splitting | `lowered.rs:1-4` | Files split but not module boundaries; no information hiding |
| Line limit 4100 too loose | `architecture-rules.py:20-21` | Permits LLM-unfriendly file sizes |

## Critical Finding

```rust
RuntimeCall {
    runtime_fn: String,  // ← dangerous
    args: Vec<LoweredExpr>,
    span: Span,
}
```

This is risky because the runtime uses `RuntimeFn` catalog + `RuntimeLinkPlan` as single source of truth, but IR-side runtime functions are `String`. This creates risk of typos, unregistered runtimes, capability leaks, and link plan gaps.

## File Size Hotspots (LOC)

| Function/File | Lines | Risk |
|---------------|-------|------|
| `lower_expr` | 2711 | Critical |
| `RuntimeFn::spec` | 2318 | Critical |
| `emit_json_parse` | 1357 | Critical |
| `emit_expr` | 921 | Critical |
| `Lexer::tokenize` | 863 | High |
| `lower_variable_array_callback_method` | 842 | High |
| `emit_statement_with_label` | 756 | High |

## Design Dimensions Required

This project needs **triple separation**, not just layer separation:

1. **Phase boundary** (when to decide): Parser → NameResolver → BuiltinResolver → HIR → MIR → RuntimeLinkPlan → Backend
2. **Semantic domain boundary** (what semantics): array, object, class, module, builtin/host API, async, string/regexp, number/bigint
3. **Capability boundary** (external capability required): WASI, filesystem, clock, random, Node host shim
