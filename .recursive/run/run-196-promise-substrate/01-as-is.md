# Phase 1: Analysis — Promise Minimal Substrate

## Current State

### WAT Runtime (already implemented in `crates/backend-wasm/src/runtime_promise.rs`)
- `$promise_constructor` — allocates 4-slot array (state, result, onFulfilled, onRejected), returns ARRAY-tagged
- `$promise_resolve` — creates fulfilled promise (state=1) with given value
- `$promise_reject` — creates rejected promise (state=2) with given reason
- `$promise_then` — stores onFulfilled/onRejected callbacks based on state
- `$promise_catch` — stores onRejected callback based on state

### RuntimeFn Registration (`runtime_fn.rs`)
- `PromiseConstructor`, `PromiseResolve`, `PromiseReject`, `PromiseThen`, `PromiseCatch` all registered

### IR Routing
- `program_builtins.rs`: `Promise.resolve` → `PromiseResolve`, `Promise.reject` → `PromiseReject`
- `collection_method_runtime_fn`: `("Promise", "then")` → `PromiseThen`, `("Promise", "catch")` → `PromiseCatch`
- `resolver_expr.rs` line 2515: `new Promise(executor)` → `RuntimeCall { runtime_fn: "PromiseConstructor", ... }`
- `resolver_expr.rs` line 1645: `Promise.resolve()` / `Promise.reject()` via `resolve_method_to_runtime_fn` (static call path)

### Missing
- No `promise_basic_matches_node_output` test
- `fixtures/core-semantics/promise-basic.ts` didn't exist
- `local_classes` tracking for Promise instances (`infer_class_for_expr` doesn't infer Promise from RuntimeCall)

### Existing Tests
- `build_smoke_promise_basic` in `m6_promise.rs` — build-only test, passes

### Key Design Decisions
- Promise is stored as an array (ARRAY-tagged) with fixed 4-slot layout
- State: 0=pending, 1=fulfilled, 2=rejected
- PromiseConstructor WAT ignores executor parameter (no sync executor call)
- `.then()` / `.catch()` routing requires local_classes to include Promise
