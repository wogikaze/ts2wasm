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
updated: 2026-05-06
status: done
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
- [x] timeout 機構
- [x] 最大並列数制限
- [x] panic/error aggregation

Out of scope:
- [x] 分散コンパイル

## Affected paths

Expected:
- `crates/cli/src/`
- `crates/compiler/src/server.rs`

## Acceptance criteria

- [x] timeout 超過時にジョブが中断される
- [x] 最大並列数が制限される
- [x] panic が aggregation される

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
- `issues/done/5040-compiler-resource-limits.md` before this closure
- `issues/done/5040-compiler-resource-limits.md` after this closure

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.

## Completion evidence

Implemented and verified in commit `55f9bbf9` (`compiler: cover server batch resource limits`).

Code evidence:
- `crates/compiler/src/server.rs` keeps batch timeout handling with timeout-filled per-item responses.
- `crates/compiler/src/server.rs` caps batch worker count through `BatchLimits` and the existing `TS2WASM_SERVER_MAX_WORKERS` path.
- `crates/compiler/src/server.rs` catches per-item panics and reports them as `InvariantViolation` responses instead of crashing the server.

Test evidence:
- `server::tests::batch_timeout_reports_unprocessed_items` verifies timeout overflow returns one error response per unprocessed item.
- `server::tests::batch_worker_count_is_capped` verifies a large requested worker count does not exceed the configured cap.
- `server::tests::batch_panic_is_reported_per_item` verifies a synthetic item panic is aggregated into that item response while surrounding items complete.

Validation:
- `cargo test -p ts2wasm-compiler server::tests::batch_ -- --nocapture` => pass (3 passed)
- `cargo nextest run -p ts2wasm-compiler batch_timeout_reports_unprocessed_items batch_worker_count_is_capped batch_panic_is_reported_per_item` => pass (3 passed)
- `cargo fmt --all --check` => pass
- `git diff --check` => pass
- `python scripts/manager.py check` => pass
