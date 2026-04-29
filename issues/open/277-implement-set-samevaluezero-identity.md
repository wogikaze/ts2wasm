---
id: 277
title: "Implement Set SameValueZero identity"
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

Replace the current string-normalized Set key behavior with SameValueZero-compatible identity for the supported value model.

Problem: issue 049 explicitly closed basic Set operations while recording that Map/Set keys are currently normalized through `value_to_string_into`, so string and number identity does not yet match ES SameValueZero.

## Current failure

The following narrow fixture should distinguish number `1` from string `"1"` while keeping duplicate numeric values unique:

```typescript
let s = new Set();
s.add(1);
s.add("1");
s.add(1);
console.log(s.has(1));
console.log(s.has("1"));
console.log(s.size);
```

Expected Node stdout:

```text
true
true
2
```

## Desired final state

Set membership uses SameValueZero-compatible identity for the supported primitive and heap object values, including distinct identity for string `"1"` and number `1`.

## Scope

In scope:

- [ ] Preserve current basic add/has/delete behavior.
- [ ] Distinguish supported primitive types that currently collide through string normalization.
- [ ] Add Node/iwasm differential coverage for number/string distinction and duplicate suppression.

Out of scope:

- WeakSet.
- Full object identity semantics if broader object identity support is missing.
- Iterator ordering APIs.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `fixtures/builtins-and-io/`
- `crates/cli/tests/m2_node_diff.rs`

Do not touch:

- `web-ui/`

## Acceptance criteria

- [ ] A fixture proves `Set` distinguishes `1` from `"1"`.
- [ ] The fixture proves duplicate values do not create duplicate entries.
- [ ] The same identity behavior is covered for `has` and `delete` where supported.
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

Split from stale broad issue 272 and from issue 049's recorded remaining risk.

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
