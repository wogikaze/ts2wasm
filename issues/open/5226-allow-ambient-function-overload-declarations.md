---
id: 5226
title: "Allow ambient function overload declarations"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Allow multiple `declare function` overload signatures for the same top-level
name without classifying them as duplicate concrete function definitions.

## Problem

`checkInfiniteExpansionTermination2.ts` parses two ambient declarations for
`combineLatest`, but `validate_ast` rejects the second one as a duplicate
function. TypeScript accepts both declarations as an ambient overload set.

Current diagnostic:

```text
DuplicateFunction: duplicate function definition: `combineLatest` at 321..334
```

## Current failure

Use the validation command below to reproduce the representative failure.

Source shape:

```ts
declare function combineLatest<TOther>(x: IObservable<TOther>[]): void;
declare function combineLatest(): void;

function fn<T>() {
    var values: ISubject<any>[] = [];
    combineLatest<T>(values);
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; two ambient Function combineLatest declarations, then Function fn
validate_ast: DuplicateFunction on the second ambient declaration
TypeScript oracle: no diagnostics for the ambient overload declarations
```

## Desired final state

The validator groups bodyless ambient `declare function` declarations as an
ambient overload set and does not report `DuplicateFunction` for the
representative declarations.

## Scope

In scope:

- [ ] Distinguish ambient `declare function` overload declarations from
  concrete function implementations.
- [ ] Accept multiple bodyless ambient declarations with the same name.
- [ ] Preserve duplicate-function diagnostics for multiple non-ambient concrete
  function bodies.
- [ ] Add a focused fixture for two `declare function f(...)` declarations.

Out of scope:

- Non-ambient overload implementations, covered by issue 5200.
- Function overload list plus class merge diagnostics, covered by issue 5199.
- Full overload resolution or generic type instantiation.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- focused fixtures

Do not touch:

- `crates/backend-wasm/`
- unrelated runtime function call semantics

## Acceptance criteria

- [ ] `checkInfiniteExpansionTermination2.ts` no longer reports `DuplicateFunction` for the second ambient `combineLatest` declaration.
- [ ] A focused fixture covers two bodyless `declare function f(...)` overload declarations.
- [ ] Existing duplicate concrete function fixtures still report duplicate implementation diagnostics.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(function) or test(ambient) or test(overload)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkInfiniteExpansionTermination2.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkInfiniteExpansionTermination2.ts --detail
```

Not run:

- none

## Notes

Split from generated bucket `issues/done/1133-implement-checkInfiniteExpansionTermination.md`.

## Completion evidence

Fill when implemented.
