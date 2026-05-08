---
id: 262
title: "Implement BigInt builtins and string conversion"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P2
depends_on: [259]
blocks: []
created: 2026-04-29
updated: 2026-04-29
completed: 2026-04-29
status: done
---

## Summary

Implement the BigInt builtin surface that depends on runtime BigInt values: `BigInt(...)`, string conversion, and selected static BigInt functions.

Problem: After BigInt values exist, builtin and conversion behavior still needs explicit runtime helpers and diagnostics so `BigInt` is not treated as an ordinary unresolved identifier or generic object.

## Current failure

```sh
tmp=/tmp/ts2wasm-262-bigint-builtins.ts
printf 'console.log(BigInt("10")); console.log(String(10n)); console.log(BigInt.asIntN(8, 255n));\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-262-bigint-builtins.wasm
```

Current result: unsupported BigInt runtime/builtin diagnostics.

## Desired final state

Supported BigInt builtins are resolved by `BuiltinResolver`, lowered through explicit runtime helpers, and covered by Node differential fixtures. Unsupported builtin forms produce issue-linked diagnostics rather than generic unsupported builtin messages.

## Scope

In scope:

- [x] Implement `BigInt(value)` for supported string, boolean, integer number, and BigInt inputs.
- [x] Implement `String(bigint)` / runtime `ToString` for BigInt without `n` suffix.
- [x] Implement `BigInt.asIntN` and `BigInt.asUintN` for supported integer bit widths, or split them into narrower follow-up issues if needed.
- [x] Add issue-linked diagnostics for unsupported BigInt builtin/coercion forms.

Out of scope:

- BigInt literal allocation; issue 259.
- BigInt arithmetic; issue 260.
- BigInt equality/comparison; issue 261.
- Full object `ToPrimitive` if the broader object model cannot support it yet.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/runtime-abi/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/`
- `docs/language-reference/javascript-features.md`
- `current-state.md`

Do not touch:

- unrelated builtin families
- parser BigInt literal syntax

## Acceptance criteria

- [x] Node/iwasm differential fixtures cover `BigInt(...)`, `String(bigint)`, template/string interpolation involving BigInt, and selected `BigInt.asIntN` / `BigInt.asUintN` cases or a split follow-up if those are not in this slice.
- [x] Unsupported BigInt builtin and conversion forms produce source diagnostics with issue 262 for the stable `new BigInt(...)` constructor rejection or issue 280 for residual dynamic builtin inputs.
- [x] Runtime linker structure tests cover selected BigInt builtin helpers and avoid new host imports.
- [x] Docs/current-state/issues state the supported builtin subset and remaining limits.

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

- [x] updated: `docs/language-reference/javascript-features.md`

Current state:

- [x] updated: `current-state.md`

Follow-up issues:

- [x] created narrower follow-up for broader dynamic BigInt builtin inputs: `issues/done/280-implement-dynamic-bigint-builtin-inputs.md`

## Notes

`BigInt` is not a constructor with `new`; `new BigInt(...)` must remain an error-compatible path.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `fb4474a` issue-262 close commit in branch `agent/262-bigint-builtins-close-20260429T092800Z`.

Validation result:

```text
command: cargo fmt --all --check
result: PASS
date: 2026-04-29

command: cargo nextest run -E 'test(bigint) or test(node_diff)'
result: PASS (34 tests passed, 494 skipped)
date: 2026-04-29

command: cargo nextest run
result: PASS (524 tests passed, 4 skipped)
date: 2026-04-29
```

Remaining risks:

- The closed slice covers the documented literal-safe builtin subset and the stable `new BigInt(...)` constructor rejection. Broader dynamic `BigInt(...)`, `BigInt.asIntN`, and `BigInt.asUintN` inputs are split to issue 280.

## Progress evidence

2026-04-29 progress slice:

- Implemented `BigInt(...)` builtin folding for supported string, boolean, integer number, unary-negative integer number, and BigInt literal inputs.
- Implemented known-BigInt `String(...)` lowering through `BigIntToString`, including known BigInt locals.
- Corrected runtime BigInt ToString behavior used by concatenation/template interpolation to omit the `n` suffix, while keeping `console.log(bigint)` output with the `n` suffix.
- Added issue-262 diagnostics for unsupported `BigInt(...)` inputs, invalid decimal string literals, `new BigInt(...)`, and `BigInt.asIntN` / `BigInt.asUintN`.
- Added Node/iwasm differential coverage: `fixtures/core-semantics/bigint-builtins-string-conversion.ts`.
- Added unsupported diagnostics coverage: `fixtures/core-semantics/bigint-builtin-as-int-n-unsupported.ts`, `fixtures/core-semantics/bigint-builtin-as-uint-n-unsupported.ts`, `fixtures/core-semantics/bigint-builtin-invalid-string-unsupported.ts`, and `fixtures/core-semantics/bigint-new-unsupported.ts`.
- Added runtime linker coverage that `BigIntToString` and `MakeBigIntLiteral` are selected without host imports.

Validation recorded in the child branch:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(bigint_builtin_string_conversion_fixture_matches_node_output_under_iwasm) or test(bigint_builtin_unsupported_forms_report_issue_262) or test(bigint_builtin_string_conversion_selects_helper_deps_without_imports)'
cargo nextest run -E 'test(bigint) or test(node_diff)'
cargo nextest run
mise run update-issue-index -- --check
mise run check issues
```

Remaining issue-262 work at that point:

- Implement `BigInt.asIntN` / `BigInt.asUintN` semantics.
- Broaden `BigInt(string)` beyond the current literal-safe StringToBigInt subset if full ECMAScript string parsing is required.

2026-04-29 progress slice for `BigInt(string)` radix forms:

- Broadened resolver-side `BigInt("<string>")` folding from decimal-only strings to unsigned binary (`0b`/`0B`), octal (`0o`/`0O`), and hexadecimal (`0x`/`0X`) integer string literals.
- Matched Node for empty/whitespace-only string inputs by folding them to `0n`.
- Kept explicit signs decimal-only so `BigInt("-0x10")` and other invalid signed non-decimal strings stay on issue-262 diagnostics instead of being silently accepted.
- Extended `fixtures/core-semantics/bigint-builtins-string-conversion.ts` with binary/octal/hex/empty string cases, and split invalid string coverage so malformed decimal and signed non-decimal strings both report issue-262.

Validation recorded in the child branch:

```sh
cargo nextest run -E 'test(bigint_builtin_string_conversion_fixture_matches_node_output_under_iwasm) or test(bigint_builtin_unsupported_forms_report_issue_262)'
PASS
```

Residual behavior split after close:

- Runtime/helper support for nonliteral `BigInt.asIntN` / `BigInt.asUintN` inputs if required beyond the current literal-safe subset is tracked by issue 280.
- Broader StringToBigInt compatibility edge cases outside the current literal-safe subset.

2026-04-29 progress slice for `BigInt.asIntN` / `BigInt.asUintN`:

- Implemented resolver-side folding for `BigInt.asIntN(bits, value)` and `BigInt.asUintN(bits, value)` when `bits` is an integer number literal in `0..=64` and `value` resolves to a BigInt literal.
- Covered signed wrap, unsigned wrap, zero-width, string conversion of the result, and 64-bit boundary values in `fixtures/core-semantics/bigint-builtin-as-int-n.ts`.
- Kept unsupported diagnostics for dynamic bit widths and non-BigInt value inputs in the existing issue-262 unsupported fixtures.
- Preserved the existing `BigInt(...)` and `String(bigint)` behavior from the earlier slice.

Validation recorded in the child branch:

```sh
cargo fmt --all --check
PASS

cargo nextest run -E 'test(bigint_builtin_as_int_n_fixture_matches_node_output_under_iwasm) or test(bigint_builtin_unsupported_forms_report_issue_262)'
PASS (2 tests)

cargo nextest run -E 'test(bigint) or test(node_diff)'
PASS (31 tests)

mise run update-issue-index -- --check
PASS

mise run check issues
PASS

cargo nextest run
PASS (503 tests, 4 skipped)
```

Remaining issue-262 work after this slice:

- Runtime/helper support for nonliteral `BigInt.asIntN` / `BigInt.asUintN` inputs if required beyond the current literal-safe subset.
- Broaden `BigInt(string)` beyond the current decimal integer string subset if full ECMAScript string-to-BigInt parsing is required.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/262-implement-bigint-builtins-and-string-conversion.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
