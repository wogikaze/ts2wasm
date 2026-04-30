---
id: 281
title: "Implement BigInt/Number edge equality and comparison"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P2
depends_on: [259, 261]
blocks: []
created: 2026-04-29
updated: 2026-04-29
completed: 2026-05-01
---

## Summary

Implement the remaining BigInt/Number abstract equality and relational comparison cases that issue 261 intentionally left outside the tagged-int literal slice.

Problem: issue 261 folds representable tagged-int number literals such as `1n == 1`, but the current number model does not yet represent or compare fractional, `NaN`, `Infinity`, and `-0` cases with Node-compatible BigInt coercion semantics.

## Current failure

```sh
tmp=/tmp/ts2wasm-281-bigint-number-edges.ts
printf 'console.log(1n == 1.0); console.log(1n == 1.5); console.log(1n < 2); console.log(1n == NaN); console.log(1n == Infinity); console.log(0n == -0);\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-281-bigint-number-edges.wasm
```

Current result: issue-linked unsupported diagnostics for unsupported mixed BigInt comparison forms, or parser/number-model limits before BigInt comparison can be evaluated.

## Desired final state

BigInt/Number abstract equality and relational comparison match Node for the supported number model. Unsupported number values remain source diagnostics with explicit issue ownership instead of silently returning an incorrect boolean.

## Scope

In scope:

- [x] Implement BigInt/Number abstract equality for integral, fractional, `NaN`, `Infinity`, and `-0` cases where the current number model can represent them.
- [x] Implement BigInt/Number relational comparison for supported number values.
- [x] Preserve source diagnostics for number forms that remain outside the current number model.
- [x] Keep mixed BigInt arithmetic out of scope; issue 260 owns arithmetic TypeError behavior.

Out of scope:

- Dynamic string parsing and object `ToPrimitive`; issue 282 owns those runtime coercion cases.
- Full floating-point runtime representation if it requires broad number-model redesign.
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
- unrelated runtime ABI representation without a compile-proven need

## Acceptance criteria

- [x] Node/iwasm differential fixtures cover BigInt/Number equality for integral, fractional, `NaN`, `Infinity`, and `-0` cases that the current number model can represent.
- [x] Node/iwasm differential fixtures cover BigInt/Number `<`, `<=`, `>`, and `>=` for supported number values.
- [x] Unrepresentable number cases are explicitly tracked with source-backed diagnostics and issue references.
- [x] Docs/current-state/issues state the supported BigInt/Number comparison subset and remaining number-model limits.

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
cargo test -p ts2wasm-cli bigint
```

## Notes

Split from issue 261 on 2026-04-29 because issue 261 already implemented BigInt/BigInt equality/comparison plus literal BigInt/String, Boolean, tagged-int Number, and nullish abstract equality. Broader Number edge cases need number-model evidence before they can be claimed.

## Progress evidence

2026-04-29:

- Added static folding for representable integer Number / BigInt relational comparisons (`<`, `<=`, `>`, `>=`) in the same compile-time subset as the existing abstract equality slice.
- Extended literal mixed Number/BigInt abstract equality coverage to unary-negative integer literals including the current `-0` token representation.
- Added Node/iwasm differential fixture coverage in `fixtures/core-semantics/bigint-mixed-number-relational.ts`.
- Validation passed: `cargo fmt --all --check`; `cargo nextest run -E 'test(bigint) or test(node_diff)'` (37 passed, 495 skipped).
- Remaining scope: fractional, `NaN`, `Infinity`, and broader number-model-sensitive cases are not closed by this slice.

2026-04-29:

- Added source-spanned issue-281 diagnostics for statically visible
  BigInt/Number comparisons against `NaN` and `Infinity`, preventing those
  cases from falling through to generic unresolved-name diagnostics while the
  broader number model remains out of scope.
- Added unsupported regression fixtures for `1n == NaN` and `1n < Infinity`.
- Remaining scope: implementing compatible `NaN` / `Infinity` / fractional
  number comparison semantics requires the broader number model and is not
  closed by this slice.

2026-04-29:

- Extended the same source-spanned issue-281 diagnostic boundary to signed unary
  special number globals such as `-Infinity`, preventing those BigInt/Number
  comparisons from falling through to generic unresolved-name diagnostics.
- Added unsupported regression coverage in
  `fixtures/core-semantics/bigint-mixed-number-unary-special-unsupported.ts`.
- Remaining scope: fractional number tokens still fail before IR name
  resolution under the current parser/number-model boundary, and compatible
  `NaN` / `Infinity` runtime comparison semantics remain unimplemented.

2026-04-29:

- Added a parser-side source-spanned issue-281 diagnostic boundary for
  statically visible fractional number token sequences in BigInt/Number
  equality and relational comparisons, covering both `1n == 1.5` and
  `1.5 < 2n` before they fall through to member-property parse diagnostics.
- Added unsupported regression fixtures for both BigInt-left and
  fractional-left forms.
- Validation passed: `cargo test -p ts2wasm-cli bigint`;
  `cargo fmt --all --check`; `cargo nextest run -E 'test(bigint) or
  test(node_diff)'` (45 passed, 551 skipped); `mise run
  update-issue-index -- --check`.
- Validation not green: `mise run check issues` fails before issue-281-specific
  checks on pre-existing missing test262 coverage-result artifact references
  in issue 308 and several done issues.
- Remaining scope: compatible fractional / `NaN` / `Infinity` runtime
  comparison semantics still require the broader number model and are not
  closed by this slice.

2026-05-01:

- Verified the current supported number model: ordinary `number` remains
  integer-only, with `-0` represented by the existing unary-negative integer
  token path, while fractional values, `NaN`, `Infinity`, and `Number.*`
  numeric constants remain outside the supported runtime number model.
- Added issue-281 diagnostic ownership for statically visible unshadowed
  `Number.*` numeric constants in BigInt/Number equality and relational
  comparisons, preventing `Number.NaN`, `Number.POSITIVE_INFINITY`,
  `Number.NEGATIVE_INFINITY`, and related numeric constants from falling into
  issue-282 mixed runtime coercion diagnostics.
- Added unsupported regression coverage in
  `fixtures/core-semantics/bigint-mixed-number-static-number-member-unsupported.ts`.
- Closure decision: the current supported BigInt/Number comparison slice is
  complete. Broader fractional / `NaN` / `Infinity` / floating-point runtime
  comparison semantics are out of the current number model and remain
  represented as source-backed issue-281 diagnostics rather than broad runtime
  floating-point implementation work.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- close commit records issue-281 verification, `Number.*` diagnostic ownership,
  and issue lifecycle evidence

Validation result:

```text
command: cargo fmt --all --check
result: PASS
date: 2026-05-01

command: cargo test -p ts2wasm-cli --test m2_node_diff bigint
result: PASS (37 passed, 0 failed)
date: 2026-05-01

command: mise run update-issue-index -- --check
result: PASS after lifecycle move; issues/index.md up to date
date: 2026-05-01

command: mise run check issues
result: FAIL due to pre-existing missing artifacts/coverage/results/test262-results.jsonl references in issues 308/312 and done issues; no issue-281-specific health failure was reported
date: 2026-05-01
```

Remaining risks:

- Compatible fractional, `NaN`, `Infinity`, and broader floating-point runtime
  comparison semantics require broader number-model work and remain explicitly
  unsupported in this slice.
