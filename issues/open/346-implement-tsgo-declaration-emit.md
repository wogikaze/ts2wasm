---
id: 346
title: "Implement TypeScript declaration emit coverage for tsgo suite (16 cases)"
type: feature
area: frontend/syntax
class: triage-needed
priority: P2
depends_on: []
blocks: []
created: 2026-04-30
updated: 2026-04-30
---

## Summary

TypeScript declaration emit (`declare`) and declaration emit-related constructs are unsupported in 16 tsgo suite test cases. The compiler fails on declarations that include `declare` modifiers or declaration emit patterns.

## Problem

tsgo coverage shows 16 cases blocked by declaration emit support (feature label: `declaration-emit`). The frontend needs to handle `declare` keyword and related declaration patterns found in the tsgo corpus.

Problem: 16 tsgo suite cases fail due to missing declaration emit support.

## Current failure

```
mise run reference-coverage -- tsgo --limit 166
# Coverage matrix shows 16 declaration-emit failures
```

## Desired final state

The `declaration-emit` unsupported count in the tsgo suite is reduced to 0. `declare` modifiers and declaration emit patterns are parsed and correctly handled (erased during compilation).

## Scope

In scope:

- [ ] Implement parsing of `declare` modifier on declarations
- [ ] Support declare class, declare function, declare module, declare namespace
- [ ] Support declare global, declare enum
- [ ] Erase declare declarations during IR lowering (no runtime emission)
- [ ] Add fixture tests for common declare patterns

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

- [ ] Declaration emit unsupported count in tsgo coverage decreases from 16
- [ ] Fixture tests cover basic declare class, declare function, declare module
- [ ] Existing tsgo suite cases that now pass are updated
- [ ] Docs/current-state/issues are synchronized when status or design changes

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
- [ ] updated: `docs/...`

Current state:

- [x] not affected
- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

Similar to type alias erasure — `declare` declarations are compile-time only and should be erased. The parser needs to accept `declare` as a declaration modifier.
