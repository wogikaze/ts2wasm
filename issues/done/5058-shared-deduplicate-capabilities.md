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
- [x] 重複排除ロジック
- [x] 安定順序の実装
- [x] validation の追加

Out of scope:
- [x] manifest schema の変更

## Affected paths

Expected:
- `crates/shared/`

## Acceptance criteria

- [x] 重複 capability が排除される
- [x] 出力順序が安定している
- [x] validation が存在する

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

## Completion evidence

### Changes

- **`crates/shared/src/capability.rs`**: Added `canonicalize()` method that deduplicates capability reasons values (per-key, preserving insertion order) and deduplicates + sorts node host imports. `to_json()` now calls `canonicalize()` on a clone before serialization.
- **3 new tests**:
  - `capability_reasons_deduplicated_in_to_json` — verifies duplicate reasons appear once in JSON output
  - `node_host_imports_deduplicated_and_sorted` — verifies dedup + alphabetical sort of imports
  - `duplicate_reason_values_are_deduplicated` — verifies triple-dedup reduces to one

### Test pass

```
24/24 tests pass in ts2wasm-shared (cargo nextest run)
```

### Formatting

```
cargo fmt --all --check — clean
```

Follow-up issues:
- [x] none

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/5058-shared-deduplicate-capabilities.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
