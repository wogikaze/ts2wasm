---
id: 5275
title: "Parse modified static class methods"
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

Parse class method declarations with TypeScript accessibility modifiers before
`static`, starting with `public static foo(): string { ... }`.

## Problem

`commentBeforeStaticMethod1.ts` tokenizes successfully, including the JSDoc
comment and `public static foo()` tokens, but after `public static` the parser
expects a parameter list and rejects the method name.

Current diagnostic:

```text
UnsupportedSyntax: expected LeftParen, got Some(Ident("foo")) at 79..82
```

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentBeforeStaticMethod1.ts
```

Representative source:

```ts
class C {
  /**
   * Returns bar
   */
  public static foo(): string {
    return "bar";
  }
}
```

Compiler evidence:

```text
tokens: ok; public, static, Ident("foo"), LeftParen are present
AST: fails with expected LeftParen, got Some(Ident("foo"))
TypeScript oracle: ok, no diagnostics; MethodDeclaration name is foo
```

## Desired final state

The class member parser recognizes identifier-named methods after accessibility
and `static` modifiers. The representative reference should advance past the
current parser failure to the next diagnostic or pass state.

## Scope

In scope:

- [ ] Parse `public static name(...) { ... }` class method declarations.
- [ ] Parse the matching `private static name(...) { ... }` path if it shares
  the same modifier parser.
- [ ] Preserve existing unmodified static method parsing.
- [ ] Preserve distinct parser paths for modified static accessors, fields, and
  quoted names.
- [ ] Add focused parser/frontend coverage for `class C { public static foo() {} }`.

Out of scope:

- Static method runtime lowering.
- JSDoc/comment emit semantics.
- Modified static accessors, tracked by
  `issues/open/5270-parse-modified-class-accessor-declarations.md`.
- Modified static fields, tracked by
  `issues/open/5271-parse-modified-static-class-fields.md`.
- Quoted static method names, tracked by
  `issues/open/5267-parse-string-literal-class-member-names.md`.

## Affected paths

Expected:

- `crates/frontend/src/parser.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/cli/tests/`
- focused fixtures

Do not touch:

- `crates/backend-wasm/`
- unrelated class runtime semantics

## Acceptance criteria

- [ ] `commentBeforeStaticMethod1.ts` no longer reports `expected LeftParen,
  got Some(Ident("foo"))`.
- [ ] A focused parser test accepts `class C { public static foo(): string { return "bar"; } }`.
- [ ] Existing static method parsing remains unchanged.
- [ ] Existing field/accessor/quoted-name modifier tests remain distinct.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(class) or test(static) or test(method)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentBeforeStaticMethod1.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentBeforeStaticMethod1.ts --detail --no-dashboard-data
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
`issues/done/1339-implement-commentBeforeStaticMethod.md`.

Related but distinct:

- `issues/open/5270-parse-modified-class-accessor-declarations.md`
- `issues/open/5271-parse-modified-static-class-fields.md`
- `issues/open/5267-parse-string-literal-class-member-names.md`

## Completion evidence

Fill when implemented.
