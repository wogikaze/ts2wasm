---
id: 5204
title: "Resolve lexical super property captures in super call arguments"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Resolve `super` property access captured inside an arrow function passed to a
derived constructor's `super(...)` call, such as `super(() => super.blah())`.

## Problem

`captureSuperPropertyAccessInSuperCall01.ts` tokenizes and parses successfully,
including the derived class, constructor, `super(...)` call, and arrow
argument. The pipeline later fails with `UnresolvedName: unresolved name:
this`, which hides the fact that TypeScript accepts this lexical `super`
property capture with no diagnostics.

Problem: lexical `super` property access inside arrow arguments to `super(...)` is not resolved against the derived instance context.

## Current failure

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/captureSuperPropertyAccessInSuperCall01.ts
```

Current diagnostic:

```text
error: [UnresolvedName] unresolved name: `this`
```

Representative source:

```ts
class A {
    constructor(f: () => string) {}
    public blah(): string { return ""; }
}

class B extends A {
    constructor() {
        super(() => super.blah());
    }
}
```

Triage evidence:

- Tokens and AST succeed.
- AST contains `ClassDecl B extends A`.
- Constructor body contains `Call(Ident super, [ArrowFn body:
  Call(Member(Ident super, "blah"))])`.
- TypeScript oracle reports no diagnostics.

## Desired final state

The resolver/lowering pipeline recognizes that `super.blah()` inside the arrow
argument is a lexical super property access tied to the derived constructor
context. The representative file no longer fails with an unresolved synthetic
`this` name.

## Scope

In scope:

- [x] Preserve derived-constructor context while resolving arrow functions
  passed as `super(...)` arguments
- [x] Resolve `super.method()` in that arrow body against the base class method
  surface
- [x] Avoid emitting unresolved synthetic `this` for the lexical super access
- [x] Preserve existing diagnostics for invalid `super` use outside class or
  derived-constructor contexts

Out of scope:

- Full runtime closure capture of arbitrary `super` property references
- Dynamic `super[expr]` property access
- General class runtime parity beyond this captured lexical `super` shape

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/backend-wasm/` unless resolver/lowering already exposes a supported
  class method call representation
- unrelated superclass construction semantics

## Acceptance criteria

- [x] `captureSuperPropertyAccessInSuperCall01.ts` no longer reports
  `UnresolvedName: unresolved name: this`
- [x] A focused fixture covers `class B extends A { constructor() { super(() =>
  super.blah()); } }`
- [x] Invalid `super` use outside a class/derived constructor still reports a
  diagnostic
- [x] The diagnostic or supported path is source-spanned at the `super.blah()`
  expression if full support is deferred

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(class) | test(super)'
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/captureSuperPropertyAccessInSuperCall01.ts
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

Related broad buckets such as `issues/open/420-implement-call-expression.md` and
`issues/open/1481-implement-constructorWithCapturedSuper.md` are still
generated/blocked buckets. This issue owns the exact already-parsed
`captureSuperPropertyAccessInSuperCall01.ts` unresolved-`this` blocker.

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
