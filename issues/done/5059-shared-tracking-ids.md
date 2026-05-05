---
id: 5059
title: "[shared] Add typed tracking IDs for unsupported and blocked tests"
type: feature
area: coverage
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

`tracking: String` を自由文字列にせず、`issue-xxx` / `feature:xxx` 形式の validation を追加する。

## Problem

unsupported/blocked test の tracking ID が自由文字列であり、issue 番号と feature 名の区別が不明確。

## Current failure

tracking ID の typo や誤形式が検出されない。

## Desired final state

tracking ID が `issue-xxx` / `feature:xxx` 形式に制約され、validation で検査される。

## Scope

In scope:
- [x] tracking ID の型定義
- [x] validation の追加
- [x] 既存 tracking ID の移行

Out of scope:
- [x] 新規 feature の追加

## Affected paths

Expected:
- `crates/shared/`

## Acceptance criteria

- [x] tracking ID が型付けされる
- [x] 不正な形式が validation で検出される
- [x] 既存の正しい tracking ID が移行される

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

## Completion evidence

Implemented `TrackingId` enum with `Issue(u32)` and `Feature(String)` variants
in `crates/shared/src/test_status.rs`. Added `Display` and `FromStr` with format
validation. Replaced `tracking: Option<String>` with `tracking: Option<TrackingId>`
in `TestRecord`. Migrated all tracking IDs in `m2_node_diff.rs`, `official_corpora.rs`,
and `test_infrastructure.rs` to the typed format.

Validation: `cargo test -p ts2wasm-shared` (19 pass, 6 new tracking ID tests),
`cargo nextest run` (pass, 2 pre-existing failures unrelated).

Commits: `864d842f`

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/5059-shared-tracking-ids.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
