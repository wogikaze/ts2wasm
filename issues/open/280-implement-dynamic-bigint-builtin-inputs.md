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
- `fixtures/core-semantics/bigint-builtin-dynamic-invalid-string-unsupported.ts`
- `fixtures/core-semantics/bigint-builtin-dynamic-nullish-unsupported.ts`
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

- [x] updated: `docs/language-reference/javascript-features.md`

Current state:

- [x] updated: `current-state.md`

Follow-up issues:

- [ ] none

## Notes

Issue 262 closed the literal-safe builtin slice: `BigInt(...)` for supported static inputs, known-BigInt `String(...)` and interpolation ToString, and selected literal `asIntN` / `asUintN` folding.

2026-04-29 progress: resolver-side pre-folding now recognizes direct
identifier-bound number/BigInt literal inputs for `BigInt.asIntN` and
`BigInt.asUintN` when the value is still known at the call site. This covers
`const bits = 8; const value = 255n; console.log(BigInt.asIntN(bits, value));`
without adding runtime helpers or broadening the runtime BigInt representation.
Bindings invalidated by later assignment remain issue-280 diagnostics.

2026-04-29 progress: dynamic `BigInt.asIntN(bits, value)` and
`BigInt.asUintN(bits, value)` now lower through standalone runtime helpers when
the value is a guarded signed-i64-backed BigInt and the runtime bit width is in
the supported `0..=64` range. Dynamic `BigInt(value)` now lowers for runtime
boolean, tagged-int number, and BigInt inputs. Node/iwasm differential fixtures
cover these dynamic helper paths, and non-BigInt `asIntN` / `asUintN` value
inputs remain issue-280 diagnostics. The issue remains open for dynamic
StringToBigInt and remaining out-of-slice dynamic conversion edges.

2026-04-29 progress: definitely-string dynamic `BigInt(value)` inputs now stay
out of the runtime helper and report source diagnostics linked to issue 280.
This prevents the previous unsupported runtime trap for `let s = "10"; s = s +
""; BigInt(s)`. Runtime StringToBigInt support is still not implemented.

2026-04-29 progress: added regression coverage that keeps dynamic string
`BigInt(value)` and invalid decimal string diagnostics on the source-spanned
issue-280 path. This is a guardrail only; it does not implement runtime
StringToBigInt parsing.

2026-04-29 progress: dynamic definitely-string `BigInt(value)` inputs now lower
through the existing `BigIntFromValue` runtime helper. The runtime helper
parses the current small StringToBigInt-compatible range: ASCII-trimmed decimal
strings with optional sign, unsigned `0b` / `0o` / `0x` prefixes, and
empty/whitespace-to-zero, bounded by the current single-limb/u64 BigInt
representation. `fixtures/core-semantics/bigint-builtin-dynamic-string.ts`
has Node/iwasm differential coverage. Static invalid strings remain
source-spanned issue-280 diagnostics; dynamic invalid/out-of-range strings still
trap until compatible runtime exception throwing is implemented.

2026-04-29 progress: literal-derived dynamic string inputs tracked through
string-only concatenation (for example `s = s + ""`) now keep invalid or
out-of-runtime-range `BigInt(s)` on the source-spanned issue-280 diagnostic
path instead of lowering to a runtime trap. Unknown dynamic string contents
still require compatible runtime exception throwing before the issue can close.

2026-04-29 progress: literal-derived nullish dynamic `BigInt(value)` inputs now
stay out of the `BigIntFromValue` runtime helper and report source diagnostics
linked to issue 280. `fixtures/core-semantics/bigint-builtin-dynamic-nullish-unsupported.ts`
guards the self-assigned local case so known nullish inputs do not regress to
runtime traps.

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
