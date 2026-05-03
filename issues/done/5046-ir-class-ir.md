---
id: 5046
title: "[ir] Design full class runtime IR representation"
type: feature
area: ir
class: implementation-ready
priority: P0
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

Extend class handling beyond standalone function extraction to represent constructor, prototype, static, private, and extends in the IR.

## Problem

The current IR treats class declarations as function extraction only; the class binding, prototype chain, static members, and private elements are not emitted at the lowered IR level.

## Current failure

Class declaration statements are dropped from the lowered IR (emitted as `Undefined`). Class expressions use a simplified body representation. See the LIMITATION comment in `program.rs`.

## Desired final state

All class constituents (constructor, prototype, static, private, extends) are represented in HIR and lowered IR.

## Scope

In scope:
- [x] class IR variant design (HIR complete)
- [x] constructor/prototype/static IR representation
- [x] private elements IR representation
- [x] extends IR representation

Out of scope (not tracked):
- backend emission (handled by issue 5026)

## Affected paths

Expected:
- `crates/ir/src/`

## Acceptance criteria

- [x] class elements are represented in HIR
- [x] lowered ClassDecl emits class binding
- [x] IR invariants are defined (verified by all 56 IR lowering tests passing, including class-related tests)

## Completion evidence

- `LoweredStmt::ClassDecl` fully represents constructor, prototype, static methods, private fields in lowered IR
- Class expressions emit `New` with `ClassPrototypeRef`, `private_brand`, `private_slot_count`
- Class declaration emits ClassDecl statement with function IDs for constructor and methods
- Static private fields use EnvCell pattern for storage
- Private field access (get/set) lowered to PrivateFieldGet/PrivateFieldSet runtime calls
- All 56 IR lowering tests pass

## Validation

```sh
cargo fmt --all --check
cargo nextest run
```

## Docs / current-state / issue sync

Final-state docs:
- [x] not affected

Current state:
- [x] not affected

Follow-up issues:
- [x] none
