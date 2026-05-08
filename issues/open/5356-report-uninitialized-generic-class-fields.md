---
id: 5356
title: "Report uninitialized generic class fields"
type: bug
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-08
---

## Summary

Report TS2564-style diagnostics for uninitialized instance class fields in the
representative generic class case instead of returning a false build pass.

Split from generated bucket `1472`.

## Problem

`constructorInvocationWithTooFewTypeArgs.ts` now builds successfully in ts2wasm,
but TypeScript reports TS2564 for fields `x` and `y` before the later TS2558
too-few-type-arguments diagnostic.

Problem: current failure is a false build pass for `class D<T, U> { x: T; y: U }`, where TypeScript reports uninitialized property diagnostics.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorInvocationWithTooFewTypeArgs.ts
```

Current result:

```text
BuildPass: ts2wasm build succeeded
```

TypeScript oracle diagnostics:

```text
TS2564: Property 'x' has no initializer and is not definitely assigned in the constructor.
TS2564: Property 'y' has no initializer and is not definitely assigned in the constructor.
TS2558: Expected 2 type arguments, but got 1.
```

Source context:

```ts
class D<T, U> {
   x: T
   y: U
}
var d = new D<number>();
```

Compiler evidence:

```text
tokens: ok through generic class, typed fields, and new D<number>()
ast/resolved: ok; ClassDecl D body is empty and Let d = New D is retained
coverage: executed=1, build_pass=1, unsupported=0
```

## Desired final state

The compiler reports source-spanned uninitialized instance field diagnostics for
the representative `x` and `y` fields instead of returning build success. Any
later TS2558 type-argument diagnostic is recorded separately if still uncovered.

## Scope

In scope:

- [x] Preserve enough typed instance field metadata to detect uninitialized fields in the representative class.
- [x] Report source-spanned diagnostics at fields `x` and `y`.
- [x] Add focused parser/compiler coverage for `class D<T, U> { x: T; y: U }`.

Out of scope:

- Generic constructor type-argument count checking, including TS2558.
- Full TypeScript strict-property-initialization parity.
- Runtime emission for class fields.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/ast.rs`
- `crates/compiler/src/` or `crates/ir/src/`
- focused parser/compiler tests

Do not touch:

- backend/runtime class field emission unless fresh implementation evidence proves it is required.

## Acceptance criteria

- [x] `constructorInvocationWithTooFewTypeArgs.ts` no longer reports `BuildPass`; it reports a source-spanned diagnostic for `x` or `y`.
- [x] A focused test covers `class D<T, U> { x: T; y: U }`.
- [x] `new D<number>()` remains parsed so a later TS2558 issue can be triaged after TS2564 advances.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(class) or test(field)'
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorInvocationWithTooFewTypeArgs.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorInvocationWithTooFewTypeArgs.ts --detail --no-dashboard-data
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] create a TS2558 type-argument-count issue if that remains after TS2564 advances.

## Notes

Several older buckets mention TS2564 as later oracle diagnostics, but no open
implementation-ready owner was found for this representative strict-property
initialization false-pass slice.

2026-05-08 fold-in from generated bucket `3419`:

- `mutuallyRecursiveInference.ts` also returns a false build pass while the
  TypeScript oracle reports TS2564 for uninitialized typed instance fields.
- Source context includes `class T<A> { a: A; b: any }` and
  `class X extends L<X> { a: 'a' | 'b'; b: number }`.
- Oracle diagnostics are TS2564 for `T.a`, `X.a`, and `X.b`; current compiler
  evidence parses and resolves the mutually recursive generic heritage
  `L<RT extends { a: 'a' | 'b', b: any }> extends T<RT[RT['a']]>`, erases the
  typed fields, and returns `build_pass`.

Also owns `issues/open/3592-implement-nonGenericClassExtendingGenericClassWithAny.md`:
fresh triage now build-passes and TypeScript reports TS2564 for generic field
`Foo<T>.t` before any `Foo<any>` heritage parity gap is actionable.

Also owns the TS2564 diagnostics in
`issues/open/3593-implement-nonIdenticalTypeConstraints.md`: fresh triage now
build-passes and TypeScript reports TS2564 for uninitialized fields in
`Different`, `Foo`, `Qux`, `Bar`, `Baz`, and `Quux`. The TS2428 merged
declaration type-parameter diagnostics were split to issue 5487.

## Completion evidence

Fill only when moving to `done/`.

## False-done audit

**truly-done** (5356)

- Implementation commits: verified via `git log --oneline --all --grep=5356`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
