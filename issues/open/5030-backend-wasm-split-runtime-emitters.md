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
updated: 2026-05-05status: open
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
- [ ] 文字列 emitter の分割
- [ ] 配列 emitter の分割
- [ ] BigInt emitter の分割
- [ ] 関数 emitter の分割
- [ ] GC emitter の分割
- [ ] Golden WAT テストの追加

Out of scope:
- [ ] ロジックの変更
- [ ] 新機能の追加

## Affected paths

Expected:
- `crates/backend-wasm/src/`

## Acceptance criteria

- [ ] 各機能単位のファイルが存在する
- [ ] 既存テストがすべて通過する
- [ ] Golden WAT テストが追加されている

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

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/5030-backend-wasm-split-runtime-emitters.md` before this move
- `issues/open/5030-backend-wasm-split-runtime-emitters.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
