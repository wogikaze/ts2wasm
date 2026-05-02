---
id: 5057
title: "[shared] Version capability manifest schema and migration policy"
type: feature
area: coverage
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

`schema_version: 1` 前提を明文化し、将来の manifest 変更時の compatibility/migration 方針を追加する。

## Problem

capability manifest の schema version が暗黙的であり、将来の変更時の互換性方針が未定義。

## Current failure

manifest schema 変更時の migration 手順が不明。

## Desired final state

schema version が明示され、version 間の互換性と migration 方針が文書化される。

## Scope

In scope:
- [ ] schema version の明示
- [ ] backward compatibility 方針
- [ ] migration 手順の文書化

Out of scope:
- [ ] manifest content の変更

## Affected paths

Expected:
- `crates/shared/`
- `docs/`

## Acceptance criteria

- [ ] schema version が明示される
- [ ] migration 方針が文書化される

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
