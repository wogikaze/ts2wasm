---
id: 5036
title: "[compiler] Introduce CompileReport<T> for non-fatal diagnostics"
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

現在の `Result<T, Diagnostic>` では「生成はできるが警告/unsupported もある」を表現しにくい。`value + diagnostics` 形式へ移行する。

## Problem

`Result<T, Diagnostic>` は成功か失敗かの二値であり、「コンパイルは成功したが unsupported 診断が発生した」という状態を表現できない。

## Current failure

unsupported 診断が発生してもコンパイルが続行できるケースで、診断情報が失われる。

## Desired final state

`CompileReport<T>` が値と診断リストを保持し、非 fatal 診断を propagate できる。

## Scope

In scope:
- [ ] `CompileReport<T>` 型の定義
- [ ] コンパイラパイプライン全体の移行
- [ ] 非 fatal 診断の集約と出力

Out of scope:
- [ ] 診断の自動修正
- [ ] エラー種類の網羅的追加

## Affected paths

Expected:
- `crates/cli/`

## Acceptance criteria

- [ ] `CompileReport<T>` が値と診断リストを保持する
- [ ] 非 fatal 診断が最終出力に含まれる
- [ ] 既存の fatal エラーが変わらず報告される

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
