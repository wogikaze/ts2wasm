---
id: 172
title: "Implement Amddependencycommentname"
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

Implement amdDependencyCommentName to handle 4 failing test cases in reference tests.

## Problem

Reference test results show 4 cases fail in directory `amdDependencyCommentName` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

amdDependencyCommentName is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for amdDependencyCommentName
- [ ] Add fixtures for amdDependencyCommentName behavior
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

- [ ] amdDependencyCommentName passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for amdDependencyCommentName
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
scripts/manager reference-coverage tsc --limit 8
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

- `reference/TypeScript/tests/cases/compiler/amdDependencyCommentName1.ts`
- `reference/TypeScript/tests/cases/compiler/amdDependencyCommentName2.ts`
- `reference/TypeScript/tests/cases/compiler/amdDependencyCommentName3.ts`
- `reference/TypeScript/tests/cases/compiler/amdDependencyCommentName4.ts`

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
