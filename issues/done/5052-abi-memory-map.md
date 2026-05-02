---
id: 5052
title: "[runtime-abi] Validate runtime memory map for overlap and headroom"
type: feature
area: abi
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

`SCRATCH`, `STDIN`, `DATA_START`, `HEAP_START`, GC headroom などの領域衝突を compile-time/unit test で検出する。

## Problem

runtime の memory map 領域（SCRATCH, STDIN, DATA_START, HEAP_START, GC headroom など）の重複や不足が静的に検証されていない。

## Current failure

領域の重複や headroom 不足が runtime crash として初めて検出される。

## Desired final state

memory map の全領域が compile-time または unit test で重複・headroom 検証される。

## Scope

In scope:
- [x] 全 memory region の定義とオフセット
- [x] 重複検出テスト
- [x] headroom 検証テスト

Out of scope:
- [x] dynamic memory management

## Affected paths

Expected:
- `crates/runtime-abi/`

## Acceptance criteria

- [x] 全 region の重複検出テストが通る
- [x] headroom 検証テストが通る

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
