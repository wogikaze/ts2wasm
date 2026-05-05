---
id: 5045
title: "[frontend] Improve syntax error recovery and source spans (audit reopened #5045)"
type: feature
area: frontend
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-05status: open
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
- [x] malformed syntax の error recovery
- [x] unterminated literal の span 修正
- [x] unsupported escape のメッセージ改善
- [x] RegExp flag のエラー処理

Out of scope:
- [x] 新規構文の追加

## Affected paths

Expected:
- `crates/frontend/src/`

## Acceptance criteria

- [x] 各エラーケースの span が正確
- [x] エラーメッセージが安定している

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

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/done/5045-frontend-error-recovery.md` (moved from open/ per close evidence)

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
