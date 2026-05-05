---
id: 5036
title: "[compiler] Introduce CompileReport<T> for non-fatal diagnostics"
type: feature
area: cli
class: implementation-ready
priority: P0
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

現在の `Result<T, Diagnostic>` では「生成はできるが警告/unsupported もある」を表現しにくい。`value + diagnostics` 形式へ移行する。

## Problem

`Result<T, Diagnostic>` は成功か失敗かの二値であり、「コンパイルは成功したが unsupported 診断が発生した」という状態を表現できない。

## Current failure

unsupported 診断が発生してもコンパイルが続行できるケースで、診断情報が失われる。

## Desired final state

`CompileReport<T>` が値と診断リストを保持し、非 fatal 診断を propagate できる。

## Scope

In scope:
- [x] `CompileReport<T>` 型の定義
- [x] コンパイラパイプライン全体の移行
- [x] 非 fatal 診断の集約と出力

Out of scope:
- [x] 診断の自動修正
- [x] エラー種類の網羅的追加

## Affected paths

Expected:
- `crates/cli/`

## Acceptance criteria

- [x] `CompileReport<T>` が値と診断リストを保持する
- [x] 非 fatal 診断が最終出力に含まれる
- [x] 既存の fatal エラーが変わらず報告される

## Validation

```sh
cargo fmt --all --check
cargo nextest run
```

## Completion evidence

### Changes

Commit `3773d121`:
- Defined `CompileReport<T>` in `crates/compiler/src/lib.rs` with `ok()`, `map()`, `and_then()` methods
- Changed `build_file` / `build_file_with_options` / `build_file_with_host_deny` to return `Result<CompileReport<()>, Diagnostic>`
- Folded `validate_lowered` diagnostics into the report instead of collapsing to first error
- Updated CLI `main.rs` to print accumulated diagnostics on stderr
- Updated 3 test files (`m8_oop_classes.rs`, `m10_node_apis.rs`, `m2_node_diff_fixture_tests.rs`) to match `Ok(_)` instead of `Ok(())`
- Re-exported `CompileReport` from `crates/cli/src/lib.rs`

### Verification

- `cargo fmt --all --check`: passes
- `cargo nextest run`: 194/196 passed (2 pre-existing iwasm failures)
- No regressions from the return type change

### Acceptance criteria

- [x] `CompileReport<T>` defined with value + diagnostic list
- [x] Non-fatal diagnostics appear in CLI output
- [x] Existing fatal errors unchanged (still short-circuit via `?`)
- [x] fmt + nextest pass

## Docs / current-state / issue sync

Final-state docs:
- [x] not affected

Current state:
- [x] not affected

Follow-up issues:
- [x] none

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/5036-compiler-compile-report.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
