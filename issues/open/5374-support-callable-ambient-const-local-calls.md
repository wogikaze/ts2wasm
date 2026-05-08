---
id: 5374
title: "Support callable ambient const local calls"
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

Support direct calls to ambient `declare const` values whose TypeScript-only type
annotation is a callable function signature, or classify them before lowering
reaches the generic `issue-211` function-valued local call diagnostic.

## Problem

`contextualTypeSelfReferencing.ts` now parses the ambient generic callable const
declaration and the call expression:

```ts
declare const parse: <def>(def: narrow<def>) => def;

const result = parse([{ a: "foo" }]);
```

The frontend preserves `parse` as an ambient value binding, but lowering treats
the direct call as an unsupported function-valued local call:

```text
UnsupportedSyntax: issue-211: function-valued local calls such as extracted method `parse(...)` are not supported; call receiver.method(...) directly at 339..360
```

TypeScript accepts the file with no diagnostics. This is narrower than callable
interface locals and conditional callable parameters: the callable value is an
ambient `declare const` with a generic function type annotation.

Problem: callable ambient const locals with generic call signatures currently
fall into the generic issue-211 function-valued local call boundary.

## Current failure

```sh
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypeSelfReferencing.ts
```

Current diagnostic:

```text
UnsupportedSyntax: issue-211: function-valued local calls such as extracted method `parse(...)` are not supported
span: 339..360
line 17, column 16
feature_label: method-call
```

Compiler evidence:

- tokens: ok
- ast: ok; representative AST includes `AmbientValueDecl parse` and
  `Let result = Call(Ident parse, Array[Object { a: "foo" }])`
- visible symbols include ambient binding `parse` and local binding `result`
- resolved/lowered: fails in `lower_program` at the generic issue-211
  function-valued local call boundary
- TypeScript oracle: ok, diagnostics `[]`

## Desired final state

The representative ambient callable local call no longer reports the generic
issue-211 extracted-method diagnostic. The compiler either supports the direct
call in the TypeScript-only ambient-callable shape or emits a more precise
source-spanned diagnostic that distinguishes ambient callable declarations from
unsupported extracted methods.

## Scope

In scope:

- [ ] Preserve enough callable metadata for `declare const parse: <T>(...) => T`
  bindings after frontend parsing and name resolution.
- [ ] Classify `parse([{ a: "foo" }])` before the generic extracted-method
  issue-211 path.
- [ ] Keep arbitrary function-valued local calls and extracted method calls on
  their existing unsupported diagnostic path.
- [ ] Add focused coverage for a generic ambient const callable local call.

Out of scope:

- Full runtime support for arbitrary function-valued locals.
- Callable interface-typed locals, tracked by
  `issues/done/5195-support-callable-interface-typed-local-calls.md`.
- Callable conditional-typed parameters, tracked by
  `issues/done/5196-support-callable-conditional-typed-parameter-calls.md`.
- Definite-assignment diagnostics for uninitialized function-typed locals,
  tracked by
  `issues/done/5279-report-function-typed-local-call-definite-assignment.md`.
- Method receiver semantics for `obj.method()` or extracted class methods.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/backend-wasm/` unless lowering already exposes a supported callable
  representation
- broad method-call receiver lowering

## Acceptance criteria

- [ ] `contextualTypeSelfReferencing.ts` no longer reports generic issue-211 for
  `parse([{ a: "foo" }])`.
- [ ] A focused fixture covers `declare const parse: <T>(value: T) => T;` and a
  direct `parse(...)` call.
- [ ] The diagnostic or generated behavior is source-spanned at the `parse`
  call-site identifier.
- [ ] Existing issue-211 extracted method/function-valued local fixtures keep
  their established unsupported diagnostics.
- [ ] Callable interface local and conditional parameter cases remain tracked by
  issues 5195 and 5196 rather than being silently reclassified by this slice.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(function)'
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypeSelfReferencing.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeSelfReferencing.ts --detail --no-dashboard-data
```

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from `issues/open/1515-implement-contextualTypeSelfReferencing.md`.
Related broad method-call bucket: `issues/open/435-implement-method-call.md`.

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
