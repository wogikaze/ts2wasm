---
id: 5427
title: "W5: Implement class extends/super heritage"
type: feature
area: ir
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Implement class extends/super call and super.property access lowering. Class features are the #3 test262 blocker at 6,526 files.

## Problem

6,526 test262 files hit class-related unsupported. `extends`, `super()`, `super.method()`, and `super.property` are the main gaps.

Problem: 6,526 class unsupported at full corpus.

## Scope

In scope:

- [x] Lower `extends` clause (set up prototype chain) in resolver_expr.rs
- [x] Lower `super()` call in derived constructors
- [x] Lower `super.method()` calls
- [x] Lower `super.property` access
- [x] Add build_smoke fixtures for each form
- [x] Add test file `crates/cli/tests/m8_class_heritage.rs`

Out of scope:

- class static blocks (already implemented)
- class private fields (already implemented)
- class accessors (separate issue)

## Affected paths

Expected:

- `crates/ir/src/lowered/resolver_expr.rs` — extends/super lowering
- `crates/ir/src/lowered/resolver_extra.rs` — super helper lowering
- `fixtures/classes-and-inheritance/` — new fixtures
- `crates/cli/tests/m8_class_heritage.rs` — new test file

Do not touch:

- `crates/backend-wasm/src/runtime_fn.rs` — no new RuntimeFn needed
- `crates/frontend/src/` — parser out of scope
- `crates/ir/src/name_resolver.rs` — name resolver out of scope

## Validation

```sh
cargo fmt --all --check
cargo nextest run -- m8_class_heritage
```
