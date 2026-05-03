---
id: 5027
title: "[backend-wasm] Replace throw-as-return with catchable exception runtime"
type: feature
area: backend
class: implementation-ready
priority: P0
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

現状 `throw` が実質 return 扱いになっているため、`try/catch/finally` と整合する pending exception / catch binding / finally 実行順序を実装する。

## Problem

throw が単なる早期 return として実装されており、try/catch/finally の正しい制御フロー（catch binding、finally の実行保証、例外の再スロー）を表現できない。

## Current failure

try/catch を含む fixture が throw を正しく catch できず、実行結果が Node と一致しない。

## Desired final state

throw が WAT 上の pending exception 機構を通して伝播し、try/catch/finally が正しい順序で評価される。

## Scope

In scope:
- [ ] Pending exception の runtime 表現
- [ ] catch binding（`catch (e)`）の実装
- [ ] finally ブロックの実行保証
- [ ] 再スローのサポート

Out of scope:
- [ ] カスタムエラー型の完全互換
- [ ] stack trace の互換性

## Affected paths

Expected:
- `crates/backend-wasm/src/`

## Acceptance criteria

- [ ] try/catch/finally fixture が Node differential で一致
- [ ] catch binding が正しい値を持つ
- [ ] finally が常に実行される

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
