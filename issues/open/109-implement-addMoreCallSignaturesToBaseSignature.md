---
id: 109
title: "Implement Addmorecallsignaturestobasesignature"
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

Implement addMoreCallSignaturesToBaseSignature to handle 2 failing test cases in reference tests.

## Problem

Reference test results show 2 cases fail in directory `addMoreCallSignaturesToBaseSignature` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

addMoreCallSignaturesToBaseSignature is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for addMoreCallSignaturesToBaseSignature
- [ ] Add fixtures for addMoreCallSignaturesToBaseSignature behavior
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

- [ ] addMoreCallSignaturesToBaseSignature passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for addMoreCallSignaturesToBaseSignature
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
scripts/manager reference-coverage tsc --limit 4
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

- `reference/TypeScript/tests/cases/compiler/addMoreCallSignaturesToBaseSignature.ts`
- `reference/TypeScript/tests/cases/compiler/addMoreCallSignaturesToBaseSignature2.ts`

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
