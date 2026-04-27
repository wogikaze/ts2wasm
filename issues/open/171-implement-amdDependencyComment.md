---
id: 171
title: "Implement Amddependencycomment"
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

Implement amdDependencyComment to handle 2 failing test cases in reference tests.

## Problem

Reference test results show 2 cases fail in directory `amdDependencyComment` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

amdDependencyComment is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for amdDependencyComment
- [ ] Add fixtures for amdDependencyComment behavior
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

- [ ] amdDependencyComment passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for amdDependencyComment
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

- `reference/TypeScript/tests/cases/compiler/amdDependencyComment1.ts`
- `reference/TypeScript/tests/cases/compiler/amdDependencyComment2.ts`

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
