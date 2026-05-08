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

Equality operator syntax and strict equality are implemented. Abstract equality (`==`, `!=`) initially delegated to strict equality; primitive coercion was later completed by `issues/done/216-implement-abstract-equality-coercion.md`.

## Scope

In scope:

- [x] Add equality operators to lexer/parser
- [x] Implement strict equality (===, !==)
- [x] Implement abstract equality (==, !=) with type coercion (initial partial strict fallback; primitive coercion completed by `issues/done/216-implement-abstract-equality-coercion.md`)
- [x] Handle type coercion rules per ECMAScript spec (initial partial strict fallback; primitive coercion completed by `issues/done/216-implement-abstract-equality-coercion.md`)
- [x] Add fixtures for equality operator behavior

Out of scope:

- [x] Comparison operators (<, >, <=, >=) (separate issue)
- [x] Object equality (covered by prototype chain issues)

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/`
- `crates/backend-wasm/`

## Acceptance criteria

- [x] Equality operators pass for basic comparisons
- [x] equality-operator diagnostic reduced to 0 in reference tests
- [x] Regression test added for equality operators
- [x] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 100
mise run reference-coverage -- tsgo --limit 50
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] Abstract equality primitive coercion completed by `issues/done/216-implement-abstract-equality-coercion.md`

## Notes

Start with strict equality before adding abstract equality with type coercion.

Implemented equality operators (==, !=, ===, !==) with the following changes:
- Added EqualEqual, BangEqual, StrictNotEqual tokens to lexer
- Added EqualEqual, BangEqual, StrictNotEqual to BinaryOp enum
- Updated parser to handle all equality operators
- Added EqualEqual, BangEqual, StrictNotEqual to LoweredBinaryOp enum
- Added corresponding RuntimeFn variants and runtime implementations
- Abstract equality (==, !=) initially delegated to strict_equal for simplicity. Primitive coercion was later completed by `issues/done/216-implement-abstract-equality-coercion.md`.

## Completion evidence

Commits:

- `5b38e7b` wip: start issue 058 - implement equality operators
- (pending commit for implementation)

Validation result:

```text
command: cargo nextest run
result: 202 passed, 4 skipped
date: 2026-04-26
```

Remaining risks:

- Abstract equality (==, !=) initially delegated to strict equality without full type coercion. Primitive coercion was later completed by `issues/done/216-implement-abstract-equality-coercion.md`.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/058-implement-equality-operators.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
