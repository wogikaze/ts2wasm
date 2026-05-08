---
id: 5321
title: "Report super class field access diagnostic"
type: feature
area: ir/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Report a TypeScript-compatible diagnostic when a child class accesses a parent
instance field through `super`.

## Problem

Problem: `classFieldSuperNotAccessible.ts` currently reaches lowering and
reports `UnsupportedSyntax: super method T.field not found`, while TypeScript
reports TS2855 because parent class fields are not accessible via `super`.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classFieldSuperNotAccessible.ts
```

Observed 2026-05-07:

```text
lower_program: UnsupportedSyntax super method `T.field` not found at 91..104
TypeScript oracle:
TS2855 Class field 'field' defined by the parent class is not accessible in the child class via super.
```

Representative source:

```ts
class T {
    field = () => {}
}
class T2 extends T {
    f() {
        super.field()
    }
}
```

Compiler evidence:

- Tokens and AST succeed.
- AST represents `field = () => {}` as an erased class field and
  `super.field()` as a call on member `super.field`.
- Lowering treats the access as a missing super method instead of reporting the
  TypeScript class-field-via-super diagnostic.

## Desired final state

The resolver/lowering pipeline distinguishes parent class fields from methods
for `super.field()` and reports a source-spanned TS2855-equivalent diagnostic.

## Scope

In scope:

- [ ] Preserve enough class field metadata to identify `T.field` as a field, not
  a prototype method.
- [ ] Diagnose `super.field()` when `field` is a parent class field.
- [ ] Add focused coverage for `class T { field = () => {} } class U extends T { m() { super.field(); } }`.

Out of scope:

- Supporting runtime access to parent fields through `super`.
- General `super.x` runtime property reads, tracked by issue 5255.
- Static-block `super.x`, tracked by issue 5319.

## Affected paths

Expected:

- `crates/ir/src/lowered/resolver_expr.rs`
- `crates/ir/src/`
- `crates/frontend/src/parser/statements_class.rs`
- focused IR/frontend or CLI tests

Do not touch:

- backend/runtime lowering unless the diagnostic cannot be emitted before
  backend

## Acceptance criteria

- [ ] `classFieldSuperNotAccessible.ts` no longer reports
  `super method T.field not found`.
- [ ] The new diagnostic is source-spanned at `field` or `super.field()` and
  names the class-field-via-super boundary.
- [ ] Parent prototype method calls through `super.method()` remain accepted.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir -p ts2wasm-frontend -E 'test(class) or test(super)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classFieldSuperNotAccessible.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classFieldSuperNotAccessible.ts --detail --no-dashboard-data
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

Split from stale generated bucket
`issues/open/1209-implement-classFieldSuperNotAccessible.md`.

Related but not duplicate:

- `issues/open/5255-resolve-super-property-accesses.md` covers broad `super.x`
  receiver resolution; this issue is the narrower TS2855 class-field diagnostic.

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
