---
id: 5060
title: "[shared] Provide shared fixture schemas for CLI/compiler/backend tests"
type: feature
area: coverage
class: implementation-ready
priority: P3
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

manifest と test record の fixture schema を shared に集約し、各 crate のテストが同じ validation を使うようにする。

## Problem

manifest と test record の fixture schema が各 crate でバラバラに定義されており、不一致が発生しうる。

## Current failure

同じ schema を複数の crate が独立に定義しており、更新漏れが発生する。

## Desired final state

manifest と test record の fixture schema が `crates/shared` に集約され、全 crate が同一の validation を使用する。

## Scope

In scope:
- [ ] fixture schema の shared 移行
- [ ] 既存 crate の参照更新
- [ ] 一貫性 validation の追加

Out of scope:
- [ ] 新規 fixture の追加

## Affected paths

Expected:
- `crates/shared/`
- `crates/cli/`
- `crates/compiler/`
- `crates/backend-wasm/`

## Acceptance criteria

- [ ] fixture schema が shared に集約される
- [ ] 全 crate が同一 schema を使用する
- [ ] 既存テストが通過する

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
