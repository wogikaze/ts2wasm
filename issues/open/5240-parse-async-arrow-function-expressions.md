---
id: 5240
title: "Parse async arrow function expressions"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Parse `async` arrow function expressions in expression position, including the
parenthesized async-IIFE shape `(async () => { ... })`.

## Problem

The lexer recognizes `async`, but expression parsing stops before building an
AST node for async arrow functions. Reference cases that should later exercise
`await`, async runtime, or type inference currently fail at the parser boundary.

Problem: `async () => { ... }` and `(async () => { ... })` are rejected as `unsupported expression: Async` before AST construction.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularInferredTypeOfVariable.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Async, span: Span { start: 44, end: 49 } }) at 50..51
```

Source context:

```ts
(async () => {
    function foo(p: string[]): string[] {
        return [];
    }

    let a1: string[] | undefined = [];

    while (true) {
        let a2 = foo(a1!);
        a1 = await bar(a2);
    }
});
```

Additional same-boundary evidence:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/asyncIIFE.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/asyncFunctionNoReturnType.ts
```

These cases also fail on `unsupported expression: ... Async` before the parser
builds an async arrow AST.

## Desired final state

The parser recognizes async arrow function expressions and parenthesized async
arrow expressions. If async runtime or `await` lowering remains unsupported,
the compiler reports a later, source-spanned diagnostic after the AST boundary
instead of a raw parser `Async` token error.

## Scope

In scope:

- [ ] Parse `async () => expr` and `async () => { ... }` as arrow functions
- [ ] Parse `(async () => { ... })` in expression position
- [ ] Preserve existing non-async arrow parsing and precedence behavior
- [ ] Add focused parser/fixture coverage for bare and parenthesized async arrows

Out of scope:

- Async function runtime lowering
- Promise/await semantics
- TypeScript contextual return-type inference
- Async generator functions

## Affected paths

Expected:

- `crates/frontend/src/parser/expressions_main.rs`
- `crates/frontend/src/parser/`
- focused fixtures or parser tests

Do not touch:

- backend/runtime async lowering unless fresh triage proves this parser slice has advanced to runtime

## Acceptance criteria

- [ ] A focused test proves `async () => 1` no longer reports raw `unsupported expression: Async`
- [ ] A focused test proves `(async () => { return 1; })` no longer reports raw `unsupported expression: Async`
- [ ] `circularInferredTypeOfVariable.ts` advances past the current parser failure at the parenthesized async arrow head
- [ ] `asyncIIFE.ts` advances past the current parser failure at `(async () => { ... })`
- [ ] `contextuallyTypeAsyncFunctionReturnTypeFromUnion.ts` advances past the
  current parser failure at the object property value
  `test: async () => Promise.reject(...)`
- [ ] Existing non-async arrow function tests still pass
- [ ] Any remaining unsupported async/await behavior reports a later source-spanned diagnostic with a specific feature label

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(async) or test(arrow)'
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularInferredTypeOfVariable.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/asyncIIFE.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularInferredTypeOfVariable.ts --detail --no-dashboard-data
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none expected for parser-only completion; create or update async runtime issue if parser support exposes `await`/Promise lowering

## Notes

Split from generated bucket `issues/done/1155-implement-circularInferredTypeOfVariable.md`.
Related triage buckets 751, 759, and 3758 show the same raw async-arrow parser
boundary and can be folded after fresh triage. Generated bucket 1496 was folded
on 2026-05-07 after `contextualReturnTypeOfIIFE.ts` reported the same raw
`unsupported expression: Async` failure at `(async () => { ... })()`.

Generated bucket
`issues/done/1536-implement-contextuallyTypeAsyncFunctionReturnTypeFromUnion.md`
was folded on 2026-05-07 after
`contextuallyTypeAsyncFunctionReturnTypeFromUnion.ts` reported the same raw
`unsupported expression: Async` failure for an object property value:

```ts
services: {
  test: async () => Promise.reject("some err"),
}
```

TypeScript accepts the source with no diagnostics and records the property
initializer as an `ArrowFunction`.

Generated bucket
`issues/done/3554-implement-noImplicitReturnsExclusions.md` was folded on
2026-05-08 after `noImplicitReturnsExclusions.ts` reported the same raw
`unsupported expression: Async` failure for a call argument:

```ts
registerCommand("_references-view.showHistoryItem", async (item) => {
  if (item instanceof HistoryItem) {
    return executeCommand("vscode.open", item.input.location.uri);
  }
});
```

TypeScript accepts the source with no diagnostics. The current compiler reaches
the prior function declarations and `registerCommand` declaration, then fails
before constructing the async arrow argument AST.

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
