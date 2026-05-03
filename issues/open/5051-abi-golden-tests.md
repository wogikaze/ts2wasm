---
id: 5051
title: "[runtime-abi] Add ABI layout golden tests and versioning"
type: test
area: abi
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

layout/tag/offset 定数の変更が backend runtime とずれないよう、ABI version と golden snapshot を導入する。

## Problem

runtime-abi の定数（layout/tag/offset）が暗黙的に backend と同期しており、変更時のずれを検出できない。

## Current failure

ABI 定数の変更が backend runtime と不一致を起こし、runtime error として初めて検出される。

## Desired final state

ABI layout の golden snapshot テストと versioning により、定数変更時の不一致を compile-time で検出する。

## Scope

In scope:
- [x] ABI version 定数の導入
- [x] golden layout snapshot テスト
- [x] backend との一致検証

Out of scope:
- [x] ABI 設計の変更

## Affected paths

Expected:
- `crates/runtime-abi/`

## Acceptance criteria

- [x] ABI version が定義される
- [x] golden snapshot テストが存在する
- [x] 不一致時にテストが fail する

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

**Why this was false-done**: This issue was created in the review-derived
batch (commit `2c655baf`) and placed in `issues/done/` without any
implementation work. There are zero feat/fix commits referencing #5051.
No completion evidence section exists. The issue has never been implemented.

**True-done checklist** (all must pass):

1. Implement ABI layout golden tests and versioning
2. Add completion evidence section with implementation details
3. Verify with acceptance criteria

**Commands that must pass**:

```sh
cargo fmt --all --check
cargo nextest run
```
