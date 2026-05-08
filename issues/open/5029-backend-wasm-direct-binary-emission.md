---
id: 5029
title: "[backend-wasm] Expand direct wasm binary emission beyond console.log string literal MVP (audit reopened #5029)"
type: feature
area: backend
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
completed: 2026-05-06
updated: 2026-05-06
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
- [x] 数値リテラルの binary emission
- [x] ローカル変数の読み取り
- [x] 基本式（加算、比較）
- [x] 複数文のシーケンス

Out of scope:
- [x] 複雑な control flow
- [x] 最適化を伴う emission

## Affected paths

Expected:
- `crates/backend-wasm/src/`

## Acceptance criteria

- [x] 数値 console.log fixture が direct binary で出力される
- [x] ローカル変数 fixture が direct binary で出力される
- [x] 既存 WAT emitter との出力一致テストが通る

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

### Implementation commits

- `07a793d3, 7915b56a, 1be2c987` — Expand direct wasm binary emission beyond console.log string literal

### Changed files

- crates/backend-wasm/src/binary_mvp.rs, crates/backend-wasm/src/

### Validation

```sh
cargo test -p ts2wasm-backend-wasm => PASS
```

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/done/5029-backend-wasm-direct-binary-emission.md` (moved back from done/ per audit, no completion evidence added)

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
