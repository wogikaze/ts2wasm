---
id: 5250
title: "Parse class declarations in nested block statements"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Parse `class` declarations that appear inside nested block statements in
function bodies.

## Problem

Problem: `classDeclarationBlockScoping2.ts` reports `UnsupportedSyntax: expected Comma, got Some(Ident("C"))` at a nested `{ class C {} ... }` block.

The token stream is correct, but AST construction fails when statement parsing
enters the nested block containing `class C {}`.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classDeclarationBlockScoping2.ts
```

Current diagnostic:

```text
UnsupportedSyntax: expected Comma, got Some(Ident("C")) at 90..91
```

## Scope

In scope:

- [ ] Dispatch `class` declarations correctly inside nested block statement parsing.
- [ ] Preserve following statements in the same nested block, such as `var c2 = C;`.
- [ ] Add focused parser/CLI coverage for `function f(){ { class C {} var c2 = C; } }`.

Out of scope:

- Name-resolution semantics for block-local class declarations, tracked by issue 5249.
- Class expression lowering.

## Affected paths

Expected: `crates/frontend/src/`, `crates/cli/tests/`, `fixtures/`.

Do not touch: backend/runtime ABI.

## Acceptance criteria

- [ ] `classDeclarationBlockScoping2.ts` advances past `expected Comma, got Some(Ident("C"))`.
- [ ] A focused test covers class declarations inside nested block statements.
- [ ] Existing object literal/block parsing tests still pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-cli parser
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classDeclarationBlockScoping2.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classDeclarationBlockScoping2.ts --detail --no-dashboard-data
```

## Notes

Split from `issues/done/1175-implement-classDeclarationBlockScoping.md`.
