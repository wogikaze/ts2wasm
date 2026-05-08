---
id: 5376
title: "Support ambient generic factory local calls"
type: feature
area: ir/lowering
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Support direct calls to ambient generic function values that return another
callable value, or classify them before lowering reaches the generic
`issue-211` function-valued local call diagnostic.

## Problem

`contextualTypesNegatedTypeLikeConstraintInGenericMappedType2.ts` parses the
ambient generic factory declaration and first direct call:

```ts
declare const typeTags: <I>() => <P extends ...>(fields: P) => unknown;

type Value = { _tag: "A"; a: number } | { _tag: "B"; b: number };
const matcher = typeTags<Value>();
```

Lowering rejects the call before TypeScript's intended mapped/conditional type
diagnostic can be reached:

```text
UnsupportedSyntax: issue-211: function-valued local calls such as extracted method `typeTags(...)` are not supported
```

TypeScript specializes `typeTags<Value>()`, types `matcher`, and then reports
the later TS2322 diagnostic on the intentionally invalid `C` field.

Problem: ambient generic factory locals currently fall into the generic
issue-211 function-valued local call boundary before their result can be
classified.

## Current failure

```sh
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypesNegatedTypeLikeConstraintInGenericMappedType2.ts
```

Current diagnostic:

```text
UnsupportedSyntax: issue-211: function-valued local calls such as extracted method `typeTags(...)` are not supported
span: 471..488
line 18, column 17
feature_label: type-system
```

Compiler evidence:

- tokens: ok, including conditional type `infer`, mapped type, and generic
  ambient function syntax
- ast: ok; representative AST includes `AmbientValueDecl typeTags`,
  `Let matcher = Call(Ident typeTags, [])`, and later `matcher({ ... })` calls
- visible symbols include ambient binding `typeTags` and local binding `matcher`
- resolved/lowered: fails in `lower_program` at the generic issue-211
  function-valued local call boundary
- TypeScript oracle: reaches TS2322 after typing `matcher = typeTags<Value>()`

## Desired final state

The representative `typeTags<Value>()` call no longer reports the generic
issue-211 extracted-method diagnostic. The compiler either supports the ambient
generic factory call enough to type/classify `matcher`, or emits a more precise
source-spanned diagnostic for this ambient factory-call shape.

## Scope

In scope:

- [ ] Preserve enough callable metadata for an ambient generic function value
  whose return type is another callable function type.
- [ ] Classify `typeTags<Value>()` before the generic extracted-method
  issue-211 path.
- [ ] Add focused coverage for an ambient generic zero-argument factory call
  assigned to a local.
- [ ] Keep arbitrary function-valued local calls and extracted method calls on
  their existing unsupported diagnostic path.

Out of scope:

- Full mapped-type/conditional-type evaluation after the factory call advances.
- Direct runtime support for arbitrary higher-order function values.
- Ambient callable interface values, tracked by
  `issues/open/5375-support-callable-ambient-interface-local-calls.md`.
- Direct ambient callable const calls with runtime arguments, tracked by
  `issues/open/5374-support-callable-ambient-const-local-calls.md`.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- broad method-call receiver lowering
- mapped-type semantics beyond the first callable factory boundary

## Acceptance criteria

- [ ] `contextualTypesNegatedTypeLikeConstraintInGenericMappedType2.ts` no
  longer reports generic issue-211 for `typeTags<Value>()`.
- [ ] A focused fixture covers `declare const make: <T>() => (value: T) => T;`
  and `const f = make<number>();`.
- [ ] The diagnostic or generated behavior is source-spanned at the `typeTags`
  call-site identifier.
- [ ] Existing issue-211 extracted method/function-valued local fixtures keep
  their established unsupported diagnostics.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(function)'
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypesNegatedTypeLikeConstraintInGenericMappedType2.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypesNegatedTypeLikeConstraintInGenericMappedType --detail --no-dashboard-data
```

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from
`issues/open/1517-implement-contextualTypesNegatedTypeLikeConstraintInGenericMappedType.md`.

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
