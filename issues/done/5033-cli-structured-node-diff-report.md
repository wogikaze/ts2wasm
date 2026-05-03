---
id: 5033
title: "[cli] Normalize node-diff fixture reporting into structured records"
type: feature
area: cli
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

differential test の結果を `pass/fail/unsupported/blocked` と理由・tracking ID 付きで JSONL 出力し、回帰を追いやすくする。

## Problem

現在の node-diff fixture のレポート形式が非構造化テキストであり、結果の集計や回帰検出が手動作業に依存している。

## Current failure

Differential test の pass/fail 状態を機械的に追跡できない。

## Desired final state

全 node-diff fixture が構造化 JSONL レコードを出力し、回帰検出と集計が自動化される。

## Scope

In scope:
- [x] pass/fail/unsupported/blocked 分類の実装
- [x] tracking ID による紐付け
- [x] JSONL 出力形式の定義と実装

Out of scope:
- [x] Web UI の変更

## Affected paths

Expected:
- `crates/cli/`

## Acceptance criteria

- [x] 全 node-diff fixture が JSONL 出力する
- [x] 出力スキーマが文書化されている
- [x] 既存の集計スクリプトが JSONL を消費できる

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
