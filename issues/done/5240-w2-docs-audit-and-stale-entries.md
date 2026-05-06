---
id: 5240
title: "W2: docs audit — fix stale language-reference entries and confirm semantic-core coverage"
type: docs
area: docs
class: design-ready
priority: P1
depends_on: []
blocks: [5242]
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Several entries in `docs/language-reference/javascript-features.md` are marked "未実装"
but are already implemented and passing Node/iwasm differential tests. This issue
corrects those stale entries, adds any missing fixture coverage for W2 operators,
and aligns the documentation with actual implementation state.

## Problem

Three entries are stale and block an accurate W2 gate assessment:

1. `void` operator (line 164): marked "未実装" but fully implemented (parser → resolver → backend → differential test).
2. `for...of` statement (line 175): marked "未実装" but fully implemented and passes Node differential.
3. `for...in` statement (line 174): marked "未実装" but partially implemented (runtime trap — should be "部分実装").

Additionally, bitwise NOT (`~`) and shift operators (`<<`, `>>`, `>>>`) emit
unsupported diagnostics in the resolver. These are NOT W2-scope operators, but
their presence in the diagnostic table affects the reference-coverage breakdown.
They should remain explicitly listed as unsupported with their issue numbers.

Problem: Stale "未実装" entries prevent accurate W2 gate assessment.

## Desired final state

- `docs/language-reference/javascript-features.md` entries correctly reflect implementation status
- W2-scope operators all have Node differential fixture coverage
- No W2-scope item is marked "未実装" when it already passes tests

## Scope

In scope:

- [ ] Mark `void` operator as "実装済み" in javascript-features.md
- [ ] Mark `for...of` as "実装済み" in javascript-features.md
- [ ] Mark `for...in` as "部分実装 (runtime trap)" in javascript-features.md
- [ ] Verify `void` fixture is included in m2_node_diff assertions
- [ ] Verify `for...of` fixture is included in m2_node_diff assertions
- [ ] Update `docs/05-compatibility-and-semantics.md` if W2 semantic coverage is incomplete
- [ ] `current-state.md` update

Out of scope:

- Number model expansion (NaN, Infinity, -0, Object.is) — separate issue
- Class / object / module / async gaps (W3/W4)
- Any runtime implementation changes

## Affected paths

Expected:

- `docs/language-reference/javascript-features.md`
- `docs/05-compatibility-and-semantics.md`
- `current-state.md`

Do not touch:

- `crates/` (no runtime changes)
- `fixtures/` (use existing fixtures if adequate)

## Acceptance criteria

- [ ] `void` operator row is "実装済み" in javascript-features.md
- [ ] `for...of` row is "実装済み" in javascript-features.md
- [ ] `for...in` row is "部分実装 (runtime trap)" in javascript-features.md
- [ ] All W2-scope operators have at least one fixture in the Node differential suite
- [ ] `cargo fmt --all --check` passes
- [ ] `cargo nextest run` passes

## Validation

```sh
cargo fmt --all --check
cargo nextest run
# Verify void fixture exists and passes
grep "unary-void-operator" crates/cli/tests/common/m2_node_diff_fixture_tests.rs
# Verify for-of fixture exists and passes
grep "for-of" crates/cli/tests/common/m2_node_diff_fixture_tests.rs
```

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected
- [ ] updated: `docs/language-reference/javascript-features.md`

Current state:

- [ ] not affected
- [ ] updated: `current-state.md`

Follow-up issues:

- [ ] none
- [ ] created/updated: W2 semantic core completeness (5242)
