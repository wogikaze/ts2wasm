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
- [x] schema version の明示
- [x] backward compatibility 方針
- [x] migration 手順の文書化

Out of scope:
- [x] manifest content の変更

## Affected paths

Expected:
- `crates/shared/`
- `docs/`

## Acceptance criteria

- [x] schema version が明示される
- [x] migration 方針が文書化される

## Completion evidence

- Added `pub const SCHEMA_VERSION: u32 = 1` to `crates/shared/src/capability.rs`
- Updated `new_wasi()` and `validate()` to use the named constant
- Added `schema_version_is_explicit_named_constant` test
- Added "Schema versioning and migration policy" section to `docs/11-shared-definitions.md`
- 21/21 `ts2wasm-shared` tests pass
- `cargo fmt --all --check` clean

## Validation

```sh
cargo fmt --all --check
cargo nextest run
```

## Docs / current-state / issue sync

Final-state docs:
- [x] docs/11-shared-definitions.md — added schema versioning section

Current state:
- [x] not affected

Follow-up issues:
- [x] none

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/5057-shared-manifest-versioning.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
