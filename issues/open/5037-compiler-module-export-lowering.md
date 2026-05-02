---
id: 5037
title: "[compiler] Complete entry module export lowering for local references"
type: feature
area: cli
class: implementation-ready
priority: P0
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

`export const value = 1` は進んでいるが、local binding 参照や re-export 境界が残るため、issue-5005 系を整理して実装する。

## Problem

現在の module export lowering は狭いスライスしか対応しておらず、local binding の再 export や、export された値の参照が正しく lower されない。

## Current failure

module 間の export/import が local binding 参照を含むケースで誤った WASM コードを生成する。

## Desired final state

entry module の export が local binding、re-export を含めて正しく lowered IR に変換される。

## Scope

In scope:
- [ ] local binding 参照の export lowering
- [ ] re-export の実装
- [ ] issue-5005 系の整理と実装

Out of scope:
- [ ] dynamic import
- [ ] namespace import/export

## Affected paths

Expected:
- `crates/cli/`
- `crates/ir/`

## Acceptance criteria

- [ ] local binding export fixture が正しく lower される
- [ ] re-export fixture が正しく lower される
- [ ] 既存 module fixture が後方互換を維持する

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
