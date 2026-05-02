---
id: 5049
title: "[ir] Complete destructuring, rest, and default binding lowering"
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

binding pattern の rest/default/parameter destructuring など issue-251/247 系を、name resolution と lowered IR 両方で実装する。

## Problem

destructuring binding の rest/default/parameter destructuring が name resolution / lowered IR で不完全。

## Current failure

入れ子の destructuring や rest binding、デフォルト値付き parameter destructuring が正しく lower されない。

## Desired final state

issue-251/247 系の destructuring/rest/default binding が name resolution と lowered IR で完全に実装される。

## Scope

In scope:
- [ ] rest binding の complete lowering
- [ ] default initializer の lowering
- [ ] 入れ子 destructuring の完全対応
- [ ] issue-251/247 系の整理

Out of scope:
- [ ] 任意の iterator protocol

## Affected paths

Expected:
- `crates/ir/src/`

## Acceptance criteria

- [ ] rest binding fixture の lowering が通る
- [ ] default value fixture の lowering が通る
- [ ] 入れ子 destructuring fixture の lowering が通る

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
