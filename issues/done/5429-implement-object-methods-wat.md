---
id: 5429
title: "W4: Implement Object.values/getPrototypeOf/setPrototypeOf WAT"
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

Implement WAT runtime for Object.values, Object.getPrototypeOf, and Object.setPrototypeOf. RuntimeFn routing already exists but WAT implementations are missing. Object methods are the #8 test262 blocker at 813 files.

## Problem

813 test262 files hit object-builtin unsupported. The runtime routing exists but WAT implementations for `Object.values`, `Object.getPrototypeOf`, `Object.setPrototypeOf` may be missing.

Problem: 813 object-builtin unsupported at full corpus.

## Scope

In scope:

- [x] Check runtime_objects.rs for existing Object.values WAT
- [x] If missing, add Object.values WAT (iterate own enumerable properties, push values to array)
- [x] Check Object.getPrototypeOf WAT — add if missing
- [x] Check Object.setPrototypeOf WAT — add if missing
- [x] Add build_smoke fixtures
- [x] Add test file `crates/cli/tests/m6_object_methods.rs`

Out of scope:

- Object.entries, Object.keys (may already exist — check first)
- Object.fromEntries
- Object.groupBy

## Affected paths

Expected:

- `crates/backend-wasm/src/runtime_objects.rs` — add missing WAT
- `fixtures/builtins-and-io/object-values.ts` — new fixture
- `fixtures/builtins-and-io/object-get-prototype-of.ts` — new fixture
- `crates/cli/tests/m6_object_methods.rs` — new test file

Do not touch:

- `crates/backend-wasm/src/runtime_fn.rs` — no new RuntimeFn (routing already exists)
- `crates/backend-wasm/src/runtime_fn_impl.rs` — no catalog changes
- `crates/ir/src/lowered/program_builtins.rs` — routing already exists
- `crates/frontend/` — parser out of scope

## Validation

```sh
cargo fmt --all --check
cargo nextest run -- m6_object_methods
```
