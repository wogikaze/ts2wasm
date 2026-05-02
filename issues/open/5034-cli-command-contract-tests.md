---
id: 5034
title: "[cli] Add command contract tests for build/check/dump/server"
type: test
area: cli
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

CLI サブコマンドごとの exit code、stderr、生成物、manifest 出力を固定し、ユーザー向け挙動を安定させる。

## Problem

CLI サブコマンド（build/check/dump/server）の出力形式や終了コードが暗黙的に決まっており、回帰時に検出できない。

## Current failure

CLI の出力形式変更がテストで捕捉されず、ユーザーに影響する可能性がある。

## Desired final state

全 CLI サブコマンドの exit code、stderr、生成物、manifest 出力が契約テストで固定される。

## Scope

In scope:
- [ ] build コマンドの契約テスト
- [ ] check コマンドの契約テスト
- [ ] dump コマンドの契約テスト
- [ ] server コマンドの契約テスト

Out of scope:
- [ ] CLI の新機能追加

## Affected paths

Expected:
- `crates/cli/tests/`

## Acceptance criteria

- [ ] 各サブコマンドの契約テストが存在する
- [ ] 出力変更時にテストが fail する

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
