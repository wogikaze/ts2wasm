---
id: 5035
title: "[cli] Add --explain-unsupported diagnostics mode"
type: feature
area: cli
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

unsupported 診断に、該当 fixture、tracking issue、回避策、次に実装すべき crate を表示する開発支援モードを追加する。

## Problem

unsupported 診断の情報が不十分で、開発者が次に何をすべきか判断しにくい。

## Current failure

unsupported エラー時に tracking issue や回避策が表示されない。

## Desired final state

`--explain-unsupported` フラグで、unsupported 診断の詳細（tracking issue、該当 fixture、回避策、実装優先度）が表示される。

## Scope

In scope:
- [x] `--explain-unsupported` CLI フラグの追加
- [x] tracking issue の紐付け
- [x] 該当 fixture の表示

Out of scope:
- [x] unsupported ケースの自動修正

## Affected paths

Expected:
- `crates/cli/src/`

## Acceptance criteria

- [x] `--explain-unsupported` が tracking issue を表示する
- [x] 該当 fixture パスが表示される

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
