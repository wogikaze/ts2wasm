---
id: 5044
title: "[frontend] Define and test TypeScript ambient declaration erasure boundaries"
type: feature
area: frontend
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

`declare`, ambient class/function/var, global augmentation などの runtime 影響有無を仕様化し、issue-400 系を整理する。

## Problem

ambient declaration の erasure 境界が不明確で、どの宣言が runtime に影響を与えるかの判断が一貫していない。

## Current failure

`declare` 付き宣言の erasure 漏れや過剰 erasure が発生している。

## Desired final state

ambient declaration の erasure 境界が文書化され、各ケースの fixture テストが存在する。

## Scope

In scope:
- [ ] ambient declaration 分類の仕様化
- [ ] 各ケースの fixture 追加
- [ ] issue-400 系の整理

Out of scope:
- [ ] runtime semantic の完全互換

## Affected paths

Expected:
- `crates/frontend/`
- `fixtures/`

## Acceptance criteria

- [ ] ambient declaration の分類が文書化される
- [ ] 各分類の fixture が存在する
- [ ] erasure 境界がテストで担保される

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
