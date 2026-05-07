---
id: 5377
title: "Support callable ambient interface local calls with key remap"
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

Support the `TabGroup(...)` ambient callable-interface call after mapped-type
key remapping has been parsed, or classify it before the generic issue-211
function-valued local call diagnostic.

## Problem

`contextualTypesNegatedTypeLikeConstraintInGenericMappedType3.ts` parses a
`MappedOmit<T, K>` type with key remapping and then reaches:

```ts
declare let TabGroup: _internal_ComponentTabGroup;
TabGroup({ defaultIndex: 0, onChange: (index) => { const i: number = index; } });
```

The compiler stops at:

```text
UnsupportedSyntax: issue-211: function-valued local calls such as extracted method `TabGroup(...)` are not supported at 920..1011
```

TypeScript accepts the file with diagnostics `[]` and infers `index: number`.

Problem: callable ambient interface locals after mapped-type key remapping fall
into the generic issue-211 function-valued local call boundary.

## Current failure

```sh
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypesNegatedTypeLikeConstraintInGenericMappedType3.ts
```

Compiler evidence:

- tokens: ok through mapped type key remapping
- ast: ok; includes `AmbientValueDecl TabGroup` and `Call(Ident TabGroup, Object { defaultIndex, onChange })`
- resolved/lowered: generic issue-211 function-valued local call
- TypeScript oracle: ok, diagnostics `[]`

## Scope

In scope:

- [ ] Classify ambient callable-interface `TabGroup(...)` after key-remap type aliases.
- [ ] Add focused coverage that includes a preceding mapped type with `as`.
- [ ] Keep ordinary extracted method/function-valued local issue-211 diagnostics.

Out of scope:

- Full mapped-type evaluation.
- The non-key-remap variant, tracked by issue 5375.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- broad method-call receiver lowering
- mapped-type semantics beyond this first callable ambient local boundary

## Acceptance criteria

- [ ] `contextualTypesNegatedTypeLikeConstraintInGenericMappedType3.ts` no longer reports generic issue-211 for `TabGroup(...)`.
- [ ] A focused fixture covers a preceding mapped type key remap plus an ambient callable interface local call.
- [ ] Existing issue-211 extracted method/function-valued local fixtures keep their established unsupported diagnostics.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(function)'
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypesNegatedTypeLikeConstraintInGenericMappedType3.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypesNegatedTypeLikeConstraintInGenericMappedType3.ts --detail --no-dashboard-data
```

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from `issues/done/1517-implement-contextualTypesNegatedTypeLikeConstraintInGenericMappedType.md`.

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
