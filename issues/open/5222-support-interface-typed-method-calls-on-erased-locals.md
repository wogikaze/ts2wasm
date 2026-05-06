---
id: 5222
title: "Support interface-typed method calls on erased locals"
type: feature
area: ir/lowering
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Support or precisely diagnose method calls on locals annotated with TypeScript
interface types, such as `var s: Sequence<string>; s.groupBy(...)`.

## Problem

`chainedSpecializationToObjectTypeLiteral.ts` tokenizes and parses successfully,
including the `Sequence<T>` interface declaration and the `s.groupBy(...)` /
`s2.each(...)` call expressions. Lowering then rejects the first interface
method call because the receiver `s` is an erased, uninitialized local rather
than a known runtime class instance.

Current diagnostic:

```text
UnsupportedSyntax: issue-211: unknown receiver class for method `groupBy` at 326..350
```

## Current failure

Use the validation command below to reproduce the representative failure.

Source shape:

```ts
interface Sequence<T> {
    groupBy<K>(keySelector: (value: T) => K): Sequence<{ key: K; items: T[]; }>;
}

var s: Sequence<string>;
var s2 = s.groupBy(s => s.length);
var s3 = s2.each(x => { x.key });
```

Compiler evidence:

```text
tokens: ok
ast: ok; Let s = Undefined, Let s2 = Call(Member(Ident s, "groupBy"), ...)
resolved/lowered: issue-211 unknown receiver class for method `groupBy`
TypeScript oracle: TS2454 Variable 's' is used before being assigned; hints type s2 as Sequence<{ key: number; items: string[]; }>
```

## Desired final state

The compiler no longer reports the generic unknown receiver class diagnostic
for interface-typed method calls on erased locals. The representative path
either reaches a source-spanned definite-assignment/type diagnostic for `s` or
supports the interface method-call shape enough to expose the next semantic
blocker.

## Scope

In scope:

- [ ] Preserve enough interface method metadata for locals annotated as `Sequence<T>`.
- [ ] Classify `s.groupBy(...)` before the generic unknown receiver class diagnostic.
- [ ] Handle a method return type containing an object type literal, `Sequence<{ key: K; items: T[]; }>`, without regressing ordinary object literal expressions.

Out of scope:

- Full TypeScript generic specialization or assignability.
- Runtime implementation of arbitrary erased interface values.
- Callable interface local calls covered by issue 5195.
- Parser support for object type literal call signatures covered by issue 5201.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/frontend/src/`
- `crates/cli/tests/`
- focused fixtures

Do not touch:

- `crates/backend-wasm/` unless lowering already produces a supported runtime representation
- unrelated array or class method builtins

## Acceptance criteria

- [ ] `chainedSpecializationToObjectTypeLiteral.ts` no longer reports the `unknown receiver class for method groupBy` diagnostic.
- [ ] A focused fixture covers `var s: Sequence<string>; s.groupBy(...)`.
- [ ] A focused fixture or reference assertion covers the object type literal return shape `Sequence<{ key: K; items: T[]; }>`.
- [ ] Existing direct class-method and array-method call behavior remains unchanged.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(method) or test(interface) or test(type)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/chainedSpecializationToObjectTypeLiteral.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/chainedSpecializationToObjectTypeLiteral.ts --detail
```

Not run:

- none

## Notes

Split from generated bucket `issues/done/1128-implement-chainedSpecializationToObjectTypeLiteral.md`.

Issue 5195 handles direct calls to callable interface-typed locals such as
`i("")`; this issue handles method calls through interface-typed receivers such
as `s.groupBy(...)`.

Additional superseded bucket:

- `issues/done/1370-implement-commentsInterface.md` reaches the same issue-211
  receiver family for `var i2_i: i2; i2_i.foo(30);`.

## Completion evidence

Fill when implemented.
