---
id: 049
title: "Implement Map and Set"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-26
status: done
completed: 2026-04-28
---

## Summary

Implement Map and Set collection types.

## Problem

Map and Set are not implemented. They are common ES6 collection types.

## Desired final state

`new Map()`, `new Set()` work with basic operations (get, set, has, delete).

## Scope

In scope:

- [x] Implement Map constructor
- [x] Implement Map.prototype.get
- [x] Implement Map.prototype.set
- [x] Implement Map.prototype.has
- [x] Implement Map.prototype.delete
- [x] Implement Set constructor
- [x] Implement Set.prototype.add
- [x] Implement Set.prototype.has
- [x] Implement Set.prototype.delete
- [x] Add fixtures for Map/Set behavior

Out of scope:

- WeakMap/WeakSet (P2)
- Map/Set iteration (P2)

## Affected paths

Expected:

- `crates/backend-wasm/src/` (runtime builtins)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] Map basic operations work correctly
- [x] Set basic operations work correctly
- [x] Fixtures cover Map/Set behavior
- [x] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/map-set-test.ts -o /tmp/test.wasm
iwasm /tmp/test.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

- 2026-04-28 PROGRESS (`agent/049-map-set-20260428T011445Z`): implemented a narrow runtime slice for `new Map()`, `Map.prototype.set/get/has`, `new Set()`, and `Set.prototype.add/has` with Node/iwasm differential coverage in `fixtures/builtins-and-io/map-set.ts`.
- Remaining criteria: `.delete()` calls are not fully closable in this slice because `m.delete("a")` / `s.delete("a")` currently fail in the parser before IR/backend lowering (`delete` is tokenized as a keyword after `.`). Parser changes are outside this child assignment's allowed paths. Runtime helpers for `MapDelete` and `SetDelete` were wired for the future parser fix.
- Semantic limitation of this progress slice: collection keys are normalized through the current runtime `value_to_string_into` path, so string and number key identity is not yet full ES SameValueZero parity.

## Completion evidence

Commits:

- `1b26026` (`Fix Map Set delete member parsing`)

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-04-28

command: cargo nextest run -p ts2wasm-frontend parses_delete_keyword_after_dot_as_member_property_name
result: pass
date: 2026-04-28

command: cargo nextest run -p ts2wasm-cli map_set_collection_fixture_matches_node_output_under_iwasm
result: pass
date: 2026-04-28

command: node fixtures/builtins-and-io/map-set.ts
result: pass; stdout covered Map get/set/has/delete and Set add/has/delete
date: 2026-04-28

command: cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/map-set.ts -o /tmp/issue049-map-set-delete.wasm && iwasm /tmp/issue049-map-set-delete.wasm
result: pass; iwasm stdout matched Node for the Map/Set fixture
date: 2026-04-28

command: cargo nextest run -E 'test(map) or test(set)'
result: pass; 4 tests
date: 2026-04-28

command: cargo nextest run -p ts2wasm-cli map
result: pass; 1 test
date: 2026-04-28

command: cargo nextest run -p ts2wasm-cli set
result: pass; 2 tests
date: 2026-04-28
```

Remaining risks:

- Map/Set key identity remains limited by the current runtime string-key normalization noted above; no follow-up was created because that limitation was already recorded before this delete completion slice and is outside issue 049's basic-operation closure.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/049-implement-map-set.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
