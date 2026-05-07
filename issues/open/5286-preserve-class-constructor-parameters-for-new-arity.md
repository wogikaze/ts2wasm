---
id: 5286
title: "Preserve class constructor parameters for new arity"
type: feature
area: ir
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Preserve class constructor parameter counts when lowering class declarations so
`new C(arg)` is validated against the declared constructor signature instead of
being treated as a zero-argument constructor.

This is the current narrow blocker from `commentsInheritance.ts`.

## Problem

`commentsInheritance.ts` parses and resolves class declarations with
constructors such as `constructor(a: number)`, but lowered validation records the
constructor as accepting zero arguments. Calls like `new c2(10)` then fail before
the backend can emit valid code.

Problem: `new c2(10)` reports `ArityMismatch` because the lowered constructor
metadata expects between 0 and 0 arguments.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsInheritance.ts
```

Focused coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsInheritance.ts --detail --no-dashboard-data
```

Observed result:

```text
error: [ArityMismatch] constructor 19 expects between 0 and 0 argument(s), got 1
coverage: build_pass=0, unsupported=1, unsupported_diagcodes=ArityMismatch:1
```

Source context:

```ts
class c2 {
    public field2: number;
    constructor(a: number) {
        this.field2 = a;
    }
}
var c2_i = new c2(10);
```

Compiler evidence:

```text
tokens: ok through class constructors and `new` expressions
ast: ok; classes c1, c2, c3, and c4 are present
resolved: fails during lowered validation with ArityMismatch for a constructor call
wat: wat2wasm reports a stack type mismatch after the constructor argument is left on the stack
TypeScript oracle: infers c2_i: c2 and c4_i: c4; no constructor arity diagnostic
```

## Desired final state

Class constructors with supported parameters retain their parameter arity through
lowering, and `new C(arg)` no longer reports a false zero-argument
`ArityMismatch`.

## Scope

In scope:

- [ ] Preserve one-parameter class constructor arity in the class/new-call lowering path.
- [ ] Add a focused regression for `class C { constructor(a: number) {} } new C(1);`.
- [ ] Re-run `commentsInheritance.ts` and record the next blocker if this path advances.

Out of scope:

- Full inheritance semantics or `super(...)` ordering.
- Parameter properties, tracked by constructor parameter-property issues.
- TypeScript strict-property initialization diagnostics.
- Comment or declaration emit fidelity.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/frontend/src/`
- focused class-constructor fixtures or tests

Do not touch:

- package/module resolution
- runtime ABI unless the focused regression proves constructor call ABI metadata is missing there

## Acceptance criteria

- [ ] `new C(1)` for a class with `constructor(a: number)` no longer reports `ArityMismatch` expecting 0 arguments.
- [ ] A focused regression covers a one-parameter class constructor and a `new` call with one argument.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsInheritance.ts` no longer reports the false zero-arity constructor diagnostic for `new c2(10)`.
- [ ] Any next blocker from `commentsInheritance.ts` is recorded in this issue or split to a follow-up if outside this scope.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsInheritance.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsInheritance.ts --detail --no-dashboard-data
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

Split from `issues/open/1369-implement-commentsInheritance.md`.

Related but not duplicates:

- `issues/done/413-implement-arity.md` covered broader historical arity work, not this class-constructor lowering bug.
- `issues/done/5135-fix-builtin-arity-validation-coercion-globals.md` and `issues/done/5136-fix-arity-validation-regexp-string-prototype.md` cover builtin constructor and method arity validation.
- `issues/open/2460-implement-functionParameterArityMismatch.md` is a generated TypeScript function arity bucket, not a class `new` constructor issue.
- `issues/open/5268-support-derived-constructor-parameter-properties-after-super.md` covers derived constructor parameter properties, not ordinary constructor arity metadata.

## Completion evidence

Fill when implemented.
