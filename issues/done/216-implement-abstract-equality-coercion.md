---
id: 216
title: "Implement abstract equality coercion"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-28
updated: 2026-04-28
completed: 2026-04-28
---

## Summary

Implement ECMAScript coercion rules for abstract equality operators `==` and `!=`.

## Problem

Issue 058 added equality operator support but records that abstract equality currently delegates to strict equality. That is a semantic placeholder for mixed-type comparisons.

## Desired final state

`==` and `!=` match ECMAScript abstract equality for the supported value types, with unsupported object/primitive coercions diagnosed or tracked.

## Scope

In scope:

- [x] Implement primitive coercion cases for `undefined`, `null`, boolean, number, and string.
- [x] Preserve strict equality semantics for `===` and `!==`.
- [x] Add Node differential fixtures for mixed-type equality and inequality.
- [x] Track object/ToPrimitive gaps explicitly if not completed in this slice.

Out of scope:

- Full object `ToPrimitive` behavior if the object model is not ready.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/`
- `docs/language-reference/javascript-features.md`
- `docs/05-compatibility-and-semantics.md`
- `current-state.md`

Do not touch:

- none

## Acceptance criteria

- [x] `==` and `!=` no longer behave as strict equality for supported mixed primitive cases.
- [x] `===` and `!==` behavior is unchanged.
- [x] Node differential fixtures cover representative coercion rules.
- [x] Docs/current-state/issues are synchronized after behavior changes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo nextest run -E 'test(equal|equality)'
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] update `docs/language-reference/javascript-features.md`
- [x] update `docs/05-compatibility-and-semantics.md`

Current state:

- [x] update `current-state.md`

Follow-up issues:

- [x] none

## Notes

Created from issue 203 audit of `issues/done/058-implement-equality-operators.md`.

## Completion evidence

Commits:

- `c50ff75` issue-216: implement primitive abstract equality

Validation result:

```text
command: cargo test -p ts2wasm-cli --test m2_node_diff abstract_equality_fixture_matches_node_output_under_iwasm -- --nocapture
result: passed
date: 2026-04-28

command: cargo nextest run -E 'test(equal|equality)'
result: no tests matched; rerun with regex-style expression below
date: 2026-04-28

command: cargo nextest run -E 'test(~equal) | test(~equality)'
result: 1 passed
date: 2026-04-28

command: cargo test -p ts2wasm-cli --test m2_node_diff m3_semantic_fixtures_match_node_output_under_iwasm -- --nocapture
result: passed
date: 2026-04-28

command: cargo fmt --all --check
result: passed
date: 2026-04-28

command: cargo nextest run
result: 195 passed, 4 skipped
date: 2026-04-28
```

Remaining risks:

- Object `ToPrimitive`, floating point, `NaN`, and `-0` are outside this primitive tagged-int slice and remain tied to object/number-model follow-up work.
