---
id: 5030
title: "[backend-wasm] Split large runtime/WAT emitters into testable components (audit reopened #5030)"
type: refactor
area: backend
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-05
status: open
---

## Summary

`expr_emit` や runtime emitter が巨大化しているため、文字列・配列・BigInt・関数・GC などの単位に分割し、golden WAT テストを追加する。

## Problem

`expr_emit.rs` などの emitter ファイルが肥大化しており、保守性とテスト容易性が低下している。

## Current failure

単一ファイルの巨大化により、コードレビューが難しく、単体テストが不足している。

## Desired final state

emitter が機能単位のファイルに分割され、各モジュールに golden WAT テストが存在する。

## Scope

In scope:
- [x] 文字列 emitter の分割
- [x] 配列 emitter の分割
- [x] BigInt emitter の分割
- [x] 関数 emitter の分割
- [x] GC emitter の分割
- [x] Golden WAT テストの追加

Out of scope:
- [x] ロジックの変更
- [x] 新機能の追加

## Affected paths

Expected:
- `crates/backend-wasm/src/`

## Acceptance criteria

- [x] 各機能単位のファイルが存在する
- [x] 既存テストがすべて通過する
- [x] Golden WAT テストが追加されている

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
- `issues/done/5030-backend-wasm-split-runtime-emitters.md` (moved from open/ per close evidence)

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
