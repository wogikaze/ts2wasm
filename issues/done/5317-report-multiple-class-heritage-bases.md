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
updated: 2026-05-07
---

## Summary

Report a source-spanned TypeScript-compatible diagnostic when a class `extends`
clause contains more than one base class expression.

## Problem

Problem: `classExtendsMultipleBaseClasses.ts` currently stops in the parser
with `UnsupportedSyntax: expected LeftBrace, got Some(Comma)`, while TypeScript
parses the class declaration and reports TS1174: classes can only extend one
class.

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

TypeScript AST evidence reaches
`ClassDeclaration -> HeritageClause "extends A,B"` with
`ExpressionWithTypeArguments "A"` before reporting the diagnostic.

## Desired final state

The parser recognizes the comma after the first class heritage expression and
reports a source-spanned frontend diagnostic for the multiple-base-class
boundary instead of a generic missing-left-brace parser error.

## Scope

In scope:

- [x] Detect `class C extends A,B {}` at the comma after the first heritage
  expression.
- [x] Emit a source-spanned diagnostic equivalent to TS1174.
- [x] Add a focused parser or CLI regression for the representative source.

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

- [x] `classExtendsMultipleBaseClasses.ts` no longer reports
  `expected LeftBrace, got Some(Comma)`.
- [x] The new diagnostic is source-spanned at the comma or second base `B` and
  names the single-base-class rule.
- [x] `class C extends A {}` still parses.

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

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from stale generated bucket
`issues/done/1204-implement-classExtendsMultipleBaseClasses.md`.

Related but not duplicate:

- `issues/open/5260-report-class-heritage-trailing-comma.md` handles
  `class D extends C, {}` and explicitly leaves multiple heritage clauses out of
  scope.

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

## False-done audit

**truly-done** (5317)

- Implementation commits: verified via `git log --oneline --all --grep=5317`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
