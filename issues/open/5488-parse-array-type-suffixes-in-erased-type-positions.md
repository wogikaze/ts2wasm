---
id: 5488
title: "Parse array type suffixes in erased type positions"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: [3595]
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Teach the TypeScript erasure parser to treat `T[]` as type syntax in erased
type positions, including generic constraints, rest parameter annotations,
function return types, and direct generic call type arguments.

## Problem

`nonInferrableTypePropagation3.ts` currently fails before semantic type
propagation. The parser sees `any[]` in a type-only generic constraint and
reports the value-expression empty element access diagnostic.

Problem: array type suffixes such as `any[]`, `Args[]`, and object literal
array types inside erased annotations can be misparsed as empty element access
expressions.

## Current failure

Focused triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonInferrableTypePropagation3.ts
```

Observed result:

```text
diagnostic: UnsupportedSyntax
message: issue-5150: empty element access `expr[]` requires an index expression
source: declare type Callback<Args extends any[], Out, R> = (...args: Args) => (data: Out) => R;
typescript oracle: ok; diagnostics=[]
```

Representative syntax:

```ts
declare type Callback<Args extends any[], Out, R> =
  (...args: Args) => (data: Out) => R;

declare function factory<Out>():
  <Args extends any[], R>(callback: Callback<Args, Out, R>) => (...args: Args) => R;

const make = factory<{id: string, age: number}[]>();
```

## Desired final state

Type-only array suffixes are erased without entering value-expression element
access parsing. The representative file advances past the current issue-5150
diagnostic and exposes the next parser, resolver, or semantic blocker.

## Scope

In scope:

- [ ] Parse/skip `any[]` and `Args[]` in generic type parameter constraints.
- [ ] Parse/skip rest parameter annotations such as `...args: Args` in erased
  function type aliases.
- [ ] Parse/skip generic function return type annotations containing nested
  generic constraints and array suffixes.
- [ ] Parse/skip direct generic call type arguments containing object literal
  array types, such as `factory<{id: string, age: number}[]>()`.
- [ ] Preserve issue-5150 diagnostics for true value-position empty element
  access expressions such as `number[]` as an expression.

Out of scope:

- Non-inferrable type propagation semantics.
- Runtime array behavior or array builtins.
- General tuple/rest type computation beyond erasing the syntax.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/parser/tests.rs`
- focused parser or CLI fixture

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [ ] `nonInferrableTypePropagation3.ts` no longer reports issue-5150 for
  `Args extends any[]`.
- [ ] A focused parser test accepts `declare type C<Args extends any[]> =
  (...args: Args) => void;`.
- [ ] A focused parser test accepts
  `factory<{id: string, age: number}[]>();` as an expression with erased generic
  type arguments.
- [ ] A focused negative test or existing fixture confirms true value-position
  empty element access still reports issue-5150.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonInferrableTypePropagation3.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonInferrableTypePropagation3.ts
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

Split from `issues/done/3595-implement-nonInferrableTypePropagation-type-system.md`.
The completed issue 5150 owns the diagnostic for true empty element access in
value position; this issue owns preventing type-only `T[]` syntax from reaching
that value-position diagnostic.

Related but not duplicate:

- `issues/open/5309-skip-generic-type-arguments-in-type-annotations.md`
  covers comma handling inside generic type argument lists.
- `issues/open/5150-report-empty-element-access-diagnostics.md` covers the
  value-position diagnostic.
- `issues/open/5345-parse-generic-ambient-const-type-annotations.md` covers
  ambient const annotations such as `declare const es: Either<string, number>[];`.

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

- After this parser boundary advances, the fixture may expose the intended
  non-inferrable type propagation semantic gap.
