---
id: 5039
title: "[compiler] Stabilize test262 preprocessor feature handling"
type: feature
area: cli
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

`features/includes/negative` の対応範囲を明文化し、unsupported feature を tracking ID 付きで分類する。

## Problem

test262 preprocessor の feature/include/negative ハンドリングが暗黙的で、未対応 feature の tracking が不十分。

## Current failure

未対応 feature が tracking ID なしで `unsupported` 扱いになり、improvement の優先順位が不明確。

## Desired final state

全 feature/include/negative が tracking ID 付きで分類され、対応状況が可視化される。

## Scope

In scope:
- [ ] features リストの明文化
- [ ] unsupported feature の tracking ID 分類
- [ ] include/negative ハンドリングの安定化

Out of scope:
- [ ] test262 suite の完全パス

## Affected paths

Expected:
- `crates/cli/`
- `scripts/`

## Acceptance criteria

- [ ] 全 feature が tracking ID 付きで分類される
- [ ] include/negative ハンドリングが安定する

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
