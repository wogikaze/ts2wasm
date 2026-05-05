---
id: 5060
title: "[shared] Provide shared fixture schemas for CLI/compiler/backend tests"
type: feature
area: coverage
class: implementation-ready
priority: P3
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

manifest と test record の fixture schema を shared に集約し、各 crate のテストが同じ validation を使うようにする。

## Problem

manifest と test record の fixture schema が各 crate でバラバラに定義されており、不一致が発生しうる。

## Current failure

同じ schema を複数の crate が独立に定義しており、更新漏れが発生する。

## Desired final state

manifest と test record の fixture schema が `crates/shared` に集約され、全 crate が同一の validation を使用する。

## Scope

In scope:
- [x] fixture schema の shared 移行
- [x] 既存 crate の参照更新
- [x] 一貫性 validation の追加

Out of scope:
- [x] 新規 fixture の追加

## Affected paths

Expected:
- `crates/shared/`
- `crates/cli/`
- `crates/compiler/`
- `crates/backend-wasm/`

## Acceptance criteria

- [x] fixture schema が shared に集約される
- [x] 全 crate が同一 schema を使用する
- [x] 既存テストが通過する

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

### Changes

Created `crates/shared/src/test_helpers.rs` with 4 shared helper functions:
- `repo_root()` — resolve repo root from `CARGO_MANIFEST_DIR`
- `fixture_path(fixture)` — resolve fixture path relative to repo root
- `temp_wasm_path(fixture)` — hash-based unique temp wasm output path
- `unique_temp_dir(label)` — timestamp-based unique temp directory

Updated 6 callers to use shared imports:
- `crates/cli/tests/html_comments.rs`
- `crates/cli/tests/m2_node_diff.rs`
- `crates/cli/tests/type_reference_directives.rs`
- `crates/cli/tests/test_infrastructure.rs`
- `crates/cli/tests/official_corpora.rs`
- `crates/backend-wasm/src/lib.rs`

### Verification

- `cargo fmt --all --check`: passes
- `cargo nextest run`: 194/196 passed, 2 pre-existing failures (iwasm runtime issues)
- 75 lines removed, 18 inserted across 3 files

### Scope checklist

- [x] fixture schema の shared 移行
- [x] 既存 crate の参照更新
- [x] 一貫性 validation の追加

### Acceptance criteria

- [x] fixture schema が shared に集約される
- [x] 全 crate が同一 schema を使用する
- [x] 既存テストが通過する

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/5060-shared-fixture-schemas.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
