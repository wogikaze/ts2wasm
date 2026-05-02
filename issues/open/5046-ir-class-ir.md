---
id: 5046
title: "[ir] Design full class runtime IR representation"
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

class を standalone function 抽出だけで扱う現在の制限を超え、constructor/prototype/static/private/extends を IR として表現する。

## Problem

現在の IR は class を単なる関数抽出で処理しており、constructor/prototype chain/static members/private elements/extends を IR レベルで表現できない。

## Current failure

class 宣言が IR で正しく表現されず、backend が class 固有のコードを生成できない。

## Desired final state

class の全構成要素（constructor, prototype, static, private, extends）が HIR/Lowered IR で表現される。

## Scope

In scope:
- [ ] class IR variant の設計
- [ ] constructor/prototype/static の IR 表現
- [ ] private elements の IR 表現
- [ ] extends の IR 表現

Out of scope:
- [ ] backend emission（別 issue）

## Affected paths

Expected:
- `crates/ir/src/`

## Acceptance criteria

- [ ] class の全要素が IR で表現される
- [ ] IR の不変条件が定義される
- [ ] round-trip テストが通る

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
