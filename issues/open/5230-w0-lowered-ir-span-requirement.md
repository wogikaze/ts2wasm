---
id: 5230
title: "W0: mandatory span on LoweredExpr and LoweredStmt variants"
type: refactor
area: ir
class: design-ready
priority: P1
depends_on:
  - 5226
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Add `span: Span` to all 37 `LoweredExpr` variants and all 18 `LoweredStmt` variants.
Propagate source spans from the AST through the lowering pipeline.
This is the second slice of 5226 (which added SourceDiagnostic/InternalDiagnostic types).

## Problem

`LoweredExpr` (37 variants) and `LoweredStmt` (18 variants) have zero span fields.
This means any diagnostic emitted from the lowered IR layer cannot report a source location.
103 out of 172 total `span: None` sites come from `crates/ir/src/` — concentrated in these span-less IR phases.

## Desired final state

- Every `LoweredExpr` variant carries a `span: Span` field.
- Every `LoweredStmt` variant carries a `span: Span` field.
- All construction sites in resolver_expr.rs, resolver_extra.rs, resolver.rs, program.rs, program_builtins.rs pass the source span.
- All match sites in validate.rs, expr_emit.rs, emitter.rs account for the new field.
- Zero `span: None` source diagnostics from lowered IR.

## Scope

In scope:

- [x] Add `span: Span` to all 37 `LoweredExpr` tuple/struct variants
- [x] Add `span: Span` to all 18 `LoweredStmt` tuple/struct variants
- [x] Update resolver_expr.rs (~100 construction sites)
- [x] Update resolver_extra.rs (~50 construction sites)
- [x] Update resolver.rs (~20 construction sites)
- [x] Update program.rs (~20 construction sites)
- [x] Update program_builtins.rs (~10 construction sites)
- [x] Update validate.rs match patterns (~30)
- [x] Update expr_emit.rs match patterns (~80)
- [x] Update emitter.rs match patterns (~30)
- [x] Update any other LoweredExpr/LoweredStmt match sites
- [x] `cargo test` and `cargo nextest run` all pass

Out of scope:

- HIR span requirement (separate slice)
- ResolvedExpr/ResolvedStmt span (can be done via AST span propagation)
- Span accuracy improvements
- Performance optimization of span storage

## Affected paths

Expected:

- `crates/ir/src/lowered/types.rs` — add span fields
- `crates/ir/src/lowered/resolver_expr.rs` — propagate spans
- `crates/ir/src/lowered/resolver_extra.rs` — propagate spans
- `crates/ir/src/lowered/resolver.rs` — propagate spans
- `crates/ir/src/lowered/program.rs` — propagate spans
- `crates/ir/src/lowered/program_builtins.rs` — propagate spans
- `crates/ir/src/lowered/validate.rs` — match on span field
- `crates/backend-wasm/src/expr_emit.rs` — match on span field
- `crates/backend-wasm/src/emitter.rs` — match on span field

Do not touch:

- `crates/frontend/` (AST already has spans)
- `crates/runtime-abi/`
- Test fixtures or coverage data


## False-done audit

Date: 2026-05-07

Classification: truly-done.

Audit result: retained in issues/done/. Implementation commits confirmed.
