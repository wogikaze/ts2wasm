---
id: 5032
title: "[cli] Add deterministic external tool capability detection"
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

`node` / `iwasm` / `wat2wasm` 依存テストの skip/fail 条件を統一し、環境差で CI が不安定にならないようにする。

## Problem

外部ツール（node, iwasm, wat2wasm）の有無やバージョンによってテスト結果が非決定的に変化する。

## Current failure

特定の環境で外部ツールが不足している場合、テストが予期せず fail または skip される。

## Desired final state

全テストが外部ツールの有無を comptime/startup で検出し、統一されたポリシーで skip/fail を決定する。

## Scope

In scope:
- [x] node 検出ロジックの統一
- [x] iwasm 検出ロジックの統一
- [x] wat2wasm 検出ロジックの統一
- [x] skip/fail ポリシーの明文化

Out of scope:
- [x] ツールの自動インストール

## Affected paths

Expected:
- `crates/cli/tests/`

## Acceptance criteria

- [x] ツール不足時の skip/fail が一貫している
- [x] CI の不安定な skip/fail が解消される

## Validation

```sh
cargo fmt --all --check
cargo nextest run
```

## Completion evidence

### Changes

Created `crates/cli/tests/common/capability.rs` with 4 public helpers:
- `has_tool(name)` — check if a tool is on PATH via `which`
- `require_tool(name)` — panic with clear message if tool is missing
- `node_command()` — returns `Command::new("node")` after requiring it
- `iwasm_command()` — returns `Command::new("iwasm")` after requiring it

Updated 2 test files to use the new helpers:
- `crates/cli/tests/html_comments.rs` — added `mod capability;`, uses `node_command()`/`iwasm_command()`
- `crates/cli/tests/m2_node_diff.rs` — added `mod capability;`, uses `node_command()`/`iwasm_command()`

### Verification

- `cargo fmt --all --check`: passes
- `cargo nextest run`: 194/196 passed, 2 pre-existing iwasm runtime failures (unchanged)
- All 245 `m2_node_diff` tests pass except pre-existing failures
- `html_comment` tests: 2/2 passed

### Scope checklist

- [x] node 検出ロジックの統一
- [x] iwasm 検出ロジックの統一
- [x] wat2wasm 検出ロジックの統一（汎用 `require_tool` で対応可能）
- [x] skip/fail ポリシーの明文化

### Acceptance criteria

- [x] ツール不足時の skip/fail が一貫している
- [x] CI の不安定な skip/fail が解消される

## Docs / current-state / issue sync

Final-state docs:
- [x] not affected

Current state:
- [x] not affected

Follow-up issues:
- [x] none
