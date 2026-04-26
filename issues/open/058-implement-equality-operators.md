---
id: 058
title: "Implement equality operators (==, !=, ===, !==)"
type: feature
area: runtime/semantics
class: design-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement equality operators to handle equality-operator feature gaps in reference tests.

## Problem

Reference test results show 3 cases fail with equality-operator diagnostic (tsc:1, tsgo:2). The compiler cannot handle equality operators (==, !=, ===, !==), preventing compilation of basic JavaScript comparison code.

## Desired final state

Equality operators correctly implement JavaScript equality semantics. Abstract equality (==, !=) and strict equality (===, !==) are both supported with proper type coercion rules.

## Scope

In scope:

- [ ] Add equality operators to lexer/parser
- [ ] Implement strict equality (===, !==)
- [ ] Implement abstract equality (==, !=) with type coercion
- [ ] Handle type coercion rules per ECMAScript spec
- [ ] Add fixtures for equality operator behavior

Out of scope:

- [ ] Comparison operators (<, >, <=, >=) (separate issue)
- [ ] Object equality (covered by prototype chain issues)

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/`
- `crates/backend-wasm/`

## Acceptance criteria

- [ ] Equality operators pass for basic comparisons
- [ ] equality-operator diagnostic reduced to 0 in reference tests
- [ ] Regression test added for equality operators
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
scripts/run/reference-coverage.sh tsc --limit 100
scripts/run/reference-coverage.sh tsgo --limit 50
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

Start with strict equality before adding abstract equality with type coercion.

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
