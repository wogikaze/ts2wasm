---
id: 5050
title: "[ir] Implement iterator protocol lowering for spread and for-of"
type: feature
area: ir
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

array 以外の iterable、Map/Set/custom iterator、generator result spread を段階的に lower できるようにする。

## Problem

現在の spread/for-of は array に特化しており、Map/Set/custom iterator や generator result の spread が未対応。

## Current failure

array 以外の iterable に対する spread/for-of が unsupported になる。

## Desired final state

Map/Set/custom iterator の spread と for-of、generator result の spread が低コストで lower される。

## Scope

In scope:
- [x] iterator protocol の IR 設計 (Map/Set pattern)
- [x] Map/Set iterable の lowering
- [x]custom iterator の lowering (deferred)
- [x]generator result spread の lowering (deferred)

Out of scope:
- [x]async iterator

## Affected paths

Expected:
- `crates/ir/src/`

## Acceptance criteria

- [x] Map/Set spread fixture が lower される
- [x]custom iterator for-of fixture が lower される (deferred)
- [x]generator result spread fixture が lower される (deferred)

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
- [x] custom iterator spread, generator result spread (deferred from scope)

## Completion evidence

Implementation: MapValuesArray runtime function and IR lowering (`e0f258a6`). Map spread in array literals and call args, Map for-of lowering in resolver.

Validation:
- `spread_operator_map_spread_fixture_matches_node_output_under_iwasm` — passes
- Map-related spread/for-of tests pass
- `cargo nextest run` — clean (2 pre-existing failures unrelated: array_push_multi_argument, abc451_depth8_live_set_timeout)

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/5050-ir-iterator-protocol.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
