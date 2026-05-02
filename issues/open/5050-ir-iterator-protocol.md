---
id: 5050
title: "[ir] Implement iterator protocol lowering for spread and for-of"
type: feature
area: ir
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

array 以外の iterable、Map/Set/custom iterator、generator result spread を段階的に lower できるようにする。

## Problem

現在の spread/for-of は array に特化しており、Map/Set/custom iterator や generator result の spread が未対応。

## Current failure

array 以外の iterable に対する spread/for-of が unsupported になる。

## Desired final state

Map/Set/custom iterator の spread と for-of、generator result の spread が低コストで lower される。

## Scope

In scope:
- [ ] iterator protocol の IR 設計
- [ ] Map/Set iterable の lowering
- [ ] custom iterator の lowering
- [ ] generator result spread の lowering

Out of scope:
- [ ] async iterator

## Affected paths

Expected:
- `crates/ir/src/`

## Acceptance criteria

- [ ] Map/Set spread fixture が lower される
- [ ] custom iterator for-of fixture が lower される
- [ ] generator result spread fixture が lower される

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
