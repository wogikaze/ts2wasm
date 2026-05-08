---
id: 5267
title: "Parse string literal class member names"
type: feature
area: frontend
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Teach the class member parser to accept quoted string-literal method names after
TypeScript accessibility/static modifiers, covering the first blocker in
`classStaticPropertyAccess.ts`.

This is a narrow parser slice for class declaration member names such as
`public static "\""() {}`.

## Problem

`classStaticPropertyAccess.ts` tokenizes the quoted member name correctly as a
string token, but class member parsing expects a left parenthesis after
`public static` and rejects the string-literal property name.

Problem: `reference/typescript/tests/cases/compiler/classStaticPropertyAccess.ts`
reports `expected LeftParen, got Some(String("\""))` at the static quoted method
name.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classStaticPropertyAccess.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(String("\"")) at 74..78
```

Source context:

```ts
class A {
    public static "\""() {}
    public static x: number = 1;
    public static y: number = 1;
    private static _b: number = 2;
}
```

Smart triage evidence:

```text
tokens: ok; String("\"") token at 74..78 after public static
AST: fails with expected LeftParen, got Some(String("\""))
resolved: same parser failure
TypeScript oracle: parses; later expected diagnostics TS2576, TS2341, TS2339
```

## Desired final state

The parser accepts string-literal class member names in the supported class
declaration subset after accessibility/static modifiers and proceeds past the
quoted method name in `classStaticPropertyAccess.ts`.

## Scope

In scope:

- [ ] Parse class declaration methods with string-literal names such as
      `static "\""() {}`.
- [ ] Preserve modifier handling for `public static` and existing identifier
      method names.
- [ ] Add focused parser/frontend coverage for quoted static method names in
      class declarations.
- [ ] Re-run the representative triage and split any later static property
      access/type-checking blocker separately if outside this parser slice.

Out of scope:

- Computed class member names; tracked by issue 5251.
- Full static property access type checking.
- General first-class class constructor value semantics.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- runtime ABI
- package/module resolution
- unrelated class lowering

## Acceptance criteria

- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classStaticPropertyAccess.ts` no longer reports `expected LeftParen, got Some(String("\""))`.
- [ ] A focused parser/frontend test accepts `class A { public static "\""() {} }`.
- [ ] Existing identifier-named static methods and fields continue to parse.
- [ ] Any next blocker from the same reference path is recorded in this issue or split to a follow-up if outside quoted member parsing.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classStaticPropertyAccess.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classStaticPropertyAccess.ts --detail --no-dashboard-data
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
`issues/done/1239-implement-classStaticPropertyAccess.md`.
Related but distinct open issue:
`issues/done/5251-parse-computed-class-member-names-in-class-declarations.md`.

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
