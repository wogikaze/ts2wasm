---
id: 5048
title: "[ir] Broaden BigInt lowering beyond signed-i64/first-limb slice"
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

BigInt/Number 混在、shift、bitwise、exponentiation、StringToBigInt 境界などを runtime と整合する形で拡張する。

## Problem

現在の BigInt lowering は signed-i64/first-limb の狭いスライスに制限されており、BigInt/Number 混在演算や shift/bitwise/exponentiation が未対応。

## Current failure

BigInt/Number 混在演算や BigInt shift/bitwise を含む fixture が unsupported になる。

## Desired final state

BigInt/Number 混在、shift、bitwise、exponentiation、StringToBigInt の lowering が実装される。

## Scope

In scope:
- [ ] BigInt/Number 混在演算の lowering
- [ ] BigInt shift/bitwise/exponentiation の lowering
- [ ] StringToBigInt 境界の実装

Out of scope:
- [ ] 任意精度演算の完全互換

## Affected paths

Expected:
- `crates/ir/src/`

## Acceptance criteria

- [ ] BigInt/Number 混在 fixture の lowering が通る
- [ ] BigInt shift/bitwise/exponentiation fixture の lowering が通る

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
