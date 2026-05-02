---
id: 5056
title: "[shared] Replace manual TestRecord JSON construction with serde serialization"
type: refactor
area: coverage
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

`to_json_line` が手書き JSON escape なので、serde に寄せて escaping 漏れや schema drift を防ぐ。

## Problem

`TestRecord` の `to_json_line` が手書きの JSON escape を使用しており、escaping 漏れや schema drift のリスクがある。

## Current failure

特殊文字を含む TestRecord が不正な JSON を出力する可能性がある。

## Desired final state

`TestRecord` が serde `Serialize` を使用し、常に正しい JSON を出力する。

## Scope

In scope:
- [ ] serde Serialize の derive/実装
- [ ] `to_json_line` の置き換え
- [ ] 既存 consumer の更新

Out of scope:
- [ ] スキーマの変更

## Affected paths

Expected:
- `crates/shared/`

## Acceptance criteria

- [ ] TestRecord が serde Serialize を使用する
- [ ] 特殊文字を含む TestRecord が正しく JSON 出力される
- [ ] 既存の JSON consumer が変更なく動作する

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
