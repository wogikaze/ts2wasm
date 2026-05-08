---
id: 5369
title: "Parse call-expression type arguments in class heritage"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Erase TypeScript type arguments applied to call-expression class heritage, such
as `class Foo extends Tag("Foo")<Foo, Shape>() {}`.

## Problem

The contextual nested return type reference cases 2-4 parse generic
declarations, but stop when the class heritage expression applies type
arguments to the result of `Tag("Foo")`:

```ts
class Foo extends Tag("Foo")<
  Foo,
  {
    fn: (a: string) => Effect<void>;
  }
>() {}
```

Current diagnostic:

```text
UnsupportedSyntax: expected LeftBrace, got Some(Comma)
```

Problem: the class heritage parser expects the class body after `Tag("Foo")`
and treats the following `<Foo, ...>` as runtime syntax instead of erasing it as
TypeScript type arguments.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualParamTypeVsNestedReturnTypeInference2.ts
```

Focused coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter contextualParamTypeVsNestedReturnTypeInference --detail --no-dashboard-data
```

Observed result:

```text
contextualParamTypeVsNestedReturnTypeInference2.ts: UnsupportedSyntax expected LeftBrace, got Some(Comma)
contextualParamTypeVsNestedReturnTypeInference3.ts: UnsupportedSyntax expected LeftBrace, got Some(Comma)
contextualParamTypeVsNestedReturnTypeInference4.ts: UnsupportedSyntax expected LeftBrace, got Some(Comma)
```

Compiler evidence:

```text
tokens: ok through class Foo, extends, Tag("Foo"), <, Foo, comma, object type
ast/resolved: fail before class body parsing
TypeScript oracle: diagnostics=[]
```

## Desired final state

The parser accepts and erases type arguments after call-expression heritage
callees, then continues to parse the class body. The representative references
advance to the next parser, resolver, or semantic diagnostic.

## Scope

In scope:

- [x] Parse `class C extends factory()<T>() {}` style heritage expressions.
- [x] Add a focused parser regression for `extends Tag("Foo")<Foo, Shape>()`.

Out of scope:

- Plain generic heritage such as `extends Base<T>`, tracked by `issues/open/5156-parse-generic-type-arguments-in-class-heritage.md`.
- Direct generic callable const calls outside class heritage, tracked by `issues/done/5242-w2-completion-declaration.md`.
- Contextual type inference after this parser boundary.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_class.rs`
- `crates/frontend/src/parser/expressions_main.rs`
- `crates/frontend/src/parser/tests.rs`

Do not touch:

- backend/runtime ABI
- full TypeScript generic inference
- unrelated class heritage diagnostics

## Acceptance criteria

- [x] `class Foo extends Tag("Foo")<Foo, Shape>() {}` parses without `expected LeftBrace, got Some(Comma)`.
- [x] `contextualParamTypeVsNestedReturnTypeInference2.ts` advances past the current class heritage parser diagnostic.
- [x] Existing `extends Base` and `extends Base<T>` class heritage tests continue to pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(class) or test(heritage)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualParamTypeVsNestedReturnTypeInference2.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter contextualParamTypeVsNestedReturnTypeInference --detail --no-dashboard-data
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

Split from generated bucket
`issues/open/1491-implement-contextualParamTypeVsNestedReturnTypeInference.md`
on 2026-05-07.

Related but not duplicates:

- `issues/open/5156-parse-generic-type-arguments-in-class-heritage.md` covers
  plain and qualified generic heritage references such as `extends Base<T>`.
- `issues/done/5242-w2-completion-declaration.md`
  covers direct generic calls in expression positions, not class heritage.

## Completion evidence

Fill only when implemented.
