---
id: 048
title: "Implement prototype chain"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement prototype chain for object inheritance.

## Problem

Prototype chain is not implemented. It is fundamental to JavaScript's object model.

## Desired final state

Objects have a prototype chain that is traversed for property lookups.

## Scope

In scope:

- [ ] Implement prototype slot in objects
- [ ] Implement prototype chain traversal
- [ ] Implement Object.getPrototypeOf
- [ ] Implement Object.setPrototypeOf
- [ ] Add fixtures for prototype chain behavior

Out of scope:

- __proto__ accessor (P2)

## Affected paths

Expected:

- `crates/backend-wasm/src/` (runtime)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] Prototype chain is set up correctly
- [ ] Property lookup traverses prototype chain
- [ ] Object.getPrototypeOf works
- [ ] Fixtures cover prototype chain behavior
- [ ] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/prototype-test.ts -o /tmp/test.wasm
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

- [ ] 016 (dynamic property keys)
- [ ] 030 (instanceof)

## Notes

This is a prerequisite for many features (instanceof, extends, etc.).

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
