---
id: 408
title: "Implement tsgo declaration emit: AsConstSatisfies/const generic method cases"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P2
depends_on: [399]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Handle two tsgo `declaration-emit`-classified cases by supporting `declaration`-style constructs used in:

- `declarationEmitAsConstSatisfiesNonReadonlyResult.ts`
- `declarationEmitConstObjectLiteralGenericMethod1.ts`

## Problem

These cases fail in `tsgo` coverage with `UnsupportedSyntax: declaration-emit` and block progress on closing the remaining declaration-emit work in that suite.

## Current failure

```sh
mise run reference-coverage -- tsgo --limit 166 --detail --no-web-ui | rg 'declarationEmitAsConstSatisfiesNonReadonlyResult|declarationEmitConstObjectLiteralGenericMethod1'
```

## Desired final state

- Both cases are no longer classified as `declaration-emit` unsupported.
- Parser/emit behavior for these declaration forms is implemented with erased or skipped declaration emission.

## Scope

In scope:

- [ ] Add parsing support for the declaration modifiers/constructs referenced by the two fixtures.
- [ ] Implement compiler handling so these declarations are accepted and safely handled by emit/erasure path.
- [ ] Add focused parser/build fixture assertions matching the two tsgo cases.
- [ ] Update affected docs/comments in fixtures or parser tests if behavior is normalized.

Out of scope:

- Runtime semantics for type system-only declarations.
- Broad `declaration-emit` refactoring not directly related to these cases.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/parser.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/ir/src/lowered.rs`
- `crates/ir/src/lowered/`

Do not touch:

- `crates/runtime-abi/`

## Acceptance criteria

- [ ] `reference/typescript-go/testdata/tests/cases/compiler/declarationEmitAsConstSatisfiesNonReadonlyResult.ts` no longer reports `UnsupportedSyntax: declaration-emit`.
- [ ] `reference/typescript-go/testdata/tests/cases/compiler/declarationEmitConstObjectLiteralGenericMethod1.ts` no longer reports `UnsupportedSyntax: declaration-emit`.
- [ ] Focused coverage command confirms both cases move to `build_pass` or expected non-blocked state.

## Validation

Required commands:

```sh
mise run reference-coverage -- tsgo --path-filter declarationEmitAsConstSatisfiesNonReadonlyResult.ts,declarationEmitConstObjectLiteralGenericMethod1.ts --limit 166 --no-web-ui
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
