---
id: 5202
title: "Parse member call explicit type arguments"
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

Parse and erase explicit TypeScript type arguments on member calls such as
`_.map<number, string>(...)`.

## Problem

`callbacksDontShareTypes.ts` tokenizes successfully and parses earlier
ordinary calls like `_.map(c2, rf1)`, but AST construction fails when the call
adds explicit type arguments after the member name. The parser reaches the
`>` in `_.map<number, string>(...)` and reports `expected Semicolon`, so the
reference file cannot reach the intended TypeScript definite-assignment
diagnostics.

Problem: explicit type arguments after member call callees are not parsed or erased.

## Current failure

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callbacksDontShareTypes.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Greater) at 556..557
```

Representative source:

```ts
interface Combinators {
    map<T, U>(c: Collection<T>, f: (x: T) => U): Collection<U>;
    map<T>(c: Collection<T>, f: (x: T) => any): Collection<any>;
}

var _: Combinators;
var c2: Collection<number>;
var r5a = _.map<number, string>(c2, (x) => { return x.toFixed() });
var r5b = _.map<number, string>(c2, rf1);
```

Triage evidence:

- Tokenization succeeds and includes `Member` call tokens around
  `_.map<number, string>(...)`.
- AST construction fails at the closing `>` for the explicit type argument
  list.
- Visible-symbol extraction sees `_`, `c2`, `rf1`, `r1a`, `r1b`, and partial
  `r5a` before the failure.
- TypeScript oracle reports only definite-assignment diagnostics for `_` and
  `c2`, proving the explicit member-call type argument syntax is accepted.

## Desired final state

The parser recognizes explicit TypeScript type argument lists after member
callees and erases or records them consistently with existing direct generic
call parsing. The representative file no longer fails at the `>` token and
continues to resolver/type diagnostics.

## Scope

In scope:

- [x] Parse `object.method<T, U>(args)` as a call expression
- [x] Erase or preserve the parsed type argument list consistently with current
  direct-call type argument handling
- [x] Preserve ordinary relational/comparison parsing for non-call expressions
- [x] Preserve existing plain member calls such as `_.map(c2, rf1)`

Out of scope:

- Full TypeScript overload resolution for generic member methods
- Runtime support for `Collection<T>` or `Combinators`
- Type inference for callback parameter sharing

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/backend-wasm/`
- unrelated resolver/lowering call semantics

## Acceptance criteria

- [x] `callbacksDontShareTypes.ts` no longer reports `expected Semicolon, got
  Some(Greater)` at `_.map<number, string>(...)`
- [x] A focused parser fixture covers `obj.method<number, string>(arg)`
- [x] A focused parser fixture covers ordinary `obj.method(arg)` and keeps the
  existing AST shape
- [x] A focused negative/regression fixture covers a relational expression shape
  that must not be reclassified as a generic member call

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(parser) | test(call)'
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callbacksDontShareTypes.ts
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

Issue 059 already notes a direct generic-call erasure guard. This issue is the
member-callee counterpart and should keep the same ambiguity discipline.

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

Audit result: retained in issues/done/. Implementation commits confirmed.
