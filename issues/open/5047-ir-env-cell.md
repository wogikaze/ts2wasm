---
id: 5047
title: "[ir] Implement env-cell lowering for outer-scope mutation"
type: feature
area: ir
class: implementation-ready
priority: P0
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

top-level function から外側 binding を mutate するケースを reject しているため、heap env cell ベースの代入 lowering を実装する。

## Problem

現在は外側スコープの変数への代入を静的に reject しており、closure や nested function からの mutable 参照がサポートされていない。

## Current failure

外側スコープ変数への代入を含む closure fixture がコンパイルエラーになる。

## Desired final state

heap env cell ベースの代入 lowering により、closure や nested function からの外側変数 mutation が正しく動作する。

## Scope

In scope:
- [ ] env cell の IR 設計
- [ ] heap env cell の lowering
- [ ] mutation パスの実装
- [ ] GC root 保護

Out of scope:
- [ ] 複数レベルのネストした env

## Affected paths

Expected:
- `crates/ir/src/`

## Acceptance criteria

- [ ] closure からの外側変数 mutation fixture が動作する
- [ ] env cell の GC mark が正しい

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
