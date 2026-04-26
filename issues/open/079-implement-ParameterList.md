---
id: 079
title: "Implement Parameterlist"
type: feature
area: frontend
class: design-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement ParameterList to handle 6 failing test cases in reference tests.

## Problem

Reference test results show 6 cases fail in directory `ParameterList` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

ParameterList is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for ParameterList
- [ ] Add fixtures for ParameterList behavior
- [ ] Update diagnostics appropriately

Out of scope:

- [ ] Related features (separate issues)

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/`
- `crates/backend-wasm/`

## Acceptance criteria

- [ ] ParameterList passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for ParameterList
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
scripts/run/reference-coverage.sh tsc --limit 12
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none

## Notes

## Affected test files

- `reference/TypeScript/tests/cases/compiler/ParameterList13.ts`
- `reference/TypeScript/tests/cases/compiler/ParameterList4.ts`
- `reference/TypeScript/tests/cases/compiler/ParameterList5.ts`
- `reference/TypeScript/tests/cases/compiler/ParameterList6.ts`
- `reference/TypeScript/tests/cases/compiler/ParameterList7.ts`
- `reference/TypeScript/tests/cases/compiler/ParameterList8.ts`

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
