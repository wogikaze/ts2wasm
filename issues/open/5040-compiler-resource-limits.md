---
id: 5040
title: "[compiler] Add resource limits and cancellation to server batch compilation (audit reopened #5040)"
type: feature
area: cli
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-05status: open
---

## Summary

server batch 実行に timeout、最大並列数、panic/error aggregation を入れ、巨大入力で落ちにくくする。

## Problem

現在の server batch compilation は resource limit がなく、巨大な入力や並列バッチで panic や OOM が発生する。

## Current failure

巨大なバッチ入力で server が panic し、他のジョブに影響を与える。

## Desired final state

server batch に timeout、最大並列数制限、panic/error aggregation が実装される。

## Scope

In scope:
- [ ] timeout 機構
- [ ] 最大並列数制限
- [ ] panic/error aggregation

Out of scope:
- [ ] 分散コンパイル

## Affected paths

Expected:
- `crates/cli/src/`

## Acceptance criteria

- [ ] timeout 超過時にジョブが中断される
- [ ] 最大並列数が制限される
- [ ] panic が aggregation される

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

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/5040-compiler-resource-limits.md` before this move
- `issues/open/5040-compiler-resource-limits.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
