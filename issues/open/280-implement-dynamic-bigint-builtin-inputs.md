---
id: 280
title: "Implement dynamic BigInt builtin inputs"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P2
depends_on: [259, 262]
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Extend the BigInt builtin subset beyond issue 262's literal-safe folding path.

Problem: `BigInt(...)`, `BigInt.asIntN(...)`, and `BigInt.asUintN(...)` currently work for documented static inputs, but dynamic runtime inputs still report issue-linked unsupported diagnostics.

## Current failure

```sh
tmp=/tmp/ts2wasm-280-dynamic-bigint-builtins.ts
printf 'const bits = 8; const value = 255n; console.log(BigInt.asIntN(bits, value));\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-280-dynamic-bigint-builtins.wasm
```

Current result: issue-linked unsupported diagnostic for dynamic BigInt builtin inputs.
The diagnostic owner is this issue; closed issue 262 owns only the literal-safe
BigInt builtin slice and the stable `new BigInt(...)` constructor rejection.

## Desired final state

Dynamic BigInt builtin inputs that fit the current runtime BigInt representation lower through explicit runtime helpers with Node/iwasm differential coverage. Inputs outside the supported runtime representation produce source diagnostics linked to this issue.

## Scope

In scope:

- [ ] Implement dynamic `BigInt.asIntN(bits, value)` and `BigInt.asUintN(bits, value)` for supported runtime BigInt values and supported bit widths.
- [ ] Broaden `BigInt(value)` runtime conversion for dynamic values where the current value model can preserve Node-compatible behavior.
- [ ] Keep diagnostics for out-of-slice values source-linked and issue-280-linked.

Out of scope:

- Full multi-limb BigInt arithmetic beyond the current representation.
- Broader Number model gaps such as `NaN`, `Infinity`, `-0`, and fractional values.
- Object `ToPrimitive` for arbitrary objects.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/*bigint*`
- `docs/language-reference/javascript-features.md`
- `current-state.md`

Do not touch:

- unrelated builtin families
- parser BigInt literal syntax

## Acceptance criteria

- [ ] Node/iwasm differential fixtures cover dynamic `BigInt.asIntN` and `BigInt.asUintN` inputs in the supported runtime range.
- [ ] Dynamic `BigInt(...)` conversions either match Node for supported runtime values or produce source diagnostics linked to issue 280.
- [ ] Runtime linker structure tests cover any new BigInt builtin helpers and avoid new host imports.
- [ ] Docs/current-state/issues state the supported dynamic builtin subset and remaining limits.

## Current diagnostic coverage

The following residual unsupported fixtures are intentionally linked to this
issue until dynamic runtime handling is implemented:

- `fixtures/core-semantics/bigint-builtin-as-int-n-unsupported.ts`
- `fixtures/core-semantics/bigint-builtin-as-uint-n-unsupported.ts`
- `fixtures/core-semantics/bigint-builtin-invalid-decimal-string-unsupported.ts`
- `fixtures/core-semantics/bigint-builtin-invalid-string-unsupported.ts`

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

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] updated: `docs/language-reference/javascript-features.md`

Current state:

- [ ] updated: `current-state.md`

Follow-up issues:

- [ ] none

## Notes

Issue 262 closed the literal-safe builtin slice: `BigInt(...)` for supported static inputs, known-BigInt `String(...)` and interpolation ToString, and selected literal `asIntN` / `asUintN` folding.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- none yet; issue is open

Validation result:

```text
not run for this follow-up; issue is open
```

Remaining risks:

- Dynamic builtin semantics must stay within the current runtime BigInt representation or retain source diagnostics.
