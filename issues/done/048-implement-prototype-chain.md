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
updated: 2026-04-28
completed: 2026-04-28
---

## Summary

Implement prototype chain for object inheritance.

## Problem

Prototype chain is not implemented. It is fundamental to JavaScript's object model.

## Desired final state

Objects have a prototype chain that is traversed for property lookups.

## Scope

In scope:

- [x] Implement prototype slot in objects
- [x] Implement prototype chain traversal
- [x] Implement Object.getPrototypeOf
- [x] Implement Object.setPrototypeOf
- [x] Add fixtures for prototype chain behavior

Out of scope:

- __proto__ accessor (P2)

## Affected paths

Expected:

- `crates/backend-wasm/src/` (runtime)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] Prototype chain is set up correctly
- [x] Property lookup traverses prototype chain
- [x] Object.getPrototypeOf works
- [x] Fixtures cover prototype chain behavior
- [x] No regression in existing fixtures

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

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] 016 (dynamic property keys)
- [x] 030 (instanceof)

## Notes

This is a prerequisite for many features (instanceof, extends, etc.).

## Completion evidence

Commits:

- `b2723fc` issue-048: implement prototype chain runtime slice

Validation result:

```text
command: cargo test -p ts2wasm-cli --test m2_node_diff prototype_chain_fixture_matches_node_output_under_iwasm -- --nocapture
result: pass; prototype fixture output matched Node under iwasm
date: 2026-04-28

command: cargo run -p ts2wasm-cli -- build fixtures/core-semantics/prototype.ts -o /tmp/agent-048-prototype-after.wasm && iwasm /tmp/agent-048-prototype-after.wasm
result: pass; emitted undefined, 2, true, true, 1, 1, 9, true, 9, 11, 18, 10
date: 2026-04-28

command: cargo nextest run -E 'test(prototype_chain_fixture_matches_node_output_under_iwasm)'
result: pass; 1 passed, 198 skipped
date: 2026-04-28

command: cargo nextest run -E 'test(/prototype|object|getPrototypeOf|setPrototypeOf/)'
result: pass; 5 passed, 194 skipped
date: 2026-04-28

command: cargo nextest run prototype_chain_fixture_matches_node_output_under_iwasm
result: pass; 1 passed, 198 skipped
date: 2026-04-28

command: cargo fmt --all --check
result: pass
date: 2026-04-28

command: mise run check-agent-state
result: pass
date: 2026-04-28

command: mise run check-issue-health
result: pass
date: 2026-04-28

command: mise run check-repo-smoke
result: pass
date: 2026-04-28

command: cargo nextest run
result: pass; 195 passed, 4 skipped
date: 2026-04-28

note: The assignment-specified `cargo nextest run -E 'test(prototype|object|getPrototypeOf|setPrototypeOf)'` ran 0 tests and exited 4 because the filter did not match nextest test names; equivalent targeted nextest commands above passed.
```

Remaining risks:

- Ordinary object literals currently start with a null prototype in this runtime subset; fixture coverage avoids claiming full `Object.prototype` semantics.
