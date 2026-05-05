---
id: 5039
title: "[compiler] Stabilize test262 preprocessor feature handling (audit reopened #5039)"
type: feature
area: cli
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-05status: open
---

## Summary

`features/includes/negative` の対応範囲を明文化し、unsupported feature を tracking ID 付きで分類する。

## Problem

test262 preprocessor の feature/include/negative ハンドリングが暗黙的で、未対応 feature の tracking が不十分。

## Current failure

未対応 feature が tracking ID なしで `unsupported` 扱いになり、improvement の優先順位が不明確。

## Desired final state

全 feature/include/negative が tracking ID 付きで分類され、対応状況が可視化される。

## Scope

In scope:
- [ ] features リストの明文化
- [ ] unsupported feature の tracking ID 分類
- [ ] include/negative ハンドリングの安定化

Out of scope:
- [ ] test262 suite の完全パス

## Affected paths

Expected:
- `crates/cli/`
- `scripts/`

## Acceptance criteria

- [ ] 全 feature が tracking ID 付きで分類される
- [ ] include/negative ハンドリングが安定する

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
- `issues/open/5039-compiler-test262-preprocessor.md` before this move
- `issues/open/5039-compiler-test262-preprocessor.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
