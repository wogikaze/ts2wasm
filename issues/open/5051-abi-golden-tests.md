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
- [ ] ABI version 定数の導入
- [ ] golden layout snapshot テスト
- [ ] backend との一致検証

Out of scope:
- [ ] ABI 設計の変更

## Affected paths

Expected:
- `crates/runtime-abi/`

## Acceptance criteria

- [ ] ABI version が定義される
- [ ] golden snapshot テストが存在する
- [ ] 不一致時にテストが fail する

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
