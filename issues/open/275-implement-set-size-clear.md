---
id: 275
title: "Implement Set size and clear"
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

Implement the remaining basic Set state APIs that were outside issue 049's constructor/add/has/delete slice.

Problem: `new Set()` currently has validated constructor/add/has/delete coverage, but `Set.prototype.size` and `Set.prototype.clear` are not covered by the closed basic Set fixture.

## Current failure

No current fixture covers this behavior. The expected narrow reproduction is:

```typescript
let s = new Set();
console.log(s.size);
s.add("a");
s.add("b");
s.add("a");
console.log(s.size);
s.clear();
console.log(s.size);
console.log(s.has("a"));
```

Expected Node stdout:

```text
0
2
0
false
```

## Desired final state

`Set.prototype.size` reflects the number of unique elements in the current supported Set representation, and `Set.prototype.clear` removes all entries.

## Scope

In scope:

- [ ] Add lowering/runtime support for `Set.prototype.size`.
- [ ] Add lowering/runtime support for `Set.prototype.clear`.
- [ ] Add Node/iwasm differential fixture coverage for size, duplicate add, clear, and post-clear `has`.

Out of scope:

- Iterable constructor arguments.
- SameValueZero parity beyond the current collection key representation.
- Set iteration protocol.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `fixtures/builtins-and-io/`
- `crates/cli/tests/m2_node_diff.rs`

Do not touch:

- `web-ui/`

## Acceptance criteria

- [ ] A fixture proves `new Set().size` starts at `0`.
- [ ] The fixture proves duplicate `add` calls do not increase `size`.
- [ ] The fixture proves `clear()` empties the Set and subsequent `has` returns `false`.
- [ ] Node and iwasm stdout match for the fixture.

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

Split from stale broad issue 272 after issue 049 closed the constructor/add/has/delete subset.

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
