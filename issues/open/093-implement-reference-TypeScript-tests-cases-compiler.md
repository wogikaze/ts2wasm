---
id: 093
title: "Implement Compiler (parser-syntax)"
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

Implement support for reference/TypeScript/tests/cases/compiler to handle 195 failing test cases in reference tests.

## Problem

Reference test results show 195 cases fail in directory `reference/TypeScript/tests/cases/compiler` with diagnostics: equality-operator, function, name-resolution, parser-syntax, unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

reference/TypeScript/tests/cases/compiler is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for reference/TypeScript/tests/cases/compiler
- [ ] Add fixtures for reference/TypeScript/tests/cases/compiler behavior
- [ ] Update diagnostics appropriately

Out of scope:

- [ ] Related directories (separate issues)

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/`
- `crates/backend-wasm/`

## Acceptance criteria

- [ ] reference/TypeScript/tests/cases/compiler passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for reference/TypeScript/tests/cases/compiler
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
scripts/run/reference-coverage.sh tsc --limit 390
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

- `reference/TypeScript/tests/cases/compiler/2dArrays.ts`
- `reference/TypeScript/tests/cases/compiler/APILibCheck.ts`
- `reference/TypeScript/tests/cases/compiler/APISample_Watch.ts`
- `reference/TypeScript/tests/cases/compiler/APISample_WatchWithDefaults.ts`
- `reference/TypeScript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts`
- `reference/TypeScript/tests/cases/compiler/APISample_compile.ts`
- `reference/TypeScript/tests/cases/compiler/APISample_jsdoc.ts`
- `reference/TypeScript/tests/cases/compiler/APISample_linter.ts`
- `reference/TypeScript/tests/cases/compiler/APISample_parseConfig.ts`
- `reference/TypeScript/tests/cases/compiler/APISample_transform.ts`
- ... and 185 more files

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
