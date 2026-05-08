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

Current behavior (string normalization):

```text
true
true
1
```

## Desired final state

Set membership uses SameValueZero-compatible identity for the supported primitive and heap object values, including distinct identity for string `"1"` and number `1`.

## Scope

In scope:

- [x] Preserve current basic add/has/delete behavior.
- [x] Distinguish supported primitive types that currently collide through string normalization.
- [x] Add Node/iwasm differential coverage for number/string distinction and duplicate suppression.

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

- [x] A fixture proves `Set` distinguishes `1` from `"1"`.
- [x] The fixture proves duplicate values do not create duplicate entries.
- [x] The same identity behavior is covered for `has` and `delete` where supported.
- [x] Node and iwasm stdout match for the fixture.

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

- not affected

Current state:

- not affected

Follow-up issues:

- none

## Notes

Split from stale broad issue 272 and from issue 049's recorded remaining risk.

**Current implementation limitation**: The current Set implementation uses `value_to_string_into` to normalize all values to strings before storing them. This means that number `1` and string `"1"` are treated as the same key. Implementing SameValueZero identity requires a significant change to the collection infrastructure to store tagged values directly and implement proper value comparison semantics.

## Progress

- 2026-04-29: Added the first Set-only identity slice. Set `add`/`has`/`delete`
  now store and compare the current tagged value directly instead of normalizing
  through `value_to_string_into`, which distinguishes number `1` from string
  `"1"` for supported Set operations. The regression fixture
  `fixtures/builtins-and-io/set-identity-number-string.ts` matches Node/iwasm
  output for duplicate suppression, `has`, `delete`, and `size`. Full object
  identity remains out of scope for this issue.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- progress commit records the Set-only identity slice
- close commit records validation and issue lifecycle evidence

Validation result:

```text
command: cargo fmt --all --check
result: PASS
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli set
result: PASS; 6 tests run, 6 passed, 338 skipped
date: 2026-04-29

command: mise run update-issue-index -- --check
result: PASS before lifecycle move
date: 2026-04-29

command: mise run check issues
result: PASS before lifecycle move
date: 2026-04-29
```

Remaining risks:

- Full object identity remains out of scope for this Set primitive identity slice.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/277-implement-set-samevaluezero-identity.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
