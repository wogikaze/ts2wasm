---
id: 5318
title: "Support class expression class heritage"
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

Accept class declarations whose `extends` clause is a direct class expression,
starting with `class A extends class Expr {} {}`.

## Problem

Problem: `classFieldSuperAccessible.ts` parses to AST successfully, including
`ClassDecl A extends ClassExpr Expr`, but builtin resolution rejects the
heritage expression with `UnsupportedSyntax: only simple inheritance (extends
ClassName) is supported`.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classFieldSuperAccessible.ts
```

Observed 2026-05-07:

```text
resolved: UnsupportedSyntax only simple inheritance (extends ClassName) is supported
TypeScript oracle: ok, diagnostics=[]
```

Representative source:

```ts
class A extends class Expr {} {
    static {
        console.log(super.name);
    }
}
```

Compiler evidence:

- Tokens succeed for the nested `class Expr {}` heritage expression.
- AST succeeds with `ClassDecl { name: "A", extends: Some(ClassExpr { name:
  "Expr" }) }`.
- Resolution stops in builtin/class heritage validation before later `super.x`
  behavior can be observed.

## Desired final state

The resolver/lowering path accepts a direct `ClassExpr` in class heritage and
advances the representative file past the generic simple-inheritance rejection.

## Scope

In scope:

- [ ] Accept `extends class Expr {}` as a class heritage constructor source.
- [ ] Preserve the nested class expression name for diagnostics or generated
  constructor metadata.
- [ ] Add focused coverage for `class A extends class Expr {} {}`.

Out of scope:

- General class expression values, tracked by issue 5248.
- Call-expression class heritage, tracked by issue 5252.
- Invalid/non-constructor heritage diagnostics, tracked by issue 5256.
- Full `super.name`, `super.EPSILON`, or accessor-super runtime parity after
  this representative advances past heritage resolution.

## Affected paths

Expected:

- `crates/ir/src/builtin_resolver.rs`
- `crates/ir/src/name_resolver.rs`
- `crates/ir/src/`
- focused frontend/IR or CLI tests/fixtures

Do not touch:

- parser code unless the existing AST evidence changes
- unrelated class runtime behavior

## Acceptance criteria

- [ ] `classFieldSuperAccessible.ts` no longer reports the generic
  `only simple inheritance (extends ClassName) is supported` diagnostic for
  `extends class Expr {}`.
- [ ] A focused regression covers direct class-expression heritage.
- [ ] If later `super` runtime behavior still blocks the file, the next
  diagnostic is recorded in a follow-up issue.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir -E 'test(class) or test(heritage)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classFieldSuperAccessible.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classFieldSuperAccessible.ts --detail --no-dashboard-data
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
`issues/done/1207-implement-classFieldSuperAccessible.md`.

Related but not duplicates:

- `issues/open/5248-lower-class-expressions.md` handles class expressions used
  as ordinary assignment/initializer values.
- `issues/open/5252-support-call-expression-class-heritage.md` handles
  `extends makeBaseClass()`.
- `issues/open/5256-report-non-constructor-class-heritage-expressions.md`
  handles invalid non-constructor heritage diagnostics.

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
