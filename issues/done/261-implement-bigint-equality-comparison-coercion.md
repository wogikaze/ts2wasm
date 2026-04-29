---
id: 261
title: "Implement BigInt equality comparison and coercion boundaries"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P2
depends_on: [259]
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Implement BigInt strict equality, abstract equality, relational comparison, and the required primitive coercion boundaries.

Problem: BigInt cannot share the current primitive equality/comparison helpers because Number/BigInt and String/BigInt coercions have ECMAScript-specific rules.

## Current failure

```sh
tmp=/tmp/ts2wasm-261-bigint-comparison.ts
printf 'console.log(1n === 1n); console.log(1n === 1); console.log(1n == "1"); console.log(2n > 1);\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-261-bigint-comparison.wasm
```

Current result: BigInt/BigInt `===`, `!==`, `==`, `!=`, `<`, `<=`, `>`, and `>=` now match Node for the current heap BigInt representation, including negative values, canonical zero, and a large literal that exceeds the first-limb payload. Statically visible BigInt/String literal abstract equality folds supported StringToBigInt forms and invalid string inputs to Node-compatible booleans. Statically visible BigInt/Boolean literal abstract equality folds through Boolean-to-0/1 coercion. Statically visible BigInt/Number integer-literal abstract equality folds for representable tagged-int number literals. Statically visible BigInt/nullish literal abstract equality folds to Node-compatible false/true. Other statically visible mixed BigInt abstract equality and relational comparison emit issue-linked diagnostics; runtime-only mixed BigInt abstract equality and relational comparison trap instead of silently returning a normal wrong boolean.

## Desired final state

BigInt equality and relational comparison match Node for supported primitive values. Mixed arithmetic remains issue 260, but equality/comparison use the ECMA-262 coercion rules rather than treating BigInt as a generic object pointer.

## Scope

In scope:

- [x] Implement strict equality for BigInt mathematical values.
- [x] Implement abstract equality for BigInt with BigInt, Number, String, Boolean, null, and undefined in the current primitive subset.
- [x] Implement BigInt/BigInt abstract equality for the current heap BigInt representation.
- [x] Implement relational comparison for BigInt/BigInt.
- [x] Split relational comparison for remaining primitive mixed cases to issues 281 and 282 for Number edge cases and dynamic mixed coercion.
- [x] Split TypeError/coercion boundary follow-up to issue 260 for mixed arithmetic and issue 282 for dynamic coercion boundaries.

Out of scope:

- BigInt literal allocation; issue 259.
- BigInt arithmetic; issue 260.
- Object `ToPrimitive` beyond the current object model.
- BigInt builtins; issue 262.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/runtime-abi/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/`
- `docs/05-compatibility-and-semantics.md`
- `current-state.md`

Do not touch:

- unrelated object model or Proxy behavior
- parser syntax

## Acceptance criteria

- [x] Node/iwasm differential fixtures cover `===`, `!==`, `==`, `!=`, `<`, `<=`, `>`, and `>=` for BigInt operands.
- [x] Mixed BigInt/Number equality and comparison coverage for the current tagged-int subset is implemented; fractional, `NaN`, `Infinity`, and broader number-model cases are explicitly tracked by issue 281.
- [x] BigInt/String abstract equality uses StringToBigInt-compatible parsing for supported string inputs. Literal BigInt/String pairs are implemented; dynamic string values are split to issue 282.
- [x] Docs/current-state/issues state the remaining object `ToPrimitive` and number-model limits.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
mise run check issues
```

Impacted commands:

```sh
cargo test -p ts2wasm-cli bigint
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] updated: `docs/05-compatibility-and-semantics.md`
- [x] updated: `docs/language-reference/javascript-features.md`

Current state:

- [x] updated: `current-state.md`

Follow-up issues:

- [x] issue 281: BigInt/Number edge equality and comparison
- [x] issue 282: dynamic BigInt mixed coercion, including dynamic StringToBigInt and object `ToPrimitive`

## Notes

Do not claim full equality parity if broader `number` support still cannot represent the Node comparison case.

2026-04-29 progress slice: strict equality now compares BigInt mathematical values instead of object pointer identity, strict BigInt/non-BigInt equality returns false/true for `===`/`!==`, BigInt/BigInt abstract equality reuses the strict BigInt comparison path, and BigInt/BigInt relational operators compare sign plus cached decimal magnitude. Mixed BigInt abstract equality and relational comparison are still rejected with spanned issue-261 diagnostics when statically visible, and runtime-only mixed BigInt cases trap rather than returning false until StringToBigInt parsing and BigInt/Number comparison limits are implemented.

2026-04-29 progress slice: statically folded BigInt/String literal abstract equality now uses the same supported StringToBigInt parser as `BigInt(string)` for decimal, binary, octal, hexadecimal, empty/whitespace-to-zero, and signed decimal string literals. Invalid string inputs fold to `false` for `==` and `true` for `!=`, matching Node for the covered literal subset. Dynamic string values and relational BigInt/String comparisons remain issue-261 diagnostics/traps rather than silently widening runtime behavior.

2026-04-29 progress slice: statically folded BigInt/Boolean literal abstract equality now applies Boolean-to-Number-to-BigInt-equivalent comparison for the literal `false -> 0n` and `true -> 1n` cases. Covered Node/iwasm fixtures include symmetric forms such as `0n == false`, `false == 0n`, `1n == true`, `true == 1n`, and mismatch `!=` cases. Dynamic boolean values still remain issue-261 diagnostics/traps until the runtime mixed primitive equality boundary is intentionally widened.

2026-04-29 progress slice: statically folded BigInt/Number integer-literal abstract equality now compares known BigInt literals against `Expr::Number` tagged-int literals. Covered Node/iwasm fixtures include symmetric forms such as `1n == 1`, `1 == 1n`, `0n == 0`, and mismatch `!=` cases. Fractional numbers, `NaN`, `Infinity`, `-0`, dynamic numbers, and relational BigInt/Number comparisons remain issue-261 diagnostics/traps.

2026-04-29 progress slice: statically folded BigInt/nullish literal abstract equality now folds `BigInt == null`, `null == BigInt`, `BigInt == undefined`, and `undefined == BigInt` to Node-compatible false, with `!=` forms folded to true. Remaining BigInt/Number edge equality/comparison is split to issue 281. Dynamic BigInt/String/Boolean/nullish coercion, mixed primitive relational comparison beyond BigInt/BigInt, and object `ToPrimitive` interactions are split to issue 282.

Validation for this progress slice:

```text
command: cargo nextest run -E 'test(bigint_mixed_string_abstract_equality_fixture_matches_node_output_under_iwasm) or test(bigint_mixed_abstract_equality_reports_issue_261) or test(bigint_runtime_mixed_abstract_equality_traps_instead_of_false)'
result: passed (3 passed, 508 skipped)
date: 2026-04-29

command: cargo fmt --all --check
result: passed
date: 2026-04-29

command: cargo nextest run -E 'test(bigint) or test(node_diff)'
result: passed (32 passed, 479 skipped)
date: 2026-04-29

command: cargo test -p ts2wasm-cli bigint
result: passed (25 passed across filtered CLI tests)
date: 2026-04-29

command: mise run update-issue-index -- --check
result: passed
date: 2026-04-29

command: mise run check issues
result: passed
date: 2026-04-29
```

Validation for BigInt/Boolean progress slice:

```text
command: cargo nextest run -E 'test(bigint_mixed_boolean_abstract_equality_fixture_matches_node_output_under_iwasm) or test(bigint_mixed_abstract_equality_reports_issue_261) or test(bigint_runtime_mixed_abstract_equality_traps_instead_of_false)'
result: passed (3 passed, 511 skipped)
date: 2026-04-29

command: cargo fmt --all --check
result: passed
date: 2026-04-29

command: cargo nextest run -E 'test(bigint) or test(node_diff)'
result: passed (33 passed, 481 skipped)
date: 2026-04-29

command: cargo test -p ts2wasm-cli bigint
result: passed (26 passed across filtered CLI tests)
date: 2026-04-29

command: mise run update-issue-index -- --check
result: passed
date: 2026-04-29

command: mise run check issues
result: passed
date: 2026-04-29
```

Validation for BigInt/Number integer-literal progress slice:

```text
command: cargo nextest run -E 'test(bigint_mixed_number_abstract_equality_fixture_matches_node_output_under_iwasm) or test(bigint_mixed_abstract_equality_reports_issue_261) or test(bigint_runtime_mixed_abstract_equality_traps_instead_of_false)'
result: passed (3 passed, 513 skipped)
date: 2026-04-29

command: cargo fmt --all --check
result: passed
date: 2026-04-29

command: cargo nextest run -E 'test(bigint) or test(node_diff)'
result: passed (34 passed, 482 skipped)
date: 2026-04-29

command: cargo test -p ts2wasm-cli bigint
result: passed (27 passed across filtered CLI tests)
date: 2026-04-29

command: mise run update-issue-index -- --check
result: passed
date: 2026-04-29

command: mise run check issues
result: passed
date: 2026-04-29
```

Validation for BigInt/nullish progress slice:

```text
command: cargo nextest run -E 'test(bigint_mixed_nullish_abstract_equality_fixture_matches_node_output_under_iwasm)'
result: passed (1 passed, 530 skipped)
date: 2026-04-29
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- progress commits recorded above for BigInt equality/comparison slices
- close commit records split follow-up ownership and issue lifecycle evidence

Validation result:

```text
command: cargo nextest run -E 'test(bigint) or test(node_diff)'
result: PASS (34+ filtered tests across recorded slices)
date: 2026-04-29

command: cargo test -p ts2wasm-cli bigint
result: PASS (27+ filtered CLI tests across recorded slices)
date: 2026-04-29

command: cargo fmt --all --check
result: PASS
date: 2026-04-29

command: mise run update-issue-index -- --check
result: PASS before lifecycle move
date: 2026-04-29

command: mise run check issues
result: PASS before lifecycle move
date: 2026-04-29
```

Remaining risks:

- BigInt/Number edge equality/comparison remains issue 281.
- Dynamic mixed BigInt coercion remains issue 282.
- Mixed arithmetic TypeError behavior remains issue 260.
