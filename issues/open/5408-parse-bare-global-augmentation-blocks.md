---
id: 5408
title: "Parse bare global augmentation blocks"
type: bug
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Treat TypeScript `global { ... }` augmentation blocks as TypeScript syntax
rather than runtime expression statements.

This is the one remaining executable slice split from
`issues/open/3318-implement-moduleAugmentationGlobal-import-export.md`.

## Problem

`moduleAugmentationGlobal6_1.ts` contains a bare `global { interface Array<T> { x } }`
block without `declare`. TypeScript reports this as an invalid global
augmentation, but ts2wasm currently parses only an expression statement for
`global` and later fails in name resolution with `UnresolvedName`.

Problem: bare TypeScript global augmentation syntax falls through to runtime
name resolution instead of producing a TypeScript syntax/ambient-global
diagnostic.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationGlobal6_1.ts
```

Current compiler evidence:

```text
tokens: ok; `global { interface Array<T> { x } }` is tokenized
ast: ok, but only as `Expr Ident("global")`
resolved: UnresolvedName unresolved name: `global`
TypeScript oracle: TS2669 and TS2670 for invalid global augmentation form
```

## Desired final state

The frontend recognizes bare `global { ... }` as TypeScript global
augmentation syntax and rejects or erases it at the TypeScript/ambient boundary.
The representative case should no longer reach runtime name resolution for a
synthetic `global` expression.

## Scope

In scope:

- [ ] Add parser or validator handling for top-level `global { ... }` blocks.
- [ ] Preserve existing `declare global { ... }` issue-400 behavior.
- [ ] Add a focused fixture or frontend/parser regression for bare `global {}`.
- [ ] Ensure ordinary runtime identifier expressions named `global` still resolve or fail through the normal resolver path outside block-augmentation syntax.

Out of scope:

- Full TypeScript checker parity for global augmentation legality.
- Runtime implementation of global augmentation declarations.
- Module graph/package resolution.
- Broader `declare module` semantics.

## Affected paths

Expected:

- `crates/frontend/src/`
- `fixtures/`
- focused frontend/compiler tests

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [ ] `moduleAugmentationGlobal6_1.ts` no longer reports `UnresolvedName: unresolved name: global`.
- [ ] The new diagnostic or erased boundary is TypeScript-specific and source-spanned at `global`.
- [ ] A focused regression covers `global { interface Array<T> { x } }`.
- [ ] A negative/adjacent regression proves `global;` or `let y = global;` is not reclassified as an augmentation block.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -p ts2wasm-compiler
env TS2WASM_BINARY=target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationGlobal6_1.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationGlobal6_1.ts --detail --no-dashboard-data
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

The parser already has a completed ambient declaration boundary in issue 400.
This issue is narrower: the bare keyword-like `global {}` form should not be
lowered as an expression statement before the resolver sees it.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
