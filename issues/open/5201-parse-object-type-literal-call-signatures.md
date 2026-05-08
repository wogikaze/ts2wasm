---
id: 5201
title: "Parse object type literal call signatures"
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

Parse TypeScript object type literal call signatures in variable annotations,
including overloaded signatures with string-literal parameter types.

## Problem

`callSignatureFunctionOverload.ts` tokenizes successfully, but AST construction
reports `unterminated TypeScript type annotation` after seeing an object type
literal whose members are call signatures. TypeScript accepts the reference file
and infers callable object type annotations for both variables.

Problem: TypeScript object type literals with call-signature members are not parsed as complete type annotations.

## Current failure

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callSignatureFunctionOverload.ts
```

Current diagnostic:

```text
error: [UnsupportedTypeScriptSyntax] unterminated TypeScript type annotation at 288..289
```

Representative source:

```ts
var foo: {
    (name: string): string;
    (name: 'order'): string;
    (name: 'content'): string;
    (name: 'done'): string;
}
```

Triage evidence:

- Tokenization succeeds and emits the `{`, `(`, parameter name/type, return
  type, semicolon, and closing `}` tokens for the call-signature members.
- AST construction fails before producing a module AST.
- Visible-symbol extraction sees bindings `foo` and `foo2` before the failure.
- TypeScript oracle reports no diagnostics and hints both variables as object
  types containing multiple call signatures.

## Desired final state

The parser consumes object type literal annotations containing call-signature
members and preserves enough TypeScript-only type metadata for later resolver
or lowering slices. The representative file no longer fails with an
unterminated type annotation.

## Scope

In scope:

- [x] Parse object type literal annotations after `var name:`
- [x] Parse call-signature members of the form `(param: Type): ReturnType;`
- [x] Accept string-literal parameter types in those call signatures
- [x] Preserve or erase the parsed type metadata consistently with existing
  TypeScript annotation handling

Out of scope:

- Runtime support for callable object values
- Full overload resolution for callable object types
- Interface call signatures already tracked by callable-interface issues

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/backend-wasm/`
- unrelated resolver/lowering call semantics

## Acceptance criteria

- [x] `callSignatureFunctionOverload.ts` no longer reports `unterminated
  TypeScript type annotation`
- [x] A focused parser fixture covers a variable annotation with an object type
  literal containing at least two call signatures
- [x] A focused parser fixture covers a call signature parameter typed as a
  string literal, such as `'order'`
- [x] Existing object literal expression parsing remains unchanged

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(parser) | test(type)'
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callSignatureFunctionOverload.ts
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

This issue is parser-owned. Later support for calling values with callable
object types should stay separate from this syntax unblock.

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

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in issues/open/. Implementation commits confirmed.
