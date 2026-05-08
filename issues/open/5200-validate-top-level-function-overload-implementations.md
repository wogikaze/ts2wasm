---
id: 5200
title: "Validate top-level function overload implementations"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-08
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #5200.

## Summary

Handle bodyless top-level TypeScript function overload signatures together with
their implementation declaration, instead of treating the implementation as a
duplicate function definition.

## Problem

`callOverloads1.ts` and `callOverloads2.ts` parse successfully, including
bodyless function declarations, but `validate_ast` stops at a
`DuplicateFunction` diagnostic for `F1`. In TypeScript, a bodyless overload
signature immediately followed by one implementation is valid, while multiple
function bodies produce specific implementation diagnostics.

Additional representative `callbackArgsDifferByOptionality.ts` contains two
bodyless overload signatures for `x3` followed by one implementation with a
function-typed callback parameter. It currently stops at the same
`DuplicateFunction` boundary before reaching TypeScript's real unresolved `cb`
diagnostic.

Problem: top-level function overload implementation groups are currently classified as duplicate concrete functions.

## Current failure

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOverloads1.ts
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOverloads2.ts
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callbackArgsDifferByOptionality.ts
```

Current diagnostics:

```text
callOverloads1.ts: error: [DuplicateFunction] duplicate function definition: `F1` at 257..287
callOverloads2.ts: error: [DuplicateFunction] duplicate function definition: `F1` at 283..313
callbackArgsDifferByOptionality.ts: error: [DuplicateFunction] duplicate function definition: `x3` at 85..93
```

Representative sources:

```ts
function F1(s:string);
function F1(a:any) { return a; }
```

```ts
function F1(s:string) { return s; } // error
function F1(a:any) { return a; } // error
function Goo(s:string); // error - no implementation
```

```ts
function x3(callback: (x?: 'hi') => number);
function x3(callback: (x: string) => number);
function x3(callback: (x: any) => number) {
    cb();
}
```

Triage evidence:

- Tokens and AST succeed for both reference files.
- `callOverloads1.ts` AST contains bodyless `Function F1(s)` followed by
  implemented `Function F1(a)`.
- `callOverloads2.ts` AST contains two implemented `Function F1` declarations
  and a bodyless `Function Goo(s)` with no implementation.
- `callbackArgsDifferByOptionality.ts` AST contains two bodyless `Function x3`
  overload signatures followed by one implemented `Function x3`.
- TypeScript oracle reports no `F1` diagnostic for the valid overload group in
  `callOverloads1.ts`.
- TypeScript oracle reports TS2389/TS2393 for the invalid `F1` implementations
  and TS2391 for missing `Goo` implementation in `callOverloads2.ts`.
- TypeScript oracle reports only TS2304 for unresolved `cb` in
  `callbackArgsDifferByOptionality.ts`, proving the overload group itself is
  accepted.

## Desired final state

Top-level function declarations are grouped by overload signature and
implementation shape before duplicate-function validation. Valid overload
signature plus implementation groups are accepted, while invalid groups report
narrower source-spanned diagnostics for duplicate implementations, wrong
implementation names, or missing implementations.

## Scope

In scope:

- [x] Distinguish bodyless top-level function overload signatures from
  implemented function declarations
- [x] Accept a bodyless overload signature immediately followed by one
  implementation for the same name
- [x] Accept multiple bodyless overload signatures immediately followed by one
  implementation for the same name
- [x] Preserve/report duplicate implementation diagnostics for multiple
  function bodies with the same name
- [x] Report a source-spanned missing-implementation diagnostic for a bodyless
  overload declaration with no following implementation

Out of scope:

- Class/function merge diagnostics, tracked by `issues/open/5199-report-function-overload-list-class-merge-diagnostics.md`
- Ambient declaration merging
- Full overload compatibility checking between signatures and implementation
  body types

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/backend-wasm/`
- unrelated runtime call semantics

## Acceptance criteria

- [x] `callOverloads1.ts` no longer reports `DuplicateFunction` for the valid
  `F1` overload signature plus implementation
- [x] `callOverloads2.ts` reports a narrower duplicate implementation
  diagnostic for the two implemented `F1` declarations
- [x] `callbackArgsDifferByOptionality.ts` no longer reports
  `DuplicateFunction` for the valid `x3` overload signatures plus
  implementation
- [x] `contextualTypingOfLambdaReturnExpression.ts` no longer reports
  `DuplicateLocal` for the valid `callb` overload signatures plus
  implementation
- [x] A focused fixture covers one bodyless overload signature followed by one
  implementation for the same name
- [x] A focused fixture covers two bodyless overload signatures followed by one
  implementation, including a function-typed callback parameter with an
  optional string-literal parameter
- [x] A focused fixture covers two implemented function declarations with the
  same name and preserves a duplicate implementation diagnostic
- [x] A focused fixture covers one bodyless overload signature with no
  implementation and reports a missing implementation diagnostic

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(function)'
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOverloads1.ts
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOverloads2.ts
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callbackArgsDifferByOptionality.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingOfLambdaReturnExpression.ts
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

This issue only handles top-level function overload implementation grouping.
The `Foo` class/function merge errors in the same reference files are owned by
issue 5199.

2026-05-07 additional superseded bucket:

- `issues/open/1527-implement-contextualTypingOfLambdaReturnExpression.md`
  reaches the same top-level overload grouping boundary. The source has two
  bodyless `callb(lam: ...)` overload signatures followed by one implementation
  `function callb(a) { }`; resolver reports
  `DuplicateLocal: duplicate local variable: callb at 82..90` before TypeScript's
  intended lambda contextual member diagnostics on `a.length`.

2026-05-08 additional superseded bucket:

- `issues/open/3539-implement-noImplicitAnyFunctions.md` reaches the same
  top-level overload grouping boundary. The source has two bodyless
  `function f6(...)` overload signatures followed by one implementation
  `function f6(x: string, y) { return null; }`; validation reports
  `DuplicateFunction: duplicate function definition: f6 at 254..262`, while the
  TypeScript oracle reports no diagnostics.
- `issues/open/3553-implement-noImplicitAnyWithOverloads.md` reaches the same
  top-level overload grouping boundary. The source has two bodyless
  `function callb(lam: ...)` overload signatures followed by one implementation
  `function callb(a) { }`; validation reports `DuplicateFunction` for the
  second overload signature before the contextual lambda member diagnostics can
  be observed.

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
