---
id: 5199
title: "Report function overload list class merge diagnostics"
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

Classify top-level function overload declarations that are followed by a class
of the same name and report the TypeScript-compatible declaration-merge
diagnostics instead of stopping at `DuplicateFunction`.

## Problem

`callOverloads3.ts`, `callOverloads4.ts`, and `callOverloads5.ts` parse
successfully, but `validate_ast` rejects the second bodyless
`function Foo(...)` declaration as a duplicate function. The `callOverloads1.ts`
and `callOverloads2.ts` windows also contain the same class/function merge
problem after their `F1` function-overload implementation blocker is removed.
TypeScript treats these declarations as invalid function overload list/class
merges and reports diagnostics at the function and class names.

Problem: top-level bodyless function overload declarations are currently handled as duplicate concrete function implementations.

## Current failure

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOverloads3.ts
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOverloads4.ts
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOverloads5.ts
```

Current diagnostic for both files:

```text
error: [DuplicateFunction] duplicate function definition: `Foo` at 52..60
```

Representative source:

```ts
function Foo():Foo; // error
function Foo(s:string):Foo; // error
class Foo { // error
    bar1() { }
    constructor(x: any) { }
}
```

The `callOverloads4.ts` variant also includes a bodyless constructor overload
inside the class before the constructor implementation. The `callOverloads5.ts`
variant includes class method overload signatures inside the invalid class.

Triage evidence:

- Tokens and AST succeed for both reference files.
- AST contains two top-level bodyless `Function Foo` declarations followed by
  `ClassDecl Foo`.
- `callOverloads4.ts` additionally contains two `constructor` class members,
  where the first constructor declaration is bodyless.
- `callOverloads5.ts` additionally contains bodyless `bar1` class method
  overload signatures before its implementation.
- TypeScript oracle reports TS2814 for function/class merge, TS2391 for the
  missing function implementation ordering, and TS2813 for the class
  declaration implementing overload list `Foo`.

## Desired final state

The resolver distinguishes bodyless top-level overload declarations from
duplicate concrete function implementations. For the representative `Foo`
function-overload/class pair, the compiler reaches a narrower diagnostic that
matches the declaration-merge problem instead of the generic
`DuplicateFunction` boundary.

## Scope

In scope:

- [ ] Distinguish bodyless top-level function overload signatures from concrete
  duplicate function implementations
- [ ] Detect a class declaration with the same name as a pending function
  overload list
- [ ] Emit a source-spanned diagnostic for the invalid class/function overload
  merge
- [ ] Preserve duplicate-function diagnostics for multiple concrete function
  bodies

Out of scope:

- Full ambient declaration merging
- Runtime support for function/class namespace merging
- Constructor overload implementation, except preserving enough AST shape for
  `callOverloads4.ts` to reach this same top-level merge diagnostic

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/backend-wasm/`
- unrelated JavaScript duplicate function redeclaration semantics

## Acceptance criteria

- [ ] `callOverloads3.ts` no longer reports generic `DuplicateFunction` for
  the second `function Foo` overload declaration
- [ ] `callOverloads4.ts` no longer reports generic `DuplicateFunction` for
  the second `function Foo` overload declaration
- [ ] `callOverloads5.ts` no longer reports generic `DuplicateFunction` for
  the second `function Foo` overload declaration
- [ ] After issue 5200 removes the `F1` blocker, `callOverloads1.ts` and
  `callOverloads2.ts` reach a class/function merge diagnostic for `Foo`
- [ ] A focused fixture covers two bodyless `function Foo` declarations followed
  by `class Foo`
- [ ] A focused fixture covers `class Foo` followed by bodyless
  `function Foo();`
- [ ] Multiple concrete function bodies with the same name still report the
  existing duplicate-function diagnostic
- [ ] The new diagnostic is source-spanned at the offending function or class
  name

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(function) | test(class)'
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOverloads3.ts
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOverloads4.ts
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOverloads5.ts
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

Related bucket `issues/open/769-implement-augmentedTypesFunction.md` also shows
TypeScript class/function merge diagnostics, but it is currently blocked on an
earlier parser-syntax failure in a different reference file. This issue owns the
already-parsed `callOverloads3.ts` and `callOverloads4.ts` duplicate-function
blocker.

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
