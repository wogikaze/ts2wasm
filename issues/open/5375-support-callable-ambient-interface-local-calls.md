---
id: 5375
title: "Support callable ambient interface local calls"
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

Support direct calls to ambient `declare let` locals whose TypeScript-only
annotation is an interface with a call signature, or classify that shape before
lowering reaches the generic `issue-211` function-valued local call diagnostic.

## Problem

`contextualTypesNegatedTypeLikeConstraintInGenericMappedType1.ts` parses the
ambient callable interface binding and direct call:

```ts
declare let TabGroup: _internal_ComponentTabGroup;

TabGroup({
  defaultIndex: 0,
  onChange: (index) => {
    const i: number = index;
  },
});
```

Lowering treats the ambient callable interface value as an unsupported
function-valued local call:

```text
UnsupportedSyntax: issue-211: function-valued local calls such as extracted method `TabGroup(...)` are not supported
```

TypeScript accepts the file with diagnostics `[]` and infers the contextual
`onChange` callback parameter as `number`.

Problem: ambient locals typed by callable interfaces currently fall into the
generic issue-211 function-valued local call boundary.

## Current failure

```sh
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypesNegatedTypeLikeConstraintInGenericMappedType1.ts
```

Current diagnostic:

```text
UnsupportedSyntax: issue-211: function-valued local calls such as extracted method `TabGroup(...)` are not supported at 828..919
```

Compiler evidence:

- tokens: ok, including type aliases and callable interface declarations
- ast: ok; representative AST includes `AmbientValueDecl TabGroup` and
  `Call(Ident TabGroup, Object { defaultIndex, onChange })`
- visible symbols include `DEFAULT_TABS_TAG` and `TabGroup`
- resolved/lowered: fails in `lower_program` at the generic issue-211
  function-valued local call boundary
- TypeScript oracle: ok, diagnostics `[]`; callback parameter `index` is
  inferred as `number`

## Desired final state

The representative `TabGroup(...)` calls no longer report the generic issue-211
extracted-method diagnostic. The compiler either supports direct calls to
ambient callable-interface declarations or emits a more precise source-spanned
diagnostic for this ambient type-only callable shape.

## Scope

In scope:

- [ ] Preserve enough callable-interface metadata for ambient `declare let`
  bindings after frontend parsing and name resolution.
- [ ] Classify `TabGroup({ ... })` before the generic extracted-method
  issue-211 path.
- [ ] Keep non-ambient callable interface local behavior tracked separately by
  issue 5195.
- [ ] Add focused coverage for a callable interface ambient binding and direct
  call.

Out of scope:

- Full TypeScript contextual typing or mapped-type evaluation.
- Non-ambient callable interface locals, tracked by
  `issues/open/5195-support-callable-interface-typed-local-calls.md`.
- Generic ambient const function calls, tracked by
  `issues/open/5374-support-callable-ambient-const-local-calls.md` and
  `issues/open/5376-support-ambient-generic-factory-local-calls.md`.
- The mapped-type key-remap variant, tracked by
  `issues/open/5377-support-callable-ambient-interface-local-calls-with-key-remap.md`.
- Runtime support for arbitrary extracted method/function-valued locals.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- broad method-call receiver lowering
- backend callable dispatch unless lowering already exposes a supported
  callable representation

## Acceptance criteria

- [ ] `contextualTypesNegatedTypeLikeConstraintInGenericMappedType1.ts` no
  longer reports generic issue-211 for `TabGroup(...)`.
- [ ] A focused fixture covers `declare let value: CallableInterface;` where
  the interface has a generic call signature and `value(...)` is called.
- [ ] Existing issue-211 extracted method/function-valued local fixtures keep
  their established unsupported diagnostics.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(function)'
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypesNegatedTypeLikeConstraintInGenericMappedType1.ts
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
