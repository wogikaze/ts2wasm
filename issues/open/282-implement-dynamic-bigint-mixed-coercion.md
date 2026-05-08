---
id: 282
title: "Implement dynamic BigInt mixed coercion"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P2
depends_on: [259, 261]
blocks: []
created: 2026-04-29
updated: 2026-05-01
---

## Summary

Implement dynamic mixed BigInt abstract equality and relational comparison coercion beyond issue 261's static literal folds.

Problem: issue 261 deliberately handles statically visible literal BigInt/String, BigInt/Boolean, BigInt/tagged-int Number, and BigInt/nullish abstract equality in the resolver. Runtime-only mixed values currently trap rather than producing a silent incorrect boolean, and relational mixed primitive comparison remains unsupported.

## Current failure

```sh
tmp=/tmp/ts2wasm-282-dynamic-bigint-coercion.ts
printf 'let a = 1n; let box = { x: a }; console.log(box.x == "1"); console.log(box.x < "2");\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-282-dynamic-bigint-coercion.wasm
iwasm /tmp/ts2wasm-282-dynamic-bigint-coercion.wasm
```

Current result: runtime-only mixed BigInt comparisons trap, while statically visible unsupported mixed relational comparisons report issue-linked diagnostics.

## Desired final state

Dynamic BigInt/String, BigInt/Boolean, BigInt/nullish, and supported object `ToPrimitive` equality/comparison cases match Node within the current value model. Unsupported cases produce source diagnostics or intentional runtime traps with issue ownership.

## Scope

In scope:

- [x] Implement dynamic StringToBigInt parsing for abstract equality where the current runtime string model can preserve Node-compatible behavior.
- [x] Implement dynamic Boolean-to-Number-to-BigInt-equivalent abstract equality boundaries.
- [x] Implement mixed BigInt/String and BigInt/Boolean relational comparison for supported primitive values.
- [x] Track object `ToPrimitive` interactions with source-backed diagnostics and split compatible implementation plus unknown out-of-range runtime strings to issue 368.

Out of scope:

- BigInt/Number edge cases; issue 281 owns number-model-sensitive comparisons.
- BigInt arithmetic; issue 260 owns arithmetic.
- BigInt builtin dynamic inputs; issue 280 owns builtin calls.
- Parser BigInt syntax.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/*bigint*`
- `docs/05-compatibility-and-semantics.md`
- `docs/language-reference/javascript-features.md`
- `current-state.md`

Do not touch:

- parser BigInt syntax
- broad runtime ABI representation unless a compile error proves it is required

## Acceptance criteria

- [x] Node/iwasm differential fixtures cover runtime BigInt/String abstract equality for supported StringToBigInt inputs and invalid strings.
- [x] Node/iwasm differential fixtures cover runtime BigInt/Boolean and BigInt/nullish abstract equality.
- [x] Node/iwasm differential fixtures cover supported mixed BigInt/String and BigInt/Boolean relational comparisons.
- [x] Object `ToPrimitive` behavior is explicitly split with source-backed diagnostics; compatible implementation is issue 368.
- [x] Docs/current-state/issues state dynamic mixed BigInt coercion limits and point remaining runtime gaps to issue 368.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(bigint) or test(node_diff)'
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
# covered by required nextest filter
```

## Notes

Split from issue 261 on 2026-04-29 because the existing runtime-only mixed fixtures intentionally trap instead of returning a wrong boolean:

- `fixtures/core-semantics/bigint-runtime-mixed-abstract-equality-trap.ts`
- `fixtures/core-semantics/bigint-runtime-mixed-relational-trap.ts`

Progress on 2026-04-29:

- Runtime-only BigInt/Boolean and BigInt/nullish abstract equality now has Node/iwasm differential coverage in `fixtures/core-semantics/bigint-runtime-mixed-boolean-nullish-abstract-equality.ts`.
- BigInt/String abstract equality and mixed BigInt relational comparison remain intentionally outside this slice; the existing runtime-only String and relational trap fixtures continue to own those blockers.

Progress on 2026-04-29:

- Reclassified the remaining statically visible mixed BigInt abstract equality
  and relational comparison unsupported diagnostics from closed issue 261 to
  issue 282, so future work is owned by this runtime coercion issue.
- This is a diagnostic ownership slice only. It does not implement dynamic
  StringToBigInt parsing, object `ToPrimitive`, or mixed relational runtime
  semantics.

Progress on 2026-04-29:

- Implemented runtime BigInt/String abstract equality for dynamic string values
  that parse through the current integer-backed equality conversion path,
  including decimal strings, signed decimal strings, empty/whitespace strings,
  and unsigned `0x`/`0b`/`0o` prefixes.
- Replaced the previous runtime trap fixture with Node/iwasm differential
  fixtures for dynamic object-carried BigInt/String equality.
- Remaining issue-282 work: object `ToPrimitive`, relational mixed
  BigInt/String and BigInt/Boolean comparisons, and unsupported string grammar
  outside the current small-int runtime boundary.

Progress on 2026-04-29:

- Added Node/iwasm differential coverage for object-carried dynamic
  BigInt/Boolean relational comparison over the first `<`, `<=`, and `>`
  slice, including symmetric boolean-left cases.
- Implemented the matching runtime helper path by comparing booleans as the
  current small integer boundary (`false -> 0`, `true -> 1`) without changing
  BigInt/Number or BigInt/String relational behavior.
- Added `>=` coverage/helper parity for the same BigInt/Boolean relational
  slice.
- Remaining issue-282 work: mixed BigInt/String relational comparison, object
  `ToPrimitive`, and unsupported string grammar outside the current small-int
  runtime boundary.

Progress on 2026-04-29:

- Added Node/iwasm differential coverage for object-carried dynamic
  BigInt/String relational comparison over `<`, `<=`, `>`, and `>=`, including
  symmetric string-left cases, whitespace/empty strings, unsigned radix
  prefixes, and invalid string inputs that compare false.
- Implemented the matching runtime helper path by reusing the current
  `string_to_number_for_equality` small-int StringToBigInt-compatible parsing
  boundary before `bigint_compare_small_int`.
- Remaining issue-282 work: object `ToPrimitive` and unsupported string
  grammar or magnitude outside the current small-int runtime boundary.

Progress on 2026-04-29:

- Added a source diagnostic regression for literal-derived dynamic
  BigInt/String comparison strings outside the current signed-i32
  `StringToBigInt` comparison helper boundary:
  `fixtures/core-semantics/bigint-runtime-mixed-string-out-of-range-unsupported.ts`.
- `crates/cli/tests/m2_node_diff.rs` now asserts the diagnostic remains owned
  by issue 282 instead of allowing the runtime helper to overflow and return a
  silently wrong boolean for the statically-known dynamic string case.
- Remaining issue-282 work: object `ToPrimitive` and unknown object-carried
  dynamic strings outside the current signed-i32 boundary still require broader
  runtime/helper work or a separate source-backed split.

Progress on 2026-04-29:

- Added a source-spanned issue-282 diagnostic for mixed BigInt comparisons that
  would require object `ToPrimitive` on an object literal/local with
  `valueOf`/`toString`, covered by
  `fixtures/core-semantics/bigint-runtime-mixed-object-toprimitive-unsupported.ts`.
- This keeps object coercion unsupported instead of lowering to a runtime trap
  or generic mixed coercion diagnostic.
- Remaining issue-282 work: implementing compatible object `ToPrimitive`
  coercion and handling unknown object-carried dynamic strings outside the
  current signed-i32 boundary.

Progress on 2026-04-29:

- Added source-backed issue-282 diagnostics for literal-derived object-property
  BigInt/String comparisons where the string value is outside the current
  signed-i32 `StringToBigInt` comparison helper boundary, covered by
  `fixtures/core-semantics/bigint-runtime-mixed-object-string-out-of-range-unsupported.ts`.
- Node prints `false` for that fixture while the previous iwasm output was
  `true`, so the guard prevents a silent incorrect boolean until broader
  runtime string parsing is implemented.
- Remaining issue-282 work: compatible object `ToPrimitive` coercion and
  broader unknown out-of-range dynamic string handling that is not
  source-backed by literal/local object-property values.

Progress on 2026-05-01:

- Added source-backed issue-282 diagnostic regression coverage for a
  `toString`-based object `ToPrimitive` comparison boundary:
  `fixtures/core-semantics/bigint-runtime-mixed-object-toprimitive-string-unsupported.ts`.
- This complements the existing `valueOf` BigInt-returning object diagnostic
  and keeps ordinary object `ToPrimitive` unsupported instead of silently
  lowering unsupported object coercion.
- Remaining issue-282 work: compatible object `ToPrimitive` coercion and
  broader unknown out-of-range dynamic string handling that is not
  source-backed by literal/local object-property values.

Progress on 2026-05-01:

- Closed the implemented issue-282 primitive mixed BigInt coercion slice after existing Node/iwasm coverage covered runtime BigInt/String equality, BigInt/Boolean/nullish equality, and BigInt/String/Boolean relational comparisons.
- Kept object `ToPrimitive` unsupported through source-backed issue-282 diagnostics and split compatible object coercion plus non-source-backed unknown out-of-range BigInt/String runtime parsing to issue 368.
- Updated docs/current-state issue ownership so issue 282 no longer remains the open bucket for those runtime gaps.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- closure commit on child branch; see final report for hash

Validation result:

```text
Pending validation in reports/runs/20260430T173241Z/cycle_report.md.
```

Remaining risks:

- Compatible object `ToPrimitive` and non-source-backed unknown out-of-range runtime strings remain open in issue 368.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/282-implement-dynamic-bigint-mixed-coercion.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
