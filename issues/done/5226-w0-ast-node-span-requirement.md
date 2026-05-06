---
id: 5226
title: "W0: mandatory span on all AST nodes and source diagnostics"
type: refactor
area: frontend
class: design-ready
priority: P0
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Make `Span` mandatory on all source-derived AST/IR nodes and replace `Option<Span>` with `Span` in diagnostic paths for source-originating errors. `docs/12-coding-standard.md §4` requires all source-derived nodes to have a `Span`, but `Diagnostic.span` is still `Option<Span>`, and some AST nodes lack `span` fields.

## Problem

`docs/12-coding-standard.md §1.8` identifies missing spans as a root cause:

```
SpannedToken はあるが Expr / Stmt が span を持たない
lowering / validation diagnostic が span: None になる
```

Current state:
- `Diagnostic.span` is `Option<Span>` — allows `span: None` on source-originating errors
- 171 `span: None` instances exist across the codebase (valid InvariantViolation paths and possibly some source diagnostics)
- Not all `Expr`/`Stmt` variants carry a `span` field — depends on parser wave progress
- `docs/12-coding-standard.md` already requires `Span` on `Token`, `Stmt`, `Expr`, `HIR node`, `BuiltinCall`

Problem: Source-originating diagnostics without `span` make error location impossible, degrading user experience.

## Desired final state

- `Diagnostic` for source errors requires `Span` (not `Option<Span>`). A separate internal-only `InternalDiagnostic` type with `Option<Span>` exists for `InvariantViolation` and `BackendIo`.
- All `Expr`/`Stmt` variants in the frontend AST carry a `span: Span` field.
- All `HIR`/`LoweredExpr`/`LoweredStmt` variants carry a `span: Span` field.
- Zero source-originating diagnostics use `span: None`.
- `docs/12-coding-standard.md §4` checkout passes: no new `span: None` on source diagnostics.

## Scope

In scope:

- [x] Split `Diagnostic` into `SourceDiagnostic` (mandatory `Span`) and `InternalDiagnostic` (invariant violations, `Option<Span>`)
- [x] Audit all 171 `span: None` sites: classify as source-error (must get span) or invariant-violation (keep None)
- [x] Add `span` field to AST `Expr`/`Stmt` variants missing it
- [x] Add `span` field to `LoweredExpr`/`LoweredStmt` variants missing it
- [x] Update `docs/12-coding-standard.md` and `current-state.md`

Out of scope:

- HIR span requirement (separate slice if HIR diverges)
- Span accuracy improvements (just make it present)
- Performance optimization of span storage

## Affected paths

Expected:

- `crates/frontend/src/diagnostic.rs` — split `Diagnostic` type
- `crates/frontend/src/parser/` — add spans to AST nodes
- `crates/ir/src/lowered/` — add spans to lowered nodes
- `crates/compiler/src/` — adapt to new diagnostic types
- `crates/backend-wasm/src/` — adapt to new diagnostic types
- `docs/12-coding-standard.md`
- `current-state.md`

Do not touch:

- Runtime behavior or ABI
- Test fixtures or coverage data
- `crates/runtime-abi/`

## Acceptance criteria

- [x] `SourceDiagnostic` and `InternalDiagnostic` types exist; compiler pipeline uses `SourceDiagnostic` for user-facing errors
- [x] All 171 `span: None` sites are audited and classified; zero source-originating diagnostics remain with `span: None`
- [x] AST `Expr`/`Stmt` variants all carry `span: Span`
- [x] `LoweredExpr`/`LoweredStmt` variants all carry `span: Span`
- [x] `cargo test` and `cargo nextest run` all pass
- [x] `docs/12-coding-standard.md §19.9` (Diagnostic/Span gate) references new types

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
# Audit: zero new source diagnostic span: None
rg "span: None" crates/ --type rust | grep -v InvariantViolation
```

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected
- [x] updated: `docs/12-coding-standard.md §4` and `§19.9`

Current state:

- [x] not affected
- [x] updated: `current-state.md`

Follow-up issues:

- [x] none
- [x] created/updated: none

## Notes

The `SourceDiagnostic`/`InternalDiagnostic` split is the minimal change. Do NOT refactor all diagnostic call sites in one commit — introduce both types, migrate compiler phases one by one, keeping a validation pass that catches new `span: None` on source diagnostics.
