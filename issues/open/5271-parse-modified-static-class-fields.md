---
id: 5271
title: "Parse modified static class fields"
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

Teach the class member parser to accept public/private static field
declarations with initializers, starting with `private static x = 10`.

This is the first blocker in `cloduleStaticMembers.ts`.

## Problem

The parser tokenizes `private static x = 10;`, but after the modifiers it
expects a method parameter list and rejects the field name.

Problem: `reference/typescript/tests/cases/compiler/cloduleStaticMembers.ts`
reports `expected LeftParen, got Some(Ident("x"))` at `private static x = 10;`.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleStaticMembers.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("x")) at 53..54
```

Source context:

```ts
class Clod {
    private static x = 10;
    public static y = 10;
}
```

Smart triage evidence:

```text
tokens: ok; private, static, Ident("x"), Equal, Number(10)
AST: fails with expected LeftParen, got Some(Ident("x"))
resolved: same parser failure
TypeScript oracle: parses; later expected diagnostics TS2341 and TS2304
```

## Desired final state

The parser accepts static class fields after accessibility modifiers and
proceeds past `private static x = 10;` and `public static y = 10;`.

## Scope

In scope:

- [ ] Parse `private static name = expr;` class field declarations.
- [ ] Parse `public static name = expr;` class field declarations.
- [ ] Preserve existing static method parsing and ASI behavior for uninitialized
      static fields.
- [ ] Add focused parser/frontend coverage for modified static fields.
- [ ] Re-run the representative triage and split namespace/class merge or
      privacy diagnostics separately if outside this parser slice.

Out of scope:

- Runtime/backend static field lowering.
- TypeScript private/public access diagnostics.
- Namespace/class merge semantics.
- ASI-only static field boundary tracked by issue 5254.

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

- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleStaticMembers.ts` no longer reports `expected LeftParen, got Some(Ident("x"))`.
- [ ] A focused parser/frontend test accepts `class Clod { private static x = 10; public static y = 10; }`.
- [ ] Existing static method parsing remains unchanged.
- [ ] Any next blocker from the same reference path is recorded in this issue or split to a follow-up if outside this scope.

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
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleStaticMembers.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cloduleStaticMembers.ts --detail --no-dashboard-data
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

Split from generated bucket `issues/open/1255-implement-cloduleStaticMembers.md`.
Related but distinct issue: `issues/done/5254-parse-asi-between-static-class-fields.md`.

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
