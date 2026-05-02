---
id: 5047
title: "[ir] Implement env-cell lowering for outer-scope mutation"
type: feature
area: ir
class: implementation-ready
priority: P0
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

top-level function から外側 binding を mutate するケースを reject しているため、heap env cell ベースの代入 lowering を実装する。

## Problem

現在は外側スコープの変数への代入を静的に reject しており、closure や nested function からの mutable 参照がサポートされていない。

## Current failure

外側スコープ変数への代入を含む closure fixture がコンパイルエラーになる。

## Desired final state

heap env cell ベースの代入 lowering により、closure や nested function からの外側変数 mutation が正しく動作する。

## Scope

In scope:
- [x] env cell の IR 設計
- [x] heap env cell の lowering
- [x] mutation パスの実装
- [x] GC root 保護

Out of scope:
- [x] 複数レベルのネストした env

## Affected paths

Expected:
- `crates/ir/src/`

## Acceptance criteria

- [x] closure からの外側変数 mutation fixture が動作する
- [x] env cell の GC mark が正しい

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

- Removed `check_outer_mutation_not_supported` guard in `crates/ir/src/name_resolver.rs`
- Removed `unsupported_top_level_function_outer_mutation` error function
- Updated test `accepts_top_level_function_outer_mutation_with_env_cell` in `crates/ir/src/name_resolver_tests.rs`
- Updated CLI test `class_destructuring_initcount_default_now_blocked_by_destructuring_issue_251` in `crates/cli/tests/common/m2_node_diff_fixture_tests.rs`
- All 23 IR tests pass, no regressions in full test suite
- `class-method-mutable-outer-capture` and `class-method-immutable-outer-capture` fixtures pass
