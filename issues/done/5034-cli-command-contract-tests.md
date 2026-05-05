---
id: 5034
title: "[cli] Add command contract tests for build/check/dump/server (audit reopened #5034)"
type: test
area: cli
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-05status: open
---

## Summary

CLI サブコマンドごとの exit code、stderr、生成物、manifest 出力を固定し、ユーザー向け挙動を安定させる。

## Problem

CLI サブコマンド（build/check/dump/server）の出力形式や終了コードが暗黙的に決まっており、回帰時に検出できない。

## Current failure

CLI の出力形式変更がテストで捕捉されず、ユーザーに影響する可能性がある。

## Desired final state

全 CLI サブコマンドの exit code、stderr、生成物、manifest 出力が契約テストで固定される。

## Scope

In scope:
- [x] build コマンドの契約テスト
- [x] check コマンドの契約テスト
- [x] dump コマンドの契約テスト
- [x] server コマンドの契約テスト

Out of scope:
- [x] CLI の新機能追加 (not part of this issue)

## Affected paths

Expected:
- `crates/cli/tests/`

## Acceptance criteria

- [x] 各サブコマンドの契約テストが存在する
- [x] 出力変更時にテストが fail する

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
- `issues/open/5034-cli-command-contract-tests.md` -> `issues/done/5034-cli-command-contract-tests.md` (moved to done per close evidence)

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.

## Completion evidence

All four CLI subcommand contract tests were implemented in commit `d8be4677`:

- **build**: 4 tests (valid TS, invalid TS, manifest emit, host deny)
- **check**: 2 tests (valid TS, invalid TS)
- **dump**: 3 tests (all phases, invalid TS, no phase)
- **server**: 2 tests (help, unknown flags)
- **unknown subcommand**: 1 test

### Acceptance verification

- [x] 各サブコマンドの契約テストが存在する — 12 tests covering build/check/dump/server
- [x] 出力変更時にテストが fail する — Contract tests assert specific exit codes, stderr, and output files

### Validation

```sh
cargo nextest run --package ts2wasm-cli --test command_contract
```
Result: 12 passed, 0 failed (requires `npm install` for TypeScript dependency used by `check` command).
