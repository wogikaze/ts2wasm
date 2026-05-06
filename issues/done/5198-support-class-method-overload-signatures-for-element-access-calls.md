---
id: 5198
title: "Support class method overload signatures for element access calls"
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

Allow TypeScript class method overload signatures to merge with their concrete
implementation before resolving calls through element access, such as
`c["foo"](1)`.

## Problem

`callOverloadViaElementAccessExpression.ts` parses successfully, including the
element-access callees, but resolver rejects the overload declaration group as
duplicate class methods before it can type-check either call. TypeScript accepts
the overload declarations and then reports the intended assignment diagnostics
for the call results.

Problem: class method overload signatures are currently treated as duplicate method definitions.

## Current failure

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOverloadViaElementAccessExpression.ts
```

Current diagnostic:

```text
error: [DuplicateFunction] duplicate method definition: `C.foo`
```

Representative source:

```ts
class C {
    foo(x: number): number;
    foo(x: string): string;
    foo(x: any): any {
        return null;
    }
}
var c = new C();
var r: string = c["foo"](1);
var r2: number = c["foo"]("");
```

Triage evidence:

- Tokens and AST succeed.
- AST contains three class members named `foo`; the first two are bodyless
  overload signatures and the third is the implementation.
- AST contains `Let r = Call(Index(Ident c, String "foo"), Number 1)` and
  `Let r2 = Call(Index(Ident c, String "foo"), String "")`.
- Visible symbols include class `C`, binding `c = new C()`, and bindings `r`
  and `r2`.
- TypeScript oracle reports TS2322 for both assignments: the `number` result is
  not assignable to `string`, and the `string` result is not assignable to
  `number`.

## Desired final state

The resolver recognizes bodyless class method declarations as overload
signatures and merges them with the implementation instead of emitting
`DuplicateFunction`. The representative file then reaches the intended
element-access overload call behavior, either producing TypeScript-compatible
assignment diagnostics or a narrower follow-up diagnostic after the duplicate
method blocker is removed.

## Scope

In scope:

- [x] Distinguish bodyless class method overload signatures from duplicate
  concrete method implementations
- [x] Merge overload signatures with the following concrete implementation for
  the same method name
- [x] Preserve duplicate-method diagnostics for multiple concrete
  implementations
- [x] Keep the element-access call expression `c["foo"](...)` source-spanned
  after overload merging

Out of scope:

- Full overload-resolution ranking across arbitrary expressions
- Declaration emit for overload groups
- Interface or namespace overload merging not exercised by this reference path

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/backend-wasm/` unless a resolver change reveals an existing supported lowering path
- unrelated duplicate-function handling for plain JavaScript redeclarations

## Acceptance criteria

- [x] `callOverloadViaElementAccessExpression.ts` no longer reports
  `DuplicateFunction` for `C.foo`
- [x] A focused fixture covers a class with two overload signatures plus one
  implementation and calls through `instance["method"](...)`
- [x] Duplicate concrete class methods still report a duplicate-method
  diagnostic
- [x] The resulting diagnostic or supported behavior is source-spanned at the
  element-access call or assignment being checked

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(class)'
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOverloadViaElementAccessExpression.ts
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

This is not the same as the plain JavaScript `duplicate-function` bucket: this
path is a TypeScript class overload declaration group whose first two members
have no body.

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

Date: 2026-05-07

Classification: truly-done.

Audit result: retained in issues/done/. Implementation commits confirmed.
