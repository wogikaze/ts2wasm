---
id: 5038
title: "[compiler] Harden module graph resolution and diagnostics"
type: feature
area: cli
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

local `.ts/.js` 以外、missing module、cycle、default/named export mismatch の診断と graph order をより厳密にする。

## Problem

現在の module graph resolution は正常系に重点を置いており、エラーケース（missing module、cycle、export mismatch）の診断が弱い。

## Current failure

不正な module graph が検出されず、実行時エラーになる。

## Desired final state

module graph resolution が以下を確実に診断する：missing module、cycle、default/named export mismatch、不正な graph order。

## Scope

In scope:
- [ ] missing module 診断の追加
- [ ] cycle 検出と診断
- [ ] default/named export mismatch 診断
- [ ] graph order 検証

Out of scope:
- [ ] 外部 package resolution
- [ ] dynamic import

## Affected paths

Expected:
- `crates/cli/`
- `crates/ir/`

## Acceptance criteria

- [ ] 各エラーケースの診断 test fixture が追加される
- [ ] 診断が source span 付きで報告される

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
