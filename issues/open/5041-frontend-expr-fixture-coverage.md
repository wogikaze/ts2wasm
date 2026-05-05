---
id: 5041
title: "[frontend] Complete Expr AST fixture coverage (audit reopened #5041)"
type: test
area: frontend
class: implementation-ready
priority: P0
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-05
status: open
---

## Summary

全 `Expr` variant に対して parse → AST snapshot/構造検証 fixture を追加し、構文対応の抜けを可視化する。

## Problem

`Expr` enum の variant ごとに parse 結果を検証する fixture が不足しており、構文対応の網羅性が不明。

## Current failure

特定の Expr variant が parse できないことが、テスト failure でなく runtime error として初めて発覚する。

## Desired final state

全 `Expr` variant に対する parse → AST snapshot fixture が存在し、構文対応の状態が CI で担保される。

## Scope

In scope:
- [x] 全 Expr variant の列挙と現状確認
- [x] parse → AST snapshot fixture の追加 (3 uncovered variants: LogicalAssign, IndexAssign, ClassExpr)
- [x] 不足 variant の対応計画 (noted 3 remaining uncovered variants)

Out of scope:
- [x] semantic analysis (out of scope)

## Affected paths

Expected:
- `crates/frontend/src/`

## Acceptance criteria

- [x] 全 Expr variant に fixture が存在する
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
- `issues/open/5041-frontend-expr-fixture-coverage.md` (moved back from done/ per audit, no completion evidence added)

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
