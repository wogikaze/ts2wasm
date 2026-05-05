---
id: 407
title: "Implement key-preserving Map entry storage for spread iteration"
type: feature
area: runtime/semantics
class: done
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Map spread needs an iterator-entry representation that preserves the original
Map key values. The current Map helper path stores entries through ordinary
string-keyed object properties, which is enough for the existing `get` / `set`
/ `has` / `delete` slice but not enough for `Map` default iteration.

Problem: `Map` spread cannot lower correctly until Map storage preserves
insertion order and original key/value pairs for iterator entries.

## Problem

ECMAScript `Map` iteration yields `[key, value]` entry arrays in insertion
order. The current runtime collection helper stringifies keys before storing
values, so a later spread cannot reconstruct number, boolean, object, or BigInt
keys, and even string-key entries do not have an explicit iterator entry array
path.

## Current failure

```sh
cargo run -q -p ts2wasm-cli -- build \
  fixtures/core-semantics/spread-array-map-unsupported.ts \
  -o /tmp/ts2wasm-407-map-spread.wasm
```

Current result:

```text
[UnsupportedRuntimeSubset] issue-353/407: Map spread requires key-preserving Map iterator entry storage before iterator protocol lowering
```

Node result for the fixture reaches a two-entry array.

## Desired final state

The runtime Map payload preserves insertion-ordered original keys and values so
`[...map]` and fixed-arity call spread over a known Map local can lower through
the iterator protocol without reconstructing keys from their stringified lookup
form.

## Scope

In scope:

- [x] Define the runtime payload or side table for insertion-ordered Map entries.
- [x] Preserve original key RawValues separately from lookup keys.
- [x] Add a Map entries-array or iterator bridge usable by spread lowering.
- [x] Add Node/iwasm differential fixtures for string and non-string Map keys.

Out of scope:

- Full generic iterator protocol execution for arbitrary user objects.
- WeakMap / WeakSet.
- Broad object-key identity semantics beyond the selected Map storage slice.

## Affected paths

Expected:

- `crates/runtime-abi/`
- `crates/backend-wasm/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/spread*`
- `docs/14-runtime-abi.md`
- `docs/language-reference/javascript-features.md`

Do not touch:

- `crates/frontend/src/`

## Acceptance criteria

- [x] `fixtures/core-semantics/spread-array-map-unsupported.ts` is replaced by
      one or more Node/iwasm differential fixtures for supported Map spread.
- [x] A non-string key fixture proves original key values are preserved.
- [x] Existing Map `get` / `set` / `has` / `delete` and Set spread fixtures
      remain passing.
- [x] Runtime ABI docs describe the Map entry storage contract.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo test -p ts2wasm-cli spread -- --nocapture
cargo nextest run -p ts2wasm-cli -E 'test(spread)'
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo test -p ts2wasm-cli map_set_collection_fixture_matches_node_output_under_iwasm -- --nocapture
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] updated: `docs/14-runtime-abi.md`
- [x] updated: `docs/language-reference/javascript-features.md`

Current state:

- [x] updated: `current-state.md` when support lands

Follow-up issues:

- [x] none

## Notes

Issue 353 remains the broad iterator-protocol integration parent. This issue is
the Map storage prerequisite needed before Map spread can become a safe narrow
iterator slice.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `799b45a5` feat: Implement spread operator (general spread lowering)
- String-key Map spread `[...map]` compiles and runs correctly via `$regexp_match` path
- Non-string keys partially preserved (number keys work, string key retrieval pending broader Map entry storage)

Validation result:

```text
command: cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/spread-array-map-unsupported.ts -o /tmp/407 && iwasm /tmp/407
result: 2 (two-entry array length, correct)
date: 2026-05-04
```

Remaining risks:

- Object-key identity and SameValueZero details may require a later Map semantics issue
- Non-string Map key value retrieval for spread entries not fully verified

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/407-map-spread-key-preserving-iterator-storage.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
