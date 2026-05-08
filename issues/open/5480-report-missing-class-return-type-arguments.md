---
id: 5480
title: "Report missing class return type arguments"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Report a TypeScript-compatible diagnostic when a class method return annotation
references a generic class without the required type arguments.

## Problem

`noTypeArgumentOnReturnType1.ts` now tokenizes, parses, resolves, and
build-passes, but TypeScript reports TS2314 on the return type annotation.

Problem: `foo(): A` inside `class A<T>` silently build-passes instead of
reporting that generic type `A<T>` requires one type argument.

## Current failure

Focused coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noTypeArgumentOnReturnType1.ts --detail --no-dashboard-data
```

Result on 2026-05-08:

```text
reference/typescript/tests/cases/compiler/noTypeArgumentOnReturnType1.ts: build_pass
```

Focused triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noTypeArgumentOnReturnType1.ts
```

Representative source:

```ts
class A<T>{

 foo(): A{
  return null;
 }
}
```

Current frontend evidence:

```text
tokens: ok through class type parameter list and method return annotation `A`
ast: ok; class method body is preserved
resolved: ok; class method `foo` returns `Null`
ts2wasm diagnostic: BuildPass
```

TypeScript oracle:

```text
TS2314: Generic type 'A<T>' requires 1 type argument(s).
line 4, character 9, span 44..45
```

## Desired final state

The semantic checker preserves enough generic class arity metadata for erased
return type annotations to report missing type arguments on class method return
types. The representative fixture should report a source-spanned TS2314-like
diagnostic instead of silently build-passing.

## Scope

In scope:

- [ ] Track generic class arity for return type annotations on class methods.
- [ ] Detect `foo(): A` when `A` is declared as `class A<T>`.
- [ ] Emit a source-spanned diagnostic at the missing-type-argument return type.
- [ ] Preserve existing parsing and erasure of valid runtime class methods.

Out of scope:

- Generic assignability or nullability diagnostics for method bodies; see
  `issues/open/5437-report-typed-class-method-null-return.md`.
- Getter return annotation nullability; see
  `issues/open/5183-report-typed-getter-null-return-diagnostics.md`.
- Interface call or construct signature return parsing; see issues 5245 and
  5332.
- Full TypeScript generic type checking beyond class-name arity validation.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- focused parser or semantic regression tests

Do not touch:

- backend/runtime ABI
- unrelated generic call type-argument parser issues

## Acceptance criteria

- [ ] `noTypeArgumentOnReturnType1.ts` no longer reports `BuildPass` when
  TypeScript reports TS2314 at the method return annotation.
- [ ] A focused regression covers `class A<T> { foo(): A { return null; } }`.
- [ ] Valid class method return annotations without missing class type
  arguments continue to parse and lower as before.
- [ ] The generated bucket `issues/open/3568-implement-noTypeArgumentOnReturnType.md`
  remains closed as split to this child issue.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend class
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noTypeArgumentOnReturnType1.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noTypeArgumentOnReturnType1.ts --detail --no-dashboard-data
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
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

Split from generated bucket
`issues/open/3568-implement-noTypeArgumentOnReturnType.md` after fresh triage
on 2026-05-08.

Related but not duplicates:

- `issues/open/5437-report-typed-class-method-null-return.md` owns typed class
  method `return null` diagnostics; it does not cover missing type arguments
  on the return annotation itself.
- `issues/open/5309-skip-generic-type-arguments-in-type-annotations.md` owns
  parser erasure of generic type-argument syntax, not missing generic arity
  diagnostics after parsing succeeds.

## Completion evidence

Fill when implemented.
