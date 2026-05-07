---
id: 5279
title: "Report function-typed local call definite assignment"
type: feature
area: ir/lowering
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-07
---

## Summary

Classify calls to uninitialized locals with TypeScript function type
annotations before lowering reaches the generic issue-211 function-valued local
call boundary.

## Problem

Problem: `commentOnParenthesizedExpressionOpenParen1.ts` parses successfully,
including the type assertion, parenthesized assignment, preserved comment, and
`f()` call. Lowering then reports the generic issue-211 function-valued local
call diagnostic:

```text
UnsupportedSyntax: issue-211: function-valued local calls such as extracted method `f(...)` are not supported; call receiver.method(...) directly at 73..76
```

TypeScript instead reports TS2454, `Variable 'f' is used before being
assigned`, at the same call-site identifier. The implementation should preserve
enough function type annotation information to report a source-spanned
definite-assignment/type diagnostic before the generic method-call boundary.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnParenthesizedExpressionOpenParen1.ts
```

Current diagnostic:

```text
UnsupportedSyntax: issue-211: function-valued local calls such as extracted method `f(...)` are not supported
span: 73..76
line 4, column 30
```

Source context:

```ts
var j;
var f: () => any;
<any>( /* Preserve */ j = f());
```

Compiler evidence:

```text
tokens: ok
ast: ok
Expr Assign name "j" expr Call callee Ident "f"
resolved/lowered: issue-211 at f()
```

TypeScript oracle:

```text
TS2454: Variable 'f' is used before being assigned.
TypeScript AST path:
ExpressionStatement -> TypeAssertionExpression -> ParenthesizedExpression ->
BinaryExpression "j = f()" -> CallExpression "f()" -> Identifier "f"
```

## Scope

In scope:

- [ ] Preserve enough `() => any` function type annotation metadata for `var f: () => any`.
- [ ] Detect calls to uninitialized function-typed locals before the generic issue-211 extracted-method diagnostic.
- [ ] Report a source-spanned diagnostic at the call-site identifier `f`.
- [ ] Add focused coverage for `<any>( /* Preserve */ j = f());`.

Out of scope:

- Runtime support for arbitrary function-valued local calls.
- Callable interface locals, tracked by `issues/open/5195-support-callable-interface-typed-local-calls.md`.
- Conditional callable types, tracked by `issues/open/5196-support-callable-conditional-typed-parameter-calls.md`.
- Method receiver support for `obj.method()`.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- backend method-call receiver lowering
- broad method-call builtin support

## Acceptance criteria

- [ ] `commentOnParenthesizedExpressionOpenParen1.ts` no longer reports generic issue-211 for `f()`.
- [ ] A focused fixture covers `var f: () => any; f();` and reports a source-spanned definite-assignment/type diagnostic at `f`.
- [ ] Type assertion and parenthesized assignment syntax around the call continue to parse.
- [ ] Existing arbitrary extracted method/function-valued local call fixtures still report their established unsupported diagnostics.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli function_typed_local_call
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnParenthesizedExpressionOpenParen1.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnParenthesizedExpressionOpenParen1.ts --detail --no-dashboard-data
```

## Notes

Split from `issues/done/1353-implement-commentOnParenthesizedExpressionOpenParen.md`.
Related broad method-call bucket: `issues/open/435-implement-method-call.md`.

Additional superseded bucket:

- `issues/done/1382-implement-commentsemitComments.md` reaches the same
  function-typed local call definite-assignment boundary at
  `var fooVar: () => void; fooVar();`. Fresh triage on 2026-05-07 reports
  unspanned `UnresolvedFunction` for `fooVar`, while TypeScript reports TS2454
  at the `fooVar` call. Comment emit behavior is not reached before this
  resolver/lowering boundary.
