---
id: 5042
title: "[frontend] Complete Stmt AST fixture coverage (audit reopened #5042)"
type: test
area: frontend
class: implementation-ready
priority: P0
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-05status: open
---

## Summary

全 `Stmt` variant に対して fixture を追加し、class/module/control-flow/try-catch などの構文境界を固定する。

## Problem

`Stmt` enum の variant ごとに parse 結果を検証する fixture が不足しており、特に class/module/try-catch の構文対応に抜けがある。

## Current failure

特定の Stmt variant（特に module 宣言、try-catch）の parse エラーが後続パイプラインで初めて検出される。

## Desired final state

全 `Stmt` variant に対する parse → AST snapshot fixture が存在し、構文対応の状態が CI で担保される。

## Scope

In scope:
- [x] 全 Stmt variant の列挙と現状確認
- [x] parse → AST snapshot fixture の追加 (15 new tests, 30 variants covered)
- [x] class/module/try-catch 構文境界の固定

Out of scope:
- [x] semantic analysis (out of scope)

## Affected paths

Expected:
- `crates/frontend/src/`

## Acceptance criteria

- [x] 全 Stmt variant に fixture が存在する
- [x] 不足 variant が可視化される

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

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/done/5042-frontend-stmt-fixture-coverage.md` (moved from open/ per close evidence)

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
