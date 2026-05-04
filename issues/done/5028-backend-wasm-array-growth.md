---
id: 5028
title: "[backend-wasm] Implement array growth and reallocation for push/write paths"
type: feature
area: backend
class: implementation-ready
priority: P0
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

array push が「容量十分前提」になっているため、capacity check、reallocate、presence bitmap 更新、GC root 保護を含めて安全化する。

## Problem

現在の array push 実装は事前確保済み容量を前提としており、容量不足時の reallocation が行われない。その結果、配列の動的成長が必要なケースでメモリ破壊や crash が発生する。

## Current failure

`arr.push(value)` を繰り返す fixture が、配列容量を超えた時点で不正なメモリアクセスを起こす。

## Desired final state

array push が capacity check → reallocation → presence bitmap 更新の安全なパスを通り、動的成長が正しく動作する。

## Scope

In scope:
- [x] Capacity check の実装
- [x] Reallocation パス（新容量計算、コピー、旧領域解放）
- [x] Presence bitmap の更新
- [x] GC root 保護

Out of scope:
- [x] 疎配列の最適化
- [x] 型付き配列

## Affected paths

Expected:
- `crates/backend-wasm/src/`

## Acceptance criteria

- [x] 動的 push fixture が Node differential で一致
- [x] capacity 超過時の再割り当てが正しく動作
- [x] GC mark が再割り当て後も正しく動作

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
- [x] none
