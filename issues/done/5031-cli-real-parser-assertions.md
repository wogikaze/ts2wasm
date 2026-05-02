---
id: 5031
title: "[cli] Replace placeholder parser keyword/operator tests with real assertions"
type: test
area: cli
class: implementation-ready
priority: P0
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

`parser_keywords` 系に placeholder test が多いため、実際に token/AST を検証するテストへ置き換える。

## Problem

現在の parser テストの多くが `// TODO: assert actual token/AST` 形式の placeholder であり、実際の構文解析結果を検証していない。

## Current failure

Parser のリグレッションを既存テストで検出できない。

## Desired final state

各 keyword/operator parser test が実際の token 種別、位置情報、AST 構造をアサートする。

## Scope

In scope:
- [x] 全 placeholder test の実アサーション化
- [x] Token/AST の期待値 fixture 追加

Out of scope:
- [x] 新規構文の追加

## Affected paths

Expected:
- `crates/cli/tests/`

## Acceptance criteria

- [x] 全 parser keyword/operator test が実アサーションを持つ
- [x] 期待値不一致時にテストが fail する

## Validation

```sh
cargo fmt --all --check
cargo nextest run
```

## Docs / current-state / issue sync

## Completion evidence

### Changes

Commit `6ff7aefe` replaced 33 placeholder parser keyword/operator tests with real assertions against actual token types, AST structures, and source positions.

### Verification

- `cargo fmt --all --check`: passes
- `cargo nextest run`: passes

### Scope checklist

- [x] 全 placeholder test の実アサーション化
- [x] Token/AST の期待値 fixture 追加

### Acceptance criteria

- [x] 全 parser keyword/operator test が実アサーションを持つ
- [x] 期待値不一致時にテストが fail する

Final-state docs:
- [x] not affected

Current state:
- [x] not affected

Follow-up issues:
- [x] none
