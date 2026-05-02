---
id: 5058
title: "[shared] Deduplicate and canonicalize capability reasons/imports"
type: refactor
area: coverage
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

capability reason や node host import が重複しうるため、安定順序・重複排除・validation を追加する。

## Problem

capability reason と node host import が重複して記録される可能性があり、順序も非決定論的。

## Current failure

同一 capability が複数回 manifest に記録される。

## Desired final state

capability reason と import が重複排除され、安定した順序で出力される。

## Scope

In scope:
- [ ] 重複排除ロジック
- [ ] 安定順序の実装
- [ ] validation の追加

Out of scope:
- [ ] manifest schema の変更

## Affected paths

Expected:
- `crates/shared/`

## Acceptance criteria

- [ ] 重複 capability が排除される
- [ ] 出力順序が安定している
- [ ] validation が存在する

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
