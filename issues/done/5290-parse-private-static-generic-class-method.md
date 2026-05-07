---
id: 5290
title: "Parse private static generic class methods"
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

Parse `private static` class method declarations whose method name is followed
by erased TypeScript type parameters, starting with
`private static privatestaticmethod<U>(a: U)`.

## Problem

`commentsTypeParameters.ts` tokenizes generic class and method declarations,
including comments inside type parameter lists. Parsing reaches the
`private static privatestaticmethod<...>` member, but after `private static`
the class member parser expects a parameter list and rejects the method name.

Problem: `reference/typescript/tests/cases/compiler/commentsTypeParameters.ts`
reports `expected LeftParen, got Some(Ident("privatestaticmethod"))`.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsTypeParameters.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("privatestaticmethod")) at 404..423
```

Source context:

```ts
class C<T> {
    private privatemethod<U extends T>(a: U) {
    }
    private static privatestaticmethod<U>(a: U) {
    }
}
```

Smart triage evidence:

```text
tokens: ok; private, static, Ident("privatestaticmethod"), Less, Ident("U") are present
AST: fails with expected LeftParen, got Some(Ident("privatestaticmethod"))
TypeScript oracle: ok, MethodDeclaration name is privatestaticmethod
```

## Desired final state

The parser accepts the `private static` generic method declaration and erases
its TypeScript type parameter list, so `commentsTypeParameters.ts` advances to
the next blocker or pass state.

## Scope

In scope:

- [ ] Parse `private static name<T>(...) { ... }` class methods.
- [ ] Erase comments/trivia inside the method type parameter list.
- [ ] Preserve existing parsing for unmodified generic methods and static generic methods.
- [ ] Add focused parser/frontend coverage for a private static generic method.

Out of scope:

- Declaration emit or comment emit fidelity.
- General type parameter semantic checking.
- Non-static private generic methods already parsed before the current failure.
- Broader modified static method parser cleanup tracked by issue 5275.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_class.rs`
- `crates/frontend/src/parser/tests.rs`
- focused fixtures

Do not touch:

- `crates/backend-wasm/`
- unrelated runtime semantics

## Acceptance criteria

- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsTypeParameters.ts` no longer reports `expected LeftParen, got Some(Ident("privatestaticmethod"))`.
- [ ] A focused parser test accepts `class C { private static m<T>(a: T) {} }`.
- [ ] Existing unmodified generic class method parsing remains covered.
- [ ] Any later declaration/comment emit blocker from this reference path is recorded separately.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(class) or test(static) or test(generic)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsTypeParameters.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsTypeParameters.ts --detail --no-dashboard-data
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

Split from generated bucket `issues/open/1380-implement-commentsTypeParameters.md`.
Related but broader issue: `issues/open/5275-parse-modified-static-class-methods.md`.

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
