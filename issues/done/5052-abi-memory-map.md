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

---

## ⚠️ False-done audit (re-opened from issues/done/)

**Why this was false-done**: Created directly in `issues/done/` as part of a review-derived batch (commit `2c655baf`, issues 5026-5060) without any implementation. The `## Completion evidence` section is entirely absent. Commit `9496b22f` claims to add memory region tests in its message but the actual diff only modifies `expressions_main.rs` — no memory map validation was ever added. The current `crates/runtime-abi/src/layout.rs` contains no region/overlap/headroom tests.

**True-done checklist** (all must pass):

1. **Add compile-time or unit-test validation** for all memory regions: SCRATCH, STDIN, DATA_START, HEAP_START, and GC headroom — detecting overlap and insufficient headroom.

2. **Commands that must pass**:
   ```sh
   cargo fmt --all --check
   cargo nextest run
   ```

3. **Specific evidence needed**:
   - Memory region overlap detection test passes
   - Headroom validation test passes
   - Completion evidence section filled with commit SHAs and validation results
