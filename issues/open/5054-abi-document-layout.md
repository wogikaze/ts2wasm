---
id: 5054
title: "[runtime-abi] Document value tags and object layout as public ABI"
type: docs
area: abi
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

backend/runtime/shared が同じ前提を参照できるよう、value tag、heap object、array、BigInt、GC header の仕様を docs 化する。

## Problem

value tag、heap object layout、array/BigInt 表現、GC header が暗黙的な知識であり、backend/runtime/shared 間で共通理解が困難。

## Current failure

ABI の仕様変更時に他 crate との整合性を手動で確認する必要がある。

## Desired final state

value tag、heap object、array、BigInt、GC header のレイアウト仕様が `docs/` で文書化される。

## Scope

In scope:
- [ ] value tag 一覧と定義
- [ ] heap object layout 仕様
- [ ] array/BigInt 表現
- [ ] GC header 仕様

Out of scope:
- [ ] runtime 実装の詳細

## Affected paths

Expected:
- `docs/`
- `crates/runtime-abi/`

## Acceptance criteria

- [ ] value tag が文書化される
- [ ] 各 heap object の layout が文書化される
- [ ] GC header が文書化される

## Validation

```sh
cargo fmt --all --check
```

## Docs / current-state / issue sync

Final-state docs:
- [ ] updated: `docs/14-runtime-abi.md`
- [ ] updated: `docs/00-docs-list.md`

Current state:
- [ ] not affected

Follow-up issues:
- [ ] none
