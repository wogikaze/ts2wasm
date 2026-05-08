---
id: 5260
title: "Report class heritage trailing comma"
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

Report a source-spanned TypeScript diagnostic for a trailing comma in a class
heritage clause, such as `class D extends C, {}`, instead of the generic
`expected LeftBrace` parser error.

## Problem

Problem: `classHeritageWithTrailingSeparator.ts` tokenizes the class heritage
clause but the parser expects the class body immediately after the base
identifier and reports:

```text
UnsupportedSyntax: expected LeftBrace, got Some(Comma) at 62..63
```

TypeScript accepts enough AST shape to point at `HeritageClause "extends C,"`
and reports `TS1009: Trailing comma not allowed.` at the comma.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classHeritageWithTrailingSeparator.ts
```

Representative source:

```ts
class C { foo: number }
class D extends C, {
}
```

## Scope

In scope:

- [ ] Detect a comma immediately after a class heritage expression before the
  class body.
- [ ] Emit a source-spanned diagnostic at the comma that names the trailing
  class heritage separator boundary.
- [ ] Preserve normal `class D extends C {}` parsing.
- [ ] Preserve existing class field/type annotation erasure for the preceding
  `class C { foo: number }` in the representative.

Out of scope:

- Multiple heritage clauses or mixin support.
- Arbitrary class heritage expression support.
- Type-checking the class field definite-assignment diagnostic also reported by
  TypeScript in this reference.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- class runtime lowering
- unrelated heritage resolver behavior

## Acceptance criteria

- [ ] `classHeritageWithTrailingSeparator.ts` no longer reports
  `expected LeftBrace, got Some(Comma)`.
- [ ] A focused parser or CLI fixture covers `class D extends C, {}`.
- [ ] The new diagnostic is source-spanned at the comma.
- [ ] `class D extends C {}` still parses.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classHeritageWithTrailingSeparator.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classHeritageWithTrailingSeparator.ts --detail --no-dashboard-data
```

## Notes

Split from `issues/open/1214-implement-classHeritageWithTrailingSeparator.md`.
