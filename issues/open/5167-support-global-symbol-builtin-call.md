---
id: 5167
title: "Support global Symbol builtin call"
type: feature
area: ir/builtin-resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-08
---

## Summary

`bigintIndex.ts` parses and reaches `resolve_builtins`/`lower_program`, but stops at `key = Symbol();` with `UnresolvedFunction: unresolved function: Symbol`. This hides the reference's intended BigInt index diagnostics.

## Problem

Problem: `reference/typescript/tests/cases/compiler/bigintIndex.ts` currently reports `UnresolvedFunction: unresolved function: Symbol` before it can validate BigInt-as-index behavior.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigintIndex.ts
```

Current compiler diagnostic:

```text
UnresolvedFunction: unresolved function: `Symbol`
```

Representative source:

```ts
let key: keyof any;
key = 123;
key = "abc";
key = Symbol();
key = 123n;
```

Compiler evidence:

- Tokens and AST are successful for the representative file.
- Name resolution reaches visible bindings `arr`, `num`, `key`, `bigNum`, and `typedArray`.
- The pipeline fails at `resolve_builtins` / `lower_program` because `Symbol` is treated as an unresolved function call.

TypeScript oracle evidence from the populated reference workspace:

```text
key: string | number | symbol
TS reports BigInt index/type diagnostics, including "Type 'bigint' cannot be used as an index type."
```

## Desired final state

The global `Symbol()` call is recognized as a builtin boundary so `bigintIndex.ts` no longer stops at `unresolved function: Symbol`. If full Symbol runtime values are still outside the current runtime subset, the compiler should emit a source-spanned, issue-linked Symbol diagnostic after builtin resolution instead of an unresolved-function error.

## Scope

In scope:

- [x] Recognize global `Symbol()` in builtin resolution/lowering.
- [x] Preserve existing bare `Symbol` identifier handling where it is used as a namespace/global value.
- [x] Add focused resolver coverage for `let key; key = Symbol();`.
- [x] Re-run `bigintIndex.ts` triage and confirm the `UnresolvedFunction` blocker is gone.

Out of scope:

- Full ECMAScript Symbol identity, registry, and property-key semantics.
- Full TypeScript `keyof any` type checking.
- BigInt-as-index TypeScript diagnostics after this blocker advances the pipeline.

## Affected paths

Expected:

- `crates/ir/src/builtin_resolver.rs`
- `crates/ir/src/lowered/program_builtins.rs`
- `crates/ir/src/lowered/resolver_expr.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/cli/tests/`

Do not touch:

- BigInt arithmetic/runtime representation unless the triage advances past Symbol and proves that is the next blocker.
- Broad type-checker diagnostics unrelated to global `Symbol()`.

## Acceptance criteria

- [x] `key = Symbol();` no longer reports `UnresolvedFunction`.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigintIndex.ts` no longer reports `unresolved function: Symbol`.
- [x] A focused resolver/lowering regression covers a global `Symbol()` call.
- [x] Any remaining unsupported Symbol semantics are source-spanned and issue-linked.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir
cargo nextest run -p ts2wasm-compiler
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigintIndex.ts
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from generated bucket `1051` on 2026-05-06. The current reference file also contains BigInt index/type-checker diagnostics; those should be triaged separately after this Symbol builtin-call blocker no longer masks them.

2026-05-08 fold-in:

- `issues/open/3574-implement-noUnusedLocals-parser-syntax.md` reaches the same
  unresolved global `Symbol()` blocker for `const x = Symbol("x")` in
  `noUnusedLocals_writeOnlyProperty_dynamicNames.ts`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in issues/open/. Implementation commits confirmed.
Future-work tracking: none identified.
