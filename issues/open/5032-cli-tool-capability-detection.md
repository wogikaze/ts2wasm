---
id: 5032
title: "[cli] Add deterministic external tool capability detection"
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

`node` / `iwasm` / `wat2wasm` 依存テストの skip/fail 条件を統一し、環境差で CI が不安定にならないようにする。

## Problem

外部ツール（node, iwasm, wat2wasm）の有無やバージョンによってテスト結果が非決定的に変化する。

## Current failure

特定の環境で外部ツールが不足している場合、テストが予期せず fail または skip される。

## Desired final state

全テストが外部ツールの有無を comptime/startup で検出し、統一されたポリシーで skip/fail を決定する。

## Scope

In scope:
- [ ] node 検出ロジックの統一
- [ ] iwasm 検出ロジックの統一
- [ ] wat2wasm 検出ロジックの統一
- [ ] skip/fail ポリシーの明文化

Out of scope:
- [ ] ツールの自動インストール

## Affected paths

Expected:
- `crates/cli/tests/`

## Acceptance criteria

- [ ] ツール不足時の skip/fail が一貫している
- [ ] CI の不安定な skip/fail が解消される

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
