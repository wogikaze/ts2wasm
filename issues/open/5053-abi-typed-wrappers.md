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
- [ ] TaggedValue wrapper
- [ ] HeapPtr wrapper
- [ ] LocalRawValue wrapper
- [ ] 既存コードの移行

Out of scope:
- [ ] backend の型変更

## Affected paths

Expected:
- `crates/runtime-abi/`
- `crates/backend-wasm/`

## Acceptance criteria

- [ ] wrapper 型が定義される
- [ ] 既存テストが通過する
- [ ] 誤った操作が compile error になる

## Validation

```sh
cargo fmt --all --check
cargo nextest run
```

## Docs / current-state / issue sync

Final-state docs:
- [ ] not affected

Current state:
- [ ] not affected

Follow-up issues:
- [ ] none

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/5053-abi-typed-wrappers.md` before this move
- `issues/open/5053-abi-typed-wrappers.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
