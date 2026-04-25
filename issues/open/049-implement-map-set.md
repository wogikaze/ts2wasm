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

- `crates/cli/src/backend/` (runtime builtins)
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
