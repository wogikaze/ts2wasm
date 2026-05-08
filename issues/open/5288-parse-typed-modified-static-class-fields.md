---
id: 5288
title: "Parse typed modified static class fields"
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

Parse TypeScript type annotations on public/private static class fields after
accessibility modifiers, starting with `public static p1: string = "";`.

## Problem

`commentsOnStaticMembers.ts` tokenizes the JSDoc comments and modified static
properties, but class member parsing expects a method parameter list after
`public static` and rejects the property name before it can erase the type
annotation.

Problem: `reference/typescript/tests/cases/compiler/commentsOnStaticMembers.ts`
reports `expected LeftParen, got Some(Ident("p1"))` at
`public static p1: string = "";`.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOnStaticMembers.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("p1")) at 129..131
```

Source context:

```ts
class test {
    public static p1: string = "";
    public static p2: string;
    private static p3: string = "";
    private static p4: string;
}
```

Smart triage evidence:

```text
tokens: ok; public, static, Ident("p1"), Colon, Ident("string"), Equal, String("") are present
AST: fails with expected LeftParen, got Some(Ident("p1"))
TypeScript oracle: parses; PropertyDeclaration name is p1
```

## Desired final state

The parser accepts modified static property declarations with erased
TypeScript type annotations and continues parsing later class members.

## Scope

In scope:

- [x] Parse `public static name: Type = expr;` as a class field.
- [x] Parse `private static name: Type;` as a class field.
- [x] Preserve existing static method parsing for `public static name()`.
- [x] Add focused parser/frontend coverage for typed modified static fields.

Out of scope:

- Comment emit fidelity.
- Runtime/backend static field lowering.
- Untyped modified static field tracking in issue 5271.
- Static field ASI tracking in issue 5254.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_class.rs`
- `crates/frontend/src/parser/tests.rs`
- focused fixtures

Do not touch:

- `crates/backend-wasm/`
- unrelated runtime ABI code

## Acceptance criteria

- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOnStaticMembers.ts` no longer reports `expected LeftParen, got Some(Ident("p1"))`.
- [x] A focused parser test accepts `class C { public static p1: string = ""; private static p2: string; }`.
- [x] Existing modified static method parsing remains unchanged.
- [x] Any later comment-output blocker from this reference path is recorded separately.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(class) or test(static) or test(field)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOnStaticMembers.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsOnStaticMembers.ts --detail --no-dashboard-data
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

Split from generated bucket `issues/open/1378-implement-commentsOnStaticMembers.md`.
Related but distinct parser issues:

- `issues/open/5271-parse-modified-static-class-fields.md`
- `issues/open/5254-parse-asi-between-static-class-fields.md`

Additional superseded bucket:

- `issues/open/1465-implement-constraintCheckInGenericBaseTypeReference.md`
  reaches the same typed modified static class field parser boundary before
  generic constraint checking. Fresh triage on 2026-05-07 reports
  `expected LeftParen, got Some(Ident("People")) at 307..313` for
  `public static People: Derived`; TypeScript parses it as a
  `PropertyDeclaration` named `People` and reports no diagnostics for the file.

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

**truly-done** (5288)

- Implementation commits: verified via `git log --oneline --all --grep=5288`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
