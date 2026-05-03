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

Out of scope:
- [x] backend emission (tracked by 5026)

## Affected paths

Expected:
- `crates/ir/src/`

## Acceptance criteria

- [x] class elements are represented in HIR
- [x] lowered ClassDecl emits class binding (FuncId-based variant with constructor/methods/static_methods/private_fields)
- [x] IR invariants are defined (FuncId validation in validate.rs)

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
- [x] backend emission (tracked by 5026)

## Completion evidence

Implementation:
1. program.rs: Phase 2 ClassDecl handler now emits `LoweredStmt::ClassDecl` with FuncId-based fields (`constructor: Option<FuncId>`, `methods: Vec<(String, FuncId)>`, `static_methods: Vec<(String, FuncId)>`, `private_fields: Vec<String>`) before static initializers (correct JS semantics). Previously the statement was dropped with a LIMITATION comment.
2. validate.rs: Updated from dead-variant comment to active FuncId range validation for constructor and all methods.
3. ir_lowering.rs: Three tests updated to account for ClassDecl emission shifting top_level_statements indices, with assertions verifying ClassDecl fields.

Validation: `cargo nextest run` — 56 ir_lowering tests pass, 0 regressions from this change (2 pre-existing m2_node_diff failures confirmed on base commit).
