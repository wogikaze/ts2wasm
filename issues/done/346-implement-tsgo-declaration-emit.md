---
id: 346
title: "Implement TypeScript declaration emit coverage for tsgo suite (16 cases)"
type: feature
area: frontend/syntax
class: triage-needed
priority: P2
depends_on: [399]
blocks: []
created: 2026-04-30
updated: 2026-05-01
status: done
completed: 2026-05-01
---

## Summary

TypeScript declaration emit (`declare`) and declaration emit-related constructs are unsupported in 16 tsgo suite test cases. The compiler fails on declarations that include `declare` modifiers or declaration emit patterns.

## Problem

tsgo coverage shows 16 cases blocked by declaration emit support (feature label: `declaration-emit`). The frontend needs to handle `declare` keyword and related declaration patterns found in the tsgo corpus.

Problem: Child bucket of issue 399; 16 tsgo suite cases fail due to missing declaration/ambient parse/erase support, but implementation must wait for the TypeScript parse/erase/emit boundary contract.

## Current failure

```
mise run reference-coverage -- tsgo --limit 166
# Coverage matrix shows 16 declaration-emit failures
```

## Desired final state

The `declaration-emit` unsupported count in the tsgo suite is reduced to 0. `declare` modifiers and declaration emit patterns are parsed and correctly handled (erased during compilation).

## Scope

In scope:

- [x] Implement parsing of `declare` modifier on declarations
- [x] Support declare class, declare function, declare module, declare namespace
- [x] Support declare global, declare enum
- [x] Erase declare declarations during IR lowering (no runtime emission)
- [x] Add fixture tests for common declare patterns

Out of scope:

- Actual .d.ts file generation (declaration emit to files)
- Ambient declarations not tagged `declare` keyword
- tsc suite ambient declaration support (separate scope)

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/parser.rs`
- `crates/ir/src/lowered.rs`
- `crates/ir/src/lowered/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/`

## Acceptance criteria

- [x] Declaration emit unsupported count in tsgo coverage decreases from 16
- [x] Fixture tests cover basic declare class, declare function, declare module
- [x] Existing tsgo suite cases that now pass are updated
- [x] Docs/current-state/issues are synchronized when status or design changes

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
mise run reference-coverage -- tsgo --limit 166
mise run update-coverage-matrix
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected
- [x] not updated in this slice

Current state:

- [x] not affected
- [x] not updated in this slice

Follow-up issues:

- [x] none

## Notes

This issue is a child implementation bucket of issue 399. Do not start broad declaration/ambient implementation until issue 399 defines the TypeScript parse/erase/emit boundary and confirms which `declare` and declaration-emit forms are pure erasure, which affect module shape, and which need narrower child issues.

Boundary decision after issue 399: `declaration-emit` maps to category 2 when declaration output or module shape matters, and category 4 when declaration-only input must be rejected or erased without runtime bindings. Ambient declaration erasure is split to issue 400.

Similar to type alias erasure, many `declare` declarations are compile-time only and should be erased if issue 399 confirms they have no runtime/module-shape effect for the selected cases. The parser then needs to accept `declare` as a declaration modifier and route it through the erasure boundary.

## Completion evidence

- Parser support and regressions were implemented in:
  - `crates/frontend/src/parser/helpers.rs`
  - `crates/frontend/src/parser/expressions.rs`
  - `crates/frontend/src/parser/statements.rs`
  - `crates/frontend/src/parser/tests.rs`
- The issue file was moved from `issues/open/` to `issues/done/`.
- `issues/index.md` was regenerated via `mise run update-issue-index`.
