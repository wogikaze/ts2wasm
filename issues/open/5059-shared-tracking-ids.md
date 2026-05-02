---
id: 5059
title: "[shared] Add typed tracking IDs for unsupported and blocked tests"
type: feature
area: coverage
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

`tracking: String` を自由文字列にせず、`issue-xxx` / `feature:xxx` 形式の validation を追加する。

## Problem

unsupported/blocked test の tracking ID が自由文字列であり、issue 番号と feature 名の区別が不明確。

## Current failure

tracking ID の typo や誤形式が検出されない。

## Desired final state

tracking ID が `issue-xxx` / `feature:xxx` 形式に制約され、validation で検査される。

## Scope

In scope:
- [ ] tracking ID の型定義
- [ ] validation の追加
- [ ] 既存 tracking ID の移行

Out of scope:
- [ ] 新規 feature の追加

## Affected paths

Expected:
- `crates/shared/`

## Acceptance criteria

- [ ] tracking ID が型付けされる
- [ ] 不正な形式が validation で検出される
- [ ] 既存の正しい tracking ID が移行される

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
