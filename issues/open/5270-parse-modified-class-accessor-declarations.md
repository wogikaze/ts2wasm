---
id: 5270
title: "Parse modified class accessor declarations"
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

Teach the class member parser to accept accessor declarations after TypeScript
accessibility and `static` modifiers, starting with `public static get p2()`.

This is the first blocker in the broad `classdecl.ts` reference bucket.

## Problem

The parser accepts the leading class declaration and earlier methods/fields,
but after `public static` it expects a method parameter list and rejects the
`get` accessor keyword.

Problem: `reference/typescript/tests/cases/compiler/classdecl.ts` reports
`expected LeftParen, got Some(Ident("get"))` at `public static get p2()`.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classdecl.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("get")) at 348..351
```

Source context:

```ts
class a {
    public static get p2() {
        return { x: 30, y: 40 };
    }

    private static d2() {
    }
}
```

Smart triage evidence:

```text
tokens: ok; public static get p2() tokens are present
AST: fails with expected LeftParen, got Some(Ident("get"))
resolved: same parser failure
TypeScript oracle: ok, diagnostics=[]
```

## Desired final state

The parser recognizes getter/setter accessors after supported class member
modifiers and proceeds past `public static get p2()` in `classdecl.ts`.

## Scope

In scope:

- [ ] Parse `public static get name() { ... }` class accessor declarations.
- [ ] Parse the matching modifier ordering for non-static public/private
      accessors if the same parser path handles them.
- [ ] Preserve existing static method and field parsing.
- [ ] Add focused parser/frontend coverage for modified static getter
      declarations.
- [ ] Re-run the representative triage and split any later constructor overload,
      namespace, declaration-emit, or class accessor blocker separately if
      outside this parser slice.

Out of scope:

- Full class accessor runtime emit.
- TypeScript accessor type-checking diagnostics.
- Computed accessor names.

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

- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classdecl.ts` no longer reports `expected LeftParen, got Some(Ident("get"))`.
- [ ] A focused parser/frontend test accepts `class A { public static get p2() { return 1; } }`.
- [ ] Existing static method and static field tests continue to parse.
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
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classdecl.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classdecl.ts --detail --no-dashboard-data
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

Split from generated bucket `issues/done/1249-implement-classdecl.md`.
Related broad class-accessor bucket: `issues/open/422-implement-class-accessor.md`.

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
