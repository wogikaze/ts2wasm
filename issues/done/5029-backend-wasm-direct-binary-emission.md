---
id: 5029
title: "[backend-wasm] Expand direct wasm binary emission beyond console.log string literal MVP (audit reopened #5029)"
type: feature
area: backend
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-05status: open
---

## Summary

direct wasm binary emitter が `console.log(<string literal>)` だけなので、数値、ローカル、基本式、複数文を段階的に対応する。

## Problem

現在の direct wasm binary emitter は MVP として console.log の文字列リテラルのみ対応しており、数値出力や変数参照、基本式など一般的なケースで WAT 経由に fallback している。

## Current failure

数値やローカル変数の console.log が WAT emitter に fallback し、direct binary emission の恩恵を受けられない。

## Desired final state

数値リテラル、ローカル変数、基本式、複数文の console.log が direct wasm binary emitter で出力される。

## Scope

In scope:
- [ ] 数値リテラルの binary emission
- [ ] ローカル変数の読み取り
- [ ] 基本式（加算、比較）
- [ ] 複数文のシーケンス

Out of scope:
- [ ] 複雑な control flow
- [ ] 最適化を伴う emission

## Affected paths

Expected:
- `crates/backend-wasm/src/`

## Acceptance criteria

- [ ] 数値 console.log fixture が direct binary で出力される
- [ ] ローカル変数 fixture が direct binary で出力される
- [ ] 既存 WAT emitter との出力一致テストが通る

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
- `issues/open/5029-backend-wasm-direct-binary-emission.md` before this move
- `issues/open/5029-backend-wasm-direct-binary-emission.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
