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
---

## Summary

Implement Map and Set collection types.

## Problem

Map and Set are not implemented. They are common ES6 collection types.

## Desired final state

`new Map()`, `new Set()` work with basic operations (get, set, has, delete).

## Scope

In scope:

- [ ] Implement Map constructor
- [ ] Implement Map.prototype.get
- [ ] Implement Map.prototype.set
- [ ] Implement Map.prototype.has
- [ ] Implement Map.prototype.delete
- [ ] Implement Set constructor
- [ ] Implement Set.prototype.add
- [ ] Implement Set.prototype.has
- [ ] Implement Set.prototype.delete
- [ ] Add fixtures for Map/Set behavior

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

- [ ] Map basic operations work correctly
- [ ] Set basic operations work correctly
- [ ] Fixtures cover Map/Set behavior
- [ ] No regression in existing fixtures

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

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

- 2026-04-28 PROGRESS (`agent/049-map-set-20260428T011445Z`): implemented a narrow runtime slice for `new Map()`, `Map.prototype.set/get/has`, `new Set()`, and `Set.prototype.add/has` with Node/iwasm differential coverage in `fixtures/builtins-and-io/map-set.ts`.
- Remaining criteria: `.delete()` calls are not fully closable in this slice because `m.delete("a")` / `s.delete("a")` currently fail in the parser before IR/backend lowering (`delete` is tokenized as a keyword after `.`). Parser changes are outside this child assignment's allowed paths. Runtime helpers for `MapDelete` and `SetDelete` were wired for the future parser fix.
- Semantic limitation of this progress slice: collection keys are normalized through the current runtime `value_to_string_into` path, so string and number key identity is not yet full ES SameValueZero parity.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
