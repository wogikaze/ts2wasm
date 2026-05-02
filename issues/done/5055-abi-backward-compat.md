---
id: 5055
title: "[runtime-abi] Add backward-compatibility tests for ABI constants"
type: test
area: abi
class: implementation-ready
priority: P3
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

将来の定数変更時に breaking change を検出できるよう、旧 version との互換性テストを追加する。

## Problem

ABI 定数の変更が過去の ABI version との互換性を壊すかどうかを検証する仕組みがない。

## Current failure

ABI 定数変更時に後方互換性の破壊が検出されない。

## Desired final state

旧 ABI version の定数との互換性テストが存在し、breaking change が CI で検出される。

## Scope

In scope:
- [x] 旧 ABI version 定数の archive
- [x] 互換性テストの追加

Out of scope:
- [x] runtime migration

## Affected paths

Expected:
- `crates/runtime-abi/`

## Acceptance criteria

- [x] 旧 version との互換性テストが存在する
- [x] breaking change 時にテストが fail する

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
