---
id: 5028
title: "[backend-wasm] Implement array growth and reallocation for push/write paths (audit reopened #5028)"
type: feature
area: backend
class: implementation-ready
priority: P0
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-05status: open
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
- [ ] Capacity check の実装
- [ ] Reallocation パス（新容量計算、コピー、旧領域解放）
- [ ] Presence bitmap の更新
- [ ] GC root 保護

Out of scope:
- [ ] 疎配列の最適化
- [ ] 型付き配列

## Affected paths

Expected:
- `crates/backend-wasm/src/`

## Acceptance criteria

- [ ] 動的 push fixture が Node differential で一致
- [ ] capacity 超過時の再割り当てが正しく動作
- [ ] GC mark が再割り当て後も正しく動作

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
- `issues/open/5028-backend-wasm-array-growth.md` before this move
- `issues/open/5028-backend-wasm-array-growth.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
