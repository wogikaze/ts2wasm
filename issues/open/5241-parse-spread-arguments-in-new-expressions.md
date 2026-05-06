---
id: 5241
title: "Parse spread arguments in new expressions"
type: bug
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Use the existing spread-aware argument parser for `new` expressions so
`new C(...args)` reaches AST/lowering instead of failing on the `...` token.

## Problem

Ordinary calls already parse spread arguments through `finish_call_args()`, but
the `new` expression parser still parses constructor arguments with
`self.expression()` directly. As a result, `new Foo<T>(...this.elements, value)`
fails during parsing even though spread argument syntax is supported elsewhere.

Problem: spread arguments in constructor calls are rejected as raw `DotDotDot` parser errors before class/type-system behavior can run.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularInlineMappedGenericTupleTypeNoCrash.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: DotDotDot, span: Span { start: 394, end: 397 } }) at 397..401
```

Source context:

```ts
public add(): Foo<[...Elements, "abc"]> {
  return new Foo<[...Elements, "abc"]>(...this.elements, { bar: "abc" });
}
```

Parser evidence:

```text
tokens: ok; includes New, Foo, type arguments, LeftParen, DotDotDot, This, Dot, elements
ast: fails before construction with unsupported expression DotDotDot at the first constructor spread argument
ordinary calls already route argument parsing through finish_call_args(), which accepts DotDotDot
new expression argument parsing still uses self.expression() directly
```

## Desired final state

`new` expressions parse constructor spread arguments into the existing
`Expr::Spread` representation, matching ordinary call syntax. If constructor
spread lowering remains unsupported, the compiler reports a later source-spanned
diagnostic instead of a parser `DotDotDot` error.

## Scope

In scope:

- [ ] Route `new` expression argument lists through the existing spread-aware argument parser
- [ ] Preserve TypeScript `new Foo<T>(...)` type-argument skipping
- [ ] Add a focused parser regression for `new C(...args, value)`
- [ ] Preserve existing ordinary call spread behavior

Out of scope:

- General iterator protocol spread semantics, tracked by issue 353
- Broad spread operator completion, tracked by issue 274
- Runtime constructor spread lowering beyond reaching a later diagnostic
- Tuple/mapped-type semantic parity for `circularInlineMappedGenericTupleTypeNoCrash.ts`

## Affected paths

Expected:

- `crates/frontend/src/parser/expressions_main.rs`
- `crates/frontend/src/parser/tests.rs`
- focused fixtures if parser tests are not enough

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`
- spread runtime semantics

## Acceptance criteria

- [ ] A focused parser test proves `new C(...args)` produces `Expr::New` with an `Expr::Spread` argument
- [ ] A focused parser test proves `new C(...args, value)` preserves argument order
- [ ] `circularInlineMappedGenericTupleTypeNoCrash.ts` advances past the current parser failure at `...this.elements`
- [ ] Existing ordinary call spread parser tests still pass
- [ ] Any remaining unsupported constructor-spread behavior is reported after AST construction with a source-spanned diagnostic

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(spread) or test(new)'
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularInlineMappedGenericTupleTypeNoCrash.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularInlineMappedGenericTupleTypeNoCrash.ts --detail --no-dashboard-data
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none expected for parser-only completion; update issue 274/353 only if later spread runtime diagnostics change

## Notes

Split from generated bucket `issues/done/1156-implement-circularInlineMappedGenericTupleTypeNoCrash.md`.

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
