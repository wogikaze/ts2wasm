---
id: 5384
title: "Resolve ambient function value references"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Resolve `declare function` bindings when they are used as function values, for
example as callback arguments in another call expression.

## Problem

Problem: `contextualTypingReturnStatementWithReturnTypeAnnotation.ts` currently
reports `UnresolvedName` for `isString` even though `isString` is a visible
ambient function declaration.

The reference parses successfully and the smart triage visible-symbol list
includes both `isString` and `getSpecsFromRaw`. Resolution still fails inside
the return statement when `isString` is passed as the second argument to
`getPropFromRaw(...)`:

```text
error: [UnresolvedName] unresolved name: `isString`
```

TypeScript accepts the reference with no diagnostics.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingReturnStatementWithReturnTypeAnnotation.ts
```

Representative source:

```ts
declare function isString(text: unknown): text is string;

declare function getPropFromRaw<T>(
  prop: "files" | "include" | "exclude" | "references",
  validateElement: (value: unknown) => boolean,
  elementTypeName: string
): PropOfRaw<T>;

function getSpecsFromRaw(
  prop: "files" | "include" | "exclude"
): PropOfRaw<string> {
  return getPropFromRaw(prop, isString, "string");
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; ambient functions `isString` and `getPropFromRaw`, then
  `Return(Call(Ident getPropFromRaw, [Ident prop, Ident isString, "string"]))`
visible symbols: function `isString`, function `getSpecsFromRaw`
resolved/lowered: UnresolvedName for `isString`
TypeScript oracle: ok, diagnostics []
```

## Desired final state

The resolver treats ambient `declare function` declarations as value bindings
when referenced in expression positions. The representative reference no longer
reports `UnresolvedName` for `isString` and advances to build-pass or the next
more specific unsupported boundary.

## Scope

In scope:

- [ ] Bind top-level ambient `declare function` declarations in the value
  namespace used by expression references.
- [ ] Resolve an ambient function identifier when passed as a callback argument.
- [ ] Preserve existing declaration erasure for emitted runtime code.
- [ ] Add focused coverage for `declare function pred(x: unknown): boolean;`
  followed by `use(pred)`.

Out of scope:

- Implementing full TypeScript type predicates or control-flow narrowing.
- Runtime support for arbitrary external ambient function calls.
- Generic return type inference for `getPropFromRaw<T>`.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- focused fixtures

Do not touch:

- `crates/backend-wasm/` unless a focused resolver fixture proves lowering now
  produces a supported callable representation.
- Unrelated test262 metadata name-resolution issues.

## Acceptance criteria

- [ ] `contextualTypingReturnStatementWithReturnTypeAnnotation.ts` no longer
  reports `UnresolvedName` for `isString`.
- [ ] A focused test covers passing an ambient declared function as a callback
  value.
- [ ] Type-only ambient declarations remain erased and do not create runtime
  values.
- [ ] Ordinary unresolved identifier diagnostics still report unresolved names.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(name) or test(resolve) or test(ambient)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingReturnStatementWithReturnTypeAnnotation.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingReturnStatementWithReturnTypeAnnotation.ts --detail --no-dashboard-data
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
`issues/done/1530-implement-contextualTypingReturnStatementWithReturnTypeAnnotation.md`.

Related but distinct:

- `issues/open/064-implement-name-resolution.md` is a superseded test262
  metadata bucket and does not own this TypeScript ambient function value
  reference.
- `issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`
  owns ambient value declarations such as `declare const`, not
  `declare function` value references.

## Completion evidence

Fill when implemented.
