---
id: 5263
title: "Report primitive implements clauses on class expressions"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Parse TypeScript `implements` clauses on class expressions, including anonymous
class expressions, and report a source-spanned diagnostic for primitive
heritage types such as `number`, `string`, and `boolean`.

## Problem

`classImplementsPrimitive.ts` tokenizes successfully and parses the leading
class declarations, but it misparses the anonymous class expression
`class implements number {}` by treating `implements` as the class name. The
parser then expects the class body and fails at `number`.

Current diagnostic:

```text
UnsupportedSyntax: expected LeftBrace, got Some(Ident("number")) at 188..194
```

TypeScript parses the class expression as `ClassExpression -> HeritageClause ->
ExpressionWithTypeArguments` and reports `TS2864` at the primitive type.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classImplementsPrimitive.ts
```

Representative source:

```ts
class C implements number { }
class C2 implements string { }
class C3 implements boolean { }

const C4 = class implements number {}
const C5 = class implements string {}
const C6 = class implements boolean {}

const C7 = class A implements number { }
const C8 = class B implements string { }
const C9 = class C implements boolean { }
```

Compiler evidence:

```text
tokens: ok; class, implements, primitive identifiers, class bodies
visible symbols before failure: C, C2, C3, binding C4, class name "implements"
ast/resolved: expected LeftBrace, got Some(Ident("number")) at anonymous class expression
TypeScript oracle: TS2864 at all primitive implements types
```

## Desired final state

The parser recognizes `implements` as a TypeScript heritage clause in class
expressions instead of an optional class name. The representative should no
longer fail with `expected LeftBrace` at `number`; it should report a
source-spanned primitive-implements diagnostic or advance to the next blocker.

## Scope

In scope:

- [x] Parse anonymous class expression `implements` clauses.
- [x] Preserve named class expression `implements` clauses such as
  `class A implements number {}`.
- [x] Emit or preserve a diagnostic at the primitive type span for `number`,
  `string`, and `boolean`.
- [x] Preserve existing supported class declaration parsing.

Out of scope:

- Full TypeScript assignability for `implements`.
- Runtime support for class expression values beyond the already supported
  class-expression boundary.
- General interface implementation checking.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/`
- focused fixtures

Do not touch:

- `crates/backend-wasm/`
- unrelated class method or inheritance lowering

## Acceptance criteria

- [x] `classImplementsPrimitive.ts` no longer reports
  `expected LeftBrace, got Some(Ident("number"))`.
- [x] A focused fixture covers `const C4 = class implements number {}`.
- [x] A focused fixture covers `const C7 = class A implements number {}`.
- [x] Diagnostics are source-spanned at the primitive type identifier.
- [x] Existing `class C implements I {}` behavior remains unchanged.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(class) or test(implements) or test(parser)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classImplementsPrimitive.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classImplementsPrimitive.ts --detail --no-dashboard-data
```

## Notes

Split from `issues/done/1219-implement-classImplementsPrimitive.md`.

## False-done audit

**truly-done** (5263)

- Implementation commits: verified via `git log --oneline --all --grep=5263`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
