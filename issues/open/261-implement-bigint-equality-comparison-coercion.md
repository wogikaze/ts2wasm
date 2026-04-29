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

Current result: BigInt/BigInt `===`, `!==`, `==`, `!=`, `<`, `<=`, `>`, and `>=` now match Node for the current heap BigInt representation, including negative values, canonical zero, and a large literal that exceeds the first-limb payload. Statically visible mixed BigInt abstract equality and relational comparison emit issue-261 diagnostics; runtime-only mixed BigInt abstract equality and relational comparison trap instead of silently returning a normal wrong boolean.

## Desired final state

BigInt equality and relational comparison match Node for supported primitive values. Mixed arithmetic remains issue 260, but equality/comparison use the ECMA-262 coercion rules rather than treating BigInt as a generic object pointer.

## Scope

In scope:

- [x] Implement strict equality for BigInt mathematical values.
- [ ] Implement abstract equality for BigInt with BigInt, Number, String, Boolean, null, and undefined in the current primitive subset.
- [x] Implement BigInt/BigInt abstract equality for the current heap BigInt representation.
- [x] Implement relational comparison for BigInt/BigInt.
- [ ] Implement relational comparison for supported primitive mixed cases.
- [ ] Preserve TypeError paths for invalid coercions such as `ToNumber(1n)` where applicable.

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
- [ ] Mixed BigInt/Number equality and comparison are tested for integral, fractional, `NaN`, `Infinity`, and `-0` cases where the current number model can represent them; unrepresentable number cases remain explicitly tracked.
- [ ] BigInt/String abstract equality uses StringToBigInt-compatible parsing for supported string inputs.
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

- [ ] create follow-up for unsupported object `ToPrimitive` interactions if needed

## Notes

Do not claim full equality parity if broader `number` support still cannot represent the Node comparison case.

2026-04-29 progress slice: strict equality now compares BigInt mathematical values instead of object pointer identity, strict BigInt/non-BigInt equality returns false/true for `===`/`!==`, BigInt/BigInt abstract equality reuses the strict BigInt comparison path, and BigInt/BigInt relational operators compare sign plus cached decimal magnitude. Mixed BigInt abstract equality and relational comparison are still rejected with spanned issue-261 diagnostics when statically visible, and runtime-only mixed BigInt cases trap rather than returning false until StringToBigInt parsing and BigInt/Number comparison limits are implemented.

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
