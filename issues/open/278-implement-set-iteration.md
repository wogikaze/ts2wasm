---
id: 278
title: "Implement Set iteration"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Implement the observable Set iteration APIs as a separate slice from basic membership operations.

Problem: issue 049 deliberately left Map/Set iteration out of scope, and issue 272's broad "Test262 Set tests pass" criterion cannot be executed until iteration behavior has its own work order.

## Current failure

No current fixture covers Set iteration order. The expected narrow reproduction is:

```typescript
let s = new Set();
s.add("a");
s.add("b");
for (const value of s) {
  console.log(value);
}
```

Expected Node stdout:

```text
a
b
```

## Desired final state

Supported Set iteration preserves insertion order and works through the same iterator path used by `for...of` and future spread support.

## Scope

In scope:

- [ ] Support `for...of` over Set values for the current Set representation.
- [ ] Add or wire `Set.prototype.values` and `Set.prototype[Symbol.iterator]` as needed for the supported slice.
- [ ] Add Node/iwasm differential coverage for insertion-order iteration.

Out of scope:

- Map iteration.
- Mutation-during-iteration edge cases.
- Full iterator closing semantics.
- Spread syntax; issue 274 tracks spread operator work and should consume this behavior when available.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `fixtures/builtins-and-io/`
- `crates/cli/tests/m2_node_diff.rs`

Do not touch:

- `web-ui/`

## Acceptance criteria

- [ ] A fixture proves `for...of` over Set emits values in insertion order.
- [ ] Duplicate `add` calls do not produce duplicate iteration entries.
- [ ] Node and iwasm stdout match for the fixture.
- [ ] Issue 274 remains the spread syntax owner unless spread is explicitly included in a future assignment.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli set
```

Impacted commands:

```sh
cargo nextest run -p ts2wasm-cli map_set_collection_fixture_matches_node_output_under_iwasm
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

Split from stale broad issue 272 after issue 049 closed the basic constructor/add/has/delete subset.

**Current implementation limitation**: The current for...of implementation only supports arrays. Implementing Set iteration requires:
1. Adding iterator protocol support for Set
2. Implementing Set.prototype.values and Set.prototype[Symbol.iterator]
3. Modifying the for...of lowering to handle Set iterables
4. Ensuring insertion order is preserved

This requires substantial changes to the iteration infrastructure.

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

- Requires substantial changes to iteration infrastructure
- May impact Map iteration as well
