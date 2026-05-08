---
id: 5428
title: "W4: Implement Array.prototype.reduce/reduceRight WAT"
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

Implement WAT runtime for Array.prototype.reduce and reduceRight. Array methods are the #7 test262 blocker at 1,035 files.

## Problem

1,035 test262 files hit array-builtin unsupported. Array.prototype.reduce and reduceRight have routing (RuntimeFn entries exist) but may lack WAT implementation or have incomplete semantics.

Problem: 1,035 array-builtin unsupported at full corpus.

## Scope

In scope:

- [ ] Check if reduce/reduceRight WAT exists in runtime_arrays.rs
- [ ] If missing, add WAT implementation (callback folding, accumulator, hole skipping)
- [ ] If existing, verify Node/iwasm semantic parity
- [ ] Add build_smoke + semantic_diff fixtures
- [ ] Add test file `crates/cli/tests/m6_array_reduce.rs`

Out of scope:

- Array.prototype.reduceRight (done via same engine as reduce, reverse iteration)
- TypedArray reduce
- Other array methods

## Affected paths

Expected:

- `crates/backend-wasm/src/runtime_arrays.rs` — add/fix reduce WAT
- `fixtures/builtins-and-io/array-reduce.ts` — new fixture
- `crates/cli/tests/m6_array_reduce.rs` — new test file

Do not touch:

- `crates/backend-wasm/src/runtime_fn.rs` — no new RuntimeFn (already exists)
- `crates/ir/` — IR out of scope
- `crates/frontend/` — parser out of scope

## Validation

```sh
cargo fmt --all --check
cargo nextest run -- m6_array_reduce
```
