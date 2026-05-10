# Phase 2: Plan

## Goal
Register the first acceptance test for Promise minimal substrate.

## Changes
1. `fixtures/core-semantics/promise-basic.ts` — NEW: minimal fixture using `new Promise()`, `Promise.resolve()`, console.log
2. `crates/cli/tests/common/m2_node_diff_fixture_tests.rs` — Add `promise_basic_matches_node_output` test

## Non-goals
- No `Promise.then()` callback execution
- No async/await integration
- No Promise.prototype.finally
- No changes to WAT runtime, IR routing, or existing tests

## Acceptance
```
cargo nextest run -p ts2wasm-cli --test m2_node_diff promise_basic_matches_node_output
```
