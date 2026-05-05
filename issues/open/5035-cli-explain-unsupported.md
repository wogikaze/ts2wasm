---
id: 5035
title: "[cli] Add --explain-unsupported diagnostics mode (audit reopened #5035)"
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

unsupported 診断に、該当 fixture、tracking issue、回避策、次に実装すべき crate を表示する開発支援モードを追加する。

## Problem

unsupported 診断の情報が不十分で、開発者が次に何をすべきか判断しにくい。

## Current failure

unsupported エラー時に tracking issue や回避策が表示されない。

## Desired final state

`--explain-unsupported` フラグで、unsupported 診断の詳細（tracking issue、該当 fixture、回避策、実装優先度）が表示される。

## Scope

In scope:
- [ ] `--explain-unsupported` CLI フラグの追加
- [ ] tracking issue の紐付け
- [ ] 該当 fixture の表示

Out of scope:
- [ ] unsupported ケースの自動修正

## Affected paths

Expected:
- `crates/cli/src/`

## Acceptance criteria

- [ ] `--explain-unsupported` が tracking issue を表示する
- [ ] 該当 fixture パスが表示される

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
- `issues/open/5035-cli-explain-unsupported.md` before this move
- `issues/open/5035-cli-explain-unsupported.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
