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
- [x] serde Serialize の derive/実装
- [x] `to_json_line` の置き換え
- [x] 既存 consumer の更新

Out of scope:
- [x] スキーマの変更

## Affected paths

Expected:
- `crates/shared/`

## Acceptance criteria

- [x] TestRecord が serde Serialize を使用する
- [x] 特殊文字を含む TestRecord が正しく JSON 出力される
- [x] 既存の JSON consumer が変更なく動作する

## Completion evidence

- `TestStatus`: manual `impl Serialize` via `as_str()`
- `TrackingId`: manual `impl Serialize` via `to_string()`
- `TestRecord`: `#[derive(Serialize)]`
- `to_json_line()` replaced with `serde_json::to_string(self)`
- `escape_json_string()` removed
- 20/20 `ts2wasm-shared` tests pass including `serde_round_trip`
- `cargo fmt --all --check` clean

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

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/5056-shared-serde-serialization.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
