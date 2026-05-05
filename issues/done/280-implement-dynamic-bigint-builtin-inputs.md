---
id: 280
title: "Implement dynamic BigInt builtin inputs"
type: feature
area: runtime/builtins
class: done
priority: P2
depends_on: [259, 262]
blocks: []
created: 2026-04-29
updated: 2026-04-30
completed: 2026-04-30
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

- [x] Implement dynamic `BigInt.asIntN(bits, value)` and `BigInt.asUintN(bits, value)` for supported runtime BigInt values and supported bit widths.
- [x] Broaden `BigInt(value)` runtime conversion for dynamic values where the current value model can preserve Node-compatible behavior.
- [x] Keep diagnostics for out-of-slice values source-linked and issue-280-linked for this slice; unknown dynamic invalid/out-of-range StringToBigInt exception parity is split to issue 333 (done).

Out of scope:

- Full multi-limb BigInt arithmetic beyond the current representation.
- Broader Number model gaps such as `NaN`, `Infinity`, `-0`, and fractional values.
- Object `ToPrimitive` for arbitrary objects.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `crates/cli/tests/common/`
- `fixtures/core-semantics/*bigint*`
- `docs/language-reference/javascript-features.md`
- `current-state.md`

Do not touch:

- unrelated builtin families
- parser BigInt literal syntax

## Acceptance criteria

- [x] Node/iwasm differential fixtures cover dynamic `BigInt.asIntN` and `BigInt.asUintN` inputs in the supported runtime range.
- [x] Dynamic `BigInt(...)` conversions either match Node for supported runtime values or produce source diagnostics for the current slice; remaining unknown dynamic invalid/out-of-range StringToBigInt exception parity is issue 333 (done).
- [x] Runtime linker structure tests cover the BigInt builtin helpers and avoid new host imports.
- [x] Docs/current-state/issues state the supported dynamic builtin subset and remaining limits.

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

- [x] created: `issues/done/333-implement-bigint-dynamic-string-exception-parity.md`

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

2026-04-29 progress: dynamic `BigInt.asIntN(bits, value)` and
`BigInt.asUintN(bits, value)` now accept literal-derived decimal string bit
widths through the runtime helper when the parsed width is within the existing
`0..=64` slice. `fixtures/core-semantics/bigint-builtin-dynamic-as-int-n.ts`
and `fixtures/core-semantics/bigint-builtin-dynamic-as-uint-n.ts` cover the
string-width path with Node/iwasm differential output.

2026-04-30 verification progress: the supported issue-280 dynamic subset is
implemented and narrow BigInt validation passes with
`cargo test -p ts2wasm-cli --test m2_node_diff bigint` (37 passed, 0 failed).
The assigned `cargo nextest run -E 'test(bigint) or test(node_diff)'` command
currently fails before executing issue-280 assertions because parent-state
split test helpers compile as standalone test targets:
`crates/cli/tests/common/m2_node_diff_fixture_tests.rs` reports
`there are too many leading super keywords`, and
`crates/frontend/src/lexer_tests.rs` cannot import lexer/parser symbols from
its module context. Issue 280 is left open as verification-ready until that
unrelated harness blocker is resolved and the assigned broad command can run.
The remaining unknown dynamic invalid/out-of-range StringToBigInt runtime
exception parity has been split to issue 333 (done) rather than broadening issue 280.

2026-04-30 child-280 verification blocker: `cargo fmt --all --check` passed,
and after `mise trust`, `mise run update-issue-index -- --check` and
`mise run check issues` passed. The required broad close gate
`cargo nextest run -E 'test(bigint) or test(node_diff)'` still cannot establish
close evidence because compilation stops before issue-280 assertions run:
`crates/backend-wasm/src/expr_emit.rs` line 1222 reports an unused formatting
argument for `Layout::ARRAY_ELEM_SHIFT`, and the same file line 1196 reports unresolved
`array_push_grow_linear_growth_threshold` in the array-push growth WAT format
string. This is outside the dynamic BigInt builtin slice, so issue 280 remains
open as verification-ready rather than being false-closed.

## Completion evidence

Completed 2026-04-30.

Commits:

- `8cb466da54ca` issue-280: close dynamic bigint builtin slice

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-04-30

command: cargo test -p ts2wasm-cli --test m2_node_diff bigint
result: pass, 37 passed
date: 2026-04-30

command: cargo nextest run -E 'test(bigint) or test(node_diff)'
result: fail only on unrelated issue-357 ABC451 iwasm timeout; 173 selected tests passed, including issue-280 BigInt dynamic builtin coverage and runtime-link BigInt helper tests
date: 2026-04-30

command: mise run update-issue-index -- --check
result: pass after `mise trust` in the fresh worktree
date: 2026-04-30

command: mise run check issues
result: pass after `mise trust` in the fresh worktree
date: 2026-04-30
```

Remaining risks:

- The only broad-filter failure observed during close validation is `m2_node_diff_fixture_tests::abc451_depth8_live_set_fixture_matches_node_output_under_iwasm`, which timed out under `iwasm` and is tracked by issue 357.
- Unknown dynamic invalid/out-of-range StringToBigInt runtime exception parity is intentionally not part of this closure and was tracked by issue 333 (done).
- Dynamic builtin semantics must stay within the current runtime BigInt representation or retain source diagnostics.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/280-implement-dynamic-bigint-builtin-inputs.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
