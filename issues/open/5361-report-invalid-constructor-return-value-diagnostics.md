---
id: 5361
title: "Report invalid constructor return value diagnostics"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Report TypeScript-compatible diagnostics when a class constructor returns a
value that is not assignable to the class instance type.

## Problem

`constructorReturningAPrimitive.ts` build-passes, but TypeScript reports
TS2322/TS2409 for a generic constructor returning a `T` value where the
constructor return type must be assignable to `B<T>`.

Problem: constructor return value semantic validation currently does not reject
non-instance return values, so the representative path incorrectly build-passes.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorReturningAPrimitive.ts
```

Current compiler diagnostic:

```text
BuildPass
```

TypeScript oracle evidence:

```text
TS2322: Type 'T' is not assignable to type 'B<T>'.
TS2409: Return type of constructor signature must be assignable to the instance type of the class.
TS2454: Variable 'x' is used before being assigned.
```

Representative source:

```ts
class B<T> {
    constructor() {
        var x: T;
        return x;
    }
}
```

## Desired final state

Semantic validation reports a source-spanned diagnostic at constructor return
statements whose returned value is not assignable to the class instance type.

## Scope

In scope:

- [ ] Detect constructor `return` statements with a value expression.
- [ ] Emit a source-spanned constructor return-value diagnostic for `return x` in the representative generic class.
- [ ] Re-run the representative triage and confirm it no longer reports BuildPass.

Out of scope:

- Full generic assignability implementation.
- Definite-assignment diagnostics for `x` before assignment.
- Runtime constructor return semantics.

## Affected paths

Expected:

- `crates/frontend/src/ast.rs`
- `crates/ir/src/semantic.rs`
- focused semantic tests

Do not touch:

- backend runtime code
- module system code

## Acceptance criteria

- [ ] `constructorReturningAPrimitive.ts` no longer reports BuildPass.
- [ ] A focused test reports the diagnostic for `class B<T> { constructor() { var x: T; return x; } }`.
- [ ] Constructors with no value return, such as `return;`, remain accepted if currently supported.
- [ ] Definite-assignment for `x` remains out of scope or is recorded as a follow-up if exposed first.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir -E 'test(class) or test(constructor) or test(return)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorReturningAPrimitive.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorReturningAPrimitive.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
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

Split from `issues/open/1478-implement-constructorReturningAPrimitive.md` on
2026-05-07 after fresh coverage reported build_pass.

The non-generic `class A { constructor() { return 1; } }` portion does not
produce a TypeScript oracle diagnostic in this reference file; keep the initial
implementation focused on the observed `return x` diagnostic.

## Completion evidence

Fill only when implemented.
