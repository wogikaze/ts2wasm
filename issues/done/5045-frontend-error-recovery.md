---
id: 5045
title: "[frontend] Improve syntax error recovery and source spans"
type: feature
area: frontend
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

malformed syntax、unterminated literal、unsupported escape、RegExp flag などの span とメッセージを安定化する。

## Problem

malformed syntax のエラー recovery が弱く、unterminated literal や unsupported escape の source span が不正確。

## Current failure

エラーメッセージの span が不正確で、デバッグが困難。

## Desired final state

malformed syntax、unterminated literal、unsupported escape、RegExp flag のエラー span とメッセージが正確かつ安定している。

## Scope

In scope:
- [ ] malformed syntax の error recovery
- [ ] unterminated literal の span 修正
- [ ] unsupported escape のメッセージ改善
- [ ] RegExp flag のエラー処理

Out of scope:
- [ ] 新規構文の追加

## Affected paths

Expected:
- `crates/frontend/src/`

## Acceptance criteria

- [ ] 各エラーケースの span が正確
- [ ] エラーメッセージが安定している

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
