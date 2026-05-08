---
id: 5416
title: "W4: Implement Object.assign/defineProperty runtime descriptor handling"
type: feature
area: runtime
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Implement correct property descriptor handling in Object.assign and Object.defineProperty runtime, including writable/enumerable/configurable attribute propagation and getter/setter descriptor support.

## Problem

Object.assign and Object.defineProperty currently pass build_smoke but may not handle all property descriptor invariants correctly: non-writable property copy behavior, getter/setter descriptor creation, and attribute preservation. This affects test262 semantic_pass for object-builtin tests.

Problem: Object.assign/defineProperty runtime descriptor semantics not fully Node-compatible.

## Current failure

```sh
# Object.defineProperty with getter/setter may differ from Node
ts2wasm build fixtures/builtins-and-io/object-descriptor-edge.ts
iwasm output.wasm  # output may differ from node
```

## Desired final state

Object.assign correctly copies property descriptors (not just values), Object.defineProperty correctly creates data/accessor descriptors, and both achieve Node/iwasm semantic_diff parity for basic descriptor operations.

## Scope

In scope:

- [ ] Verify Object.assign copies property descriptors (writable, enumerable, configurable)
- [ ] Verify Object.defineProperty creates data descriptors (value, writable, enumerable, configurable)
- [ ] Verify Object.defineProperty creates accessor descriptors (get, set, enumerable, configurable)
- [ ] Fix any gaps in descriptor attribute handling in runtime_objects.rs
- [ ] Add semantic_diff fixtures:
  - `object-assign-descriptors.ts` — Object.assign preserves descriptor attributes
  - `object-define-property-getter.ts` — defineProperty with getter
  - `object-define-property-data.ts` — defineProperty with data descriptor
  - `object-define-property-invalid.ts` — invalid descriptor shapes
- [ ] Add test file `crates/cli/tests/m6_object_descriptors.rs`

Out of scope:

- Object.freeze/seal/preventExtensions with full invariants
- Object.defineProperties (plural) — same engine, separate test
- Proxy traps for defineProperty
- [[DefineOwnProperty]] internal method for all object types

## Affected paths

Expected:

- `crates/backend-wasm/src/runtime_objects.rs` — fix descriptor handling WAT
- `fixtures/builtins-and-io/object-assign-descriptors.ts`
- `fixtures/builtins-and-io/object-define-property-getter.ts`
- `fixtures/builtins-and-io/object-define-property-data.ts`
- `crates/cli/tests/m6_object_descriptors.rs` — new test file

Do not touch:

- `crates/frontend/` — parser out of scope
- `crates/ir/` — IR out of scope
- `crates/backend-wasm/src/runtime_fn.rs` — no new RuntimeFn variants
- `crates/backend-wasm/src/runtime_fn_impl.rs` — no catalog changes
- `crates/backend-wasm/src/runtime_arrays.rs`, `runtime_strings.rs`

## Acceptance criteria

- [ ] `Object.assign(target, {x: 1})` copies `x` as writable data descriptor (matching Node)
- [ ] `Object.defineProperty(o, 'x', {value: 1})` creates data descriptor with default false for writable/enumerable/configurable
- [ ] `Object.defineProperty(o, 'x', {get: () => 42})` creates accessor descriptor
- [ ] `Object.getOwnPropertyDescriptor` returns correct descriptor after Object.assign
- [ ] Invalid descriptors (e.g., both value and get) produce TypeError matching Node
- [ ] All 3+ new semantic_diff fixtures pass (Node matches iwasm)

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -- m6_object_descriptors
cargo nextest run -- m2_node_diff
```

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [ ] not affected
- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

- runtime_objects.rs (~909 lines) has existing assign/defineProperty WAT
- Property descriptors in RawValue: the descriptor is an object with integer-tagged property flags
- The WAT needs to pack descriptor attributes into the object's property slot metadata
- Error handling: TypeError for conflicting descriptor properties (value+get, value+set, etc.)
