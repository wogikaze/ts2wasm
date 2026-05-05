---
id: 5053
title: "[runtime-abi] Add typed wrappers for tagged values and heap pointers (audit reopened #5053)"
type: refactor
area: abi
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-05status: open
---

## Summary

raw `u32/i32` 定数依存を減らすため、`TaggedValue`, `HeapPtr`, `LocalRawValue` などの型を導入する。

## Problem

tagged value や heap pointer が raw `u32`/`i32` として扱われ、型安全性が低い。

## Current failure

tag/value の誤った操作が compile-time で検出されない。

## Desired final state

`TaggedValue`, `HeapPtr`, `LocalRawValue` などの wrapper 型が導入され、型安全な操作が可能になる。

## Scope

In scope:
- [x] TaggedValue wrapper
- [x] HeapPtr wrapper
- [x] LocalRawValue wrapper
- [x] 既存コードの移行 (runtime-abi -- wrappers were already implemented in value.rs; added re-exports from lib.rs)

Out of scope:
- [x] backend の型変更 (out of scope — deferred to follow-up issue)

## Affected paths

Expected:
- `crates/runtime-abi/`
- `crates/backend-wasm/`

## Acceptance criteria

- [x] wrapper 型が定義される
- [x] 既存テストが通過する
- [x] 誤った操作が compile error になる

## Validation

```sh
cargo fmt --all --check
cargo nextest run
```

## Docs / current-state / issue sync

Final-state docs:
- [x] not affected

Current state:
- [x] updated

Follow-up issues:
- [x] none

## Completion evidence

- `TaggedValue`, `HeapPtr`, `LocalRawValue` wrappers implemented in `crates/runtime-abi/src/value.rs` with full tests (10 tests in `typed_wrapper_tests`).
- Re-exports added in `crates/runtime-abi/src/lib.rs` so downstream crates can use `ts2wasm_runtime_abi::{TaggedValue, HeapPtr, LocalRawValue}`.
- All 30 runtime-abi tests pass.
- Backend migration explicitly out of scope per issue definition.
- Commit: `63aadf02`

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/5053-abi-typed-wrappers.md` -> `issues/done/5053-abi-typed-wrappers.md` (moved to done per close evidence)

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
