---
id: 5317
title: "Report multiple class heritage bases"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-08
---

## Summary

Report a source-spanned TypeScript-compatible diagnostic when a class `extends`
clause contains more than one base class expression.

## Problem

Problem: `classExtendsMultipleBaseClasses.ts` currently stops in the parser
with `UnsupportedSyntax: expected LeftBrace, got Some(Comma)`, while TypeScript
parses the class declaration and reports TS1174: classes can only extend one
class.

The same parser boundary now owns `multipleInheritance.ts`, which stops at
`class C extends B1, B2 {}` before later TS1174 and class-member diagnostics
can be exposed.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsMultipleBaseClasses.ts
```

Observed 2026-05-07:

```text
ts2wasm: UnsupportedSyntax expected LeftBrace, got Some(Comma) at 63..64
source: class C extends A,B { }
TypeScript oracle:
TS1174 Classes can only extend a single class.
```

Additional representative path, observed 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multipleInheritance.ts
```

```text
ts2wasm: UnsupportedSyntax expected LeftBrace, got Some(Comma) at 121..122
source: class C extends B1, B2 { // duplicate member }
TypeScript oracle:
TS1174 Classes can only extend a single class.
TS1174 Classes can only extend a single class.
```

TypeScript AST evidence reaches
`ClassDeclaration -> HeritageClause "extends A,B"` with
`ExpressionWithTypeArguments "A"` before reporting the diagnostic.

## Desired final state

The parser recognizes the comma after the first class heritage expression and
reports a source-spanned frontend diagnostic for the multiple-base-class
boundary instead of a generic missing-left-brace parser error.

## Scope

In scope:

- [ ] Detect `class C extends A,B {}` at the comma after the first heritage
  expression.
- [ ] Emit a source-spanned diagnostic equivalent to TS1174.
- [ ] Add a focused parser or CLI regression for the representative source.

Out of scope:

- Trailing heritage comma without a second base, tracked by issue 5260.
- Interface multiple-base `extends` clauses, which are valid TypeScript syntax.
- Mixin/runtime support for multiple base classes.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/frontend/src/parser/`
- focused frontend or CLI tests/fixtures

Do not touch:

- class runtime lowering
- unrelated resolver/backend behavior

## Acceptance criteria

- [ ] `classExtendsMultipleBaseClasses.ts` no longer reports
  `expected LeftBrace, got Some(Comma)`; the same parser path also covers
  `multipleInheritance.ts` at `class C extends B1, B2`.
- [ ] The new diagnostic is source-spanned at the comma or second base `B` and
  names the single-base-class rule.
- [ ] `class C extends A {}` still parses.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(class) or test(heritage)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsMultipleBaseClasses.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsMultipleBaseClasses.ts --detail --no-dashboard-data
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
`issues/done/1204-implement-classExtendsMultipleBaseClasses.md`.

Related but not duplicate:

- `issues/open/5260-report-class-heritage-trailing-comma.md` handles
  `class D extends C, {}` and explicitly leaves multiple heritage clauses out of
  scope.
- `issues/done/3415-implement-multipleInheritance.md` is a generated bucket
  closed into this owner after fresh triage showed the same
  `expected LeftBrace, got Some(Comma)` parser boundary at `class C extends B1, B2`.

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
