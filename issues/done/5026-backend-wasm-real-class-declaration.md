---
id: 5026
title: "[backend-wasm] Implement real class declaration emission"
type: feature
area: backend
class: implementation-ready
priority: P0
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
closed: 2026-05-03
---

## Summary

`ClassDecl` が WAT 側で placeholder/TODO になっているため、constructor / prototype / static members / extends / private elements の最小 runtime 表現を実装する。

## Problem

現状の backend-wasm は class declaration の WAT emission がスケルトン実装であり、class 宣言を正しい WASM コードに変換できない。class は L2 互換性の核心機能である。

## Current failure

`crates/backend-wasm/src/expr_emit.rs` などで `ClassDecl` が `todo!()` または未実装のパスで落ちる。該当箇所で reproduction 可能。

## Desired final state

constructor、prototype chain、static members、extends、private elements の最小限の runtime 表現が WAT emission され、class fixture が build pass する。

## Scope

In scope:
- [x] `ClassDecl` の WAT emission 実装
- [x] constructor / prototype / static members の最低限表現
- [x] `extends` の単一継承パス
- [x] private elements の命名規則による分離

Out of scope (not tracked):
- 完全な semantic differential（別 issue）
- 複雑な inheritance chain の最適化

## Affected paths

Expected:
- `crates/backend-wasm/src/`
- `crates/backend-wasm/src/expr_emit.rs`

## Acceptance criteria

- [x] class 宣言を含む fixture が build pass する
- [x] WAT 出力に class の runtime 表現が含まれる
- [x] 回帰テスト用の fixture が追加される

## Completion evidence

- All 8 class fixtures (class-basic, class-expression, class-extends, class-generic, class-static, class-super, class-super-method, new-expression) build pass and run correctly under iwasm
- All 56 IR lowering tests pass, including class-related tests
- `LoweredStmt::ClassDecl` is fully emitted with constructor, methods, static_methods, private_fields
- Class expression support added (anonymous class expressions)
- Backend WAT emission for ClassDecl is live (no todo! paths)

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

## Notes

backend-wasm の ClassDecl emission を、最小限の runtime support と組み合わせて段階的に実装する。
Issues 5046 (IR class IR) completed in parallel; class declaration pipeline (parser → IR → backend) is fully functional for basic class constructs.
