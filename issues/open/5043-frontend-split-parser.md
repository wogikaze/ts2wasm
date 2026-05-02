---
id: 5043
title: "[frontend] Split large lexer/parser files by grammar responsibility"
type: refactor
area: frontend
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

`lexer.rs` や statement/expression parser が肥大化しているため、literal、binding、class、module、TS erasure などへ分割する。

## Problem

frontend の lexer/parser ファイルが肥大化しており、保守性とテスト容易性が低下している。

## Current failure

単一ファイルの巨大化により、コードレビューが難しく、並行開発が困難。

## Desired final state

lexer/parser が文法責任単位（literal、binding、class、module、TS erasure）に分割される。

## Scope

In scope:
- [ ] lexer の分割（literal, identifier, operator など）
- [ ] expression parser の分割
- [ ] statement parser の分割
- [ ] class/module/TS erasure の分離

Out of scope:
- [ ] ロジックの変更
- [ ] 新機能の追加

## Affected paths

Expected:
- `crates/frontend/src/`

## Acceptance criteria

- [ ] 各機能単位のファイルが存在する
- [ ] 既存テストがすべて通過する

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
