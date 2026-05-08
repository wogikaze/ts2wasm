---
id: 5422
title: "W4: Implement Promise constructor and basic methods"
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

Implement Promise constructor with executor function and basic Promise.prototype methods (.then, .catch, .finally) to start reducing the async (713) and builtin-api (2,430) unsupported test262 counts at full corpus.

## Problem

Promise is completely missing from the runtime. At full corpus test262, async=713 and builtin-api=2,430 files are unsupported, many because Promise constructor and .then/.catch are not available. Hundreds of test262 cases directly test Promise behavior.

Problem: Promise runtime not implemented.

## Current failure

```sh
# Any test using Promise constructor fails
ts2wasm build fixtures/builtins-and-io/promise-basic.ts
# UnresolvedName or UnsupportedBuiltin for Promise
```

## Desired final state

- `new Promise((resolve, reject) => ...)` compiles and runs
- `promise.then(onFulfilled, onRejected)` works
- `promise.catch(onRejected)` works
- Basic test262 Promise cases pass semantic_diff

Note: This is a first slice — not full spec compliance. Microtask queue ordering and async/await integration are deferred.

## Scope

In scope:

- [ ] Add `Promise` to global builtins in name_resolver (already planned in 5412 but may need verification)
- [ ] Add RuntimeFn variants for Promise in runtime_fn.rs
- [ ] Add RuntimeSpec entries in runtime_fn_impl.rs
- [ ] Add IR routing in program_builtins.rs (Promise / Promise.resolve / Promise.reject)
- [ ] Create new file `crates/backend-wasm/src/runtime_promise.rs` with WAT:
  - Promise constructor (executor called synchronously with resolve/reject)
  - Promise.prototype.then (callback invocation, chaining)
  - Promise.prototype.catch (delegates to then)
  - Promise.resolve / Promise.reject
- [ ] Register `runtime_promise` module in lib.rs
- [ ] Add build_smoke fixtures for basic Promise
- [ ] Add test file `crates/cli/tests/m6_promise.rs`

Out of scope:

- Microtask queue / execution order (synchronous execution for first slice)
- async/await (separate issue 416)
- Promise.all/race/allSettled/any (separate slice)
- Promise.prototype.finally (separate slice)
- Promise.withResolvers (separate slice)
- Unhandled rejection tracking

## Affected paths

Expected:

- `crates/ir/src/lowered/program_builtins.rs` — add Promise IR routing
- `crates/backend-wasm/src/runtime_fn.rs` — add Promise enum variants
- `crates/backend-wasm/src/runtime_fn_impl.rs` — add RuntimeSpec entries
- `crates/backend-wasm/src/runtime_promise.rs` — new file with WAT
- `crates/backend-wasm/src/lib.rs` — register new module
- `fixtures/builtins-and-io/promise-basic.ts` — new fixture
- `crates/cli/tests/m6_promise.rs` — new test file

Do not touch:

- `crates/frontend/src/` — parser out of scope
- `crates/ir/src/name_resolver.rs` — name resolver out of scope (separate)
- `crates/backend-wasm/src/runtime_arrays.rs` — unrelated
- `crates/backend-wasm/src/runtime_strings.rs` — unrelated

## Acceptance criteria

- [ ] `new Promise((resolve) => resolve(42))` compiles without error
- [ ] `Promise.resolve(42)` compiles without error
- [ ] `Promise.reject("err")` compiles without error
- [ ] `.then(value => ...)` compiles with arrow callback
- [ ] `.catch(err => ...)` compiles with arrow callback
- [ ] Build_smoke fixtures pass

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -- m6_promise
```

## Notes

- Register the module in lib.rs: `mod runtime_promise; pub use runtime_promise::*;`
- RuntimeFn variants: `Promise_Constructor`, `Promise_Resolve`, `Promise_Reject`, `Promise_Then`, `Promise_Catch` (or similar naming)
- program_builtins.rs: route `new Promise(...)`, `Promise.resolve(...)`, `Promise.reject(...)`, `.then(...)`, `.catch(...)`
- For the first slice: executor runs synchronously during Promise constructor call; resolve/reject store the value; then/catch callbacks invoke synchronously on resolve
