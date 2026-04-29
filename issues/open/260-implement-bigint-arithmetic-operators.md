---
id: 260
title: "Implement BigInt arithmetic operators"
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

Implement BigInt arithmetic after literal runtime values exist.

Problem: Operators such as `1n + 2n` and `-1n` require BigInt-specific runtime helpers and must not reuse small-int `number` semantics.

## Current failure

```sh
tmp=/tmp/ts2wasm-260-bigint-arithmetic.ts
printf 'console.log(1n + 2n); console.log(5n / 2n); console.log(-0n);\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-260-bigint-arithmetic.wasm
```

Current result: dynamic BigInt unary minus and binary `+`, `-`, `*`, `/`, `%` over the current signed-i64-backed helper slice match Node/iwasm output only when the builtin resolver proves the operands/results fit that slice. Out-of-slice dynamic BigInt values, such as `18446744073709551616n` stored in a local and then added, now emit spanned issue-260 diagnostics instead of silently routing through first-limb reconstruction. The guard also invalidates tracked BigInt locals assigned inside branches, loops, switch cases, and try/catch/finally blocks so a later use cannot rely on stale pre-branch safe state. Dynamic division/remainder by a known zero BigInt local now lowers to the runtime helper slice and traps at runtime instead of being rejected before lowering; true JS `RangeError` object throwing still remains issue-260 work. Full multi-limb dynamic arithmetic and complete runtime TypeError behavior for mixed Number/BigInt operands remain issue-260 work.

Progress result (2026-04-29): BigInt arithmetic where both operands are literal-foldable is resolved at compile time with arbitrary-size decimal math and Node/iwasm coverage. This did not close the runtime helper requirement.

Progress result (2026-04-29, dynamic runtime slice): BigInt locals now lower unary minus and binary `+` / `-` to BigInt runtime helpers instead of generic number operations when a pre-lowering guard proves the runtime helper slice is safe. This helper slice converts through signed i64 and the existing issue-259 first-limb/cached-decimal constructor, so it is not the full canonical multi-limb operation implementation required for final closure.

Progress result (2026-04-29, issue 263 slice): dynamic BigInt `*`, `/`, and `%` now lower to signed-i64-backed BigInt runtime helpers under the same pre-lowering proof boundary. Node/iwasm fixtures cover negative operands, truncating division, remainder sign semantics, and canonical zero; large dynamic multiplication results produce issue-260 diagnostics instead of silently lowering.

Progress result (2026-04-29, division-by-zero runtime trap slice): dynamic division/remainder by a known zero BigInt local now builds successfully and the signed-i64-backed runtime helpers explicitly trap on zero divisors. Node baseline evidence confirms these fixtures are `RangeError: Division by zero`; iwasm currently reports an `unreachable` trap because the project does not yet have compatible JS exception object throwing for this path.

Progress result (2026-04-29, known-local/literal mul-div-rem slice): dynamic BigInt `*`, `/`, and `%` now pass the early resolver for known BigInt local plus BigInt literal operand pairs and remain guarded by the same signed-i64 pre-lowering proof before runtime helper lowering. Node/iwasm fixture coverage was extended for local/literal and literal/local multiplication, truncating division, and remainder.

## Desired final state

BigInt unary minus and binary `+`, `-`, `*`, `/`, and `%` work for BigInt operands with Node differential evidence. Mixed Number/BigInt arithmetic reports or raises the ECMAScript TypeError path, not silent coercion.

## Scope

In scope:

- [ ] Add runtime helpers for BigInt unary minus and core arithmetic.
- [x] Add a dynamic runtime helper slice for BigInt unary minus and binary `+` / `-` over current signed-i64-backed heap BigInt operands.
- [x] Add a dynamic runtime helper slice for BigInt `*`, `/`, and `%` over current signed-i64-backed heap BigInt operands/results, including known-local/literal operand pairs.
- [x] Preserve canonical zero for `-0n` in the implemented literal and dynamic unary-minus slices.
- [x] Implement truncating BigInt division/remainder semantics compatible with Node for the current signed-i64-backed runtime helper slice.
- [x] Add diagnostics for statically visible Number/BigInt arithmetic mixing.
- [x] Add diagnostics for dynamic values outside the signed-i64-backed helper slice so they do not silently miscompile, including after possible control-flow assignments and large dynamic multiplication results.
- [x] Add a compiler-side literal-folding slice for BigInt unary minus and literal `+`, `-`, `*`, `/`, `%`.

Out of scope:

- BigInt literal allocation; issue 259.
- Equality/relational comparison/coercion; issue 261.
- BigInt builtins; issue 262.
- Bitwise and exponentiation operators unless explicitly split from this issue.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/runtime-abi/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/`
- `docs/14-runtime-abi.md`
- `current-state.md`

Do not touch:

- parser BigInt literal syntax
- unrelated arithmetic behavior

## Acceptance criteria

- [ ] Node/iwasm differential fixtures cover addition, subtraction, multiplication, division, remainder, unary minus, and canonical zero.
- [x] Node/iwasm differential fixture covers dynamic BigInt unary minus, binary addition/subtraction, assignment through a known BigInt local, negative results, and canonical zero for the current signed-i64-backed runtime helper slice.
- [x] Node/iwasm differential fixture covers dynamic BigInt multiplication, truncating division, remainder sign semantics, negative operands, canonical zero, and known-local/literal operand pairs for the current signed-i64-backed runtime helper slice.
- [x] Negative/runtime failure fixtures cover large dynamic BigInt add/sub operands, branch-assigned out-of-slice BigInt locals, large dynamic multiplication results, dynamic mixed Number/BigInt arithmetic as issue-260 diagnostics, and dynamic division/remainder by zero as runtime traps with Node `RangeError` baseline evidence.
- [x] Node/iwasm differential fixture covers literal addition, subtraction, multiplication, division, remainder, unary minus, canonical zero, and values larger than the issue-259 first-limb cache.
- [x] Mixed Number/BigInt arithmetic is issue-linked for the current static slice; it is not compiled as number arithmetic.
- [x] Runtime linker structure tests cover the selected BigInt arithmetic helpers and their deps.
- [x] Docs/current-state/issues remain synchronized with the operation boundary for this progress slice.

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

- [x] updated: `docs/14-runtime-abi.md`
- [x] updated: `docs/language-reference/javascript-features.md`

Current state:

- [x] updated: `current-state.md`

Follow-up issues:

- [x] completed `issues/done/263-implement-bigint-dynamic-mul-div-rem-signed-i64-slice.md` for dynamic `*` / `/` / `%` signed-i64 slice
- [ ] create narrower bitwise/exponentiation follow-up if those operators are left unsupported

## Notes

Arithmetic helpers operate on canonical BigInt heap objects and must not depend on JavaScript `number` fast paths. Issue 259 only implemented the observable literal slice using a sign/first-limb prefix plus cached decimal bytes; this issue owns full canonical multi-limb storage/operation correctness before arithmetic can be claimed compatible.

2026-04-29 progress slice: literal-only BigInt arithmetic now folds in the resolver using arbitrary-size decimal math and then emits an ordinary BigInt literal heap object.

2026-04-29 dynamic runtime progress slice: `let x = 1n; console.log(x + 2n);`, subtraction, and unary minus now route through BigInt-specific runtime helpers with Node/iwasm coverage. A pre-lowering guard rejects dynamic helper use when operands/results are not proven signed-i64-safe and reports dynamic mixed Number/BigInt arithmetic with the original source span. The guard conservatively invalidates tracked locals assigned inside nested control-flow bodies to avoid stale safe-state assumptions. The runtime helpers intentionally operate through a signed-i64 reconstruction of the existing first-limb heap payload plus cached decimal construction; full canonical multi-limb runtime storage/operation correctness, multiplication, division, remainder, bitwise/exponentiation policy, and complete mixed Number/BigInt TypeError behavior remain before this issue can close.

2026-04-29 division-by-zero runtime trap slice: `let z = 0n; console.log(a / z);` and `console.log(a % z);` now lower through `BigIntDiv` / `BigIntRem` when the operands remain inside the signed-i64-backed helper slice. The helpers check the signed-i64 divisor and trap with `unreachable` for zero. This removes the pre-lowering false boundary for dynamic zero divisors but does not claim compatible `RangeError` throwing yet.

2026-04-29 known-local/literal mul-div-rem slice: `let a = 6n; console.log(a * 2n);`, `console.log(a / 2n);`, `console.log(a % 4n);`, `console.log(12n * b);`, `console.log(12n / b);`, and `console.log(12n % b);` now pass the early resolver and lower through the existing signed-i64-backed `BigIntMul` / `BigIntDiv` / `BigIntRem` helper slice. The pre-lowering guard still rejects out-of-slice values/results before lowering.

Progress validation (2026-04-29, known-local/literal mul-div-rem slice):

```text
cargo test -p ts2wasm-cli bigint_runtime_mul_div_rem_fixture_matches_node_output_under_iwasm
PASS (1 test)

cargo test -p ts2wasm-cli bigint
PASS (32 tests selected by cargo filter: 3 dump_cli + 29 m2_node_diff)

cargo fmt --all --check
PASS

cargo nextest run
PASS (540 passed, 4 skipped)

mise run update-issue-index -- --check
PASS

mise run check issues
FAIL before issue-260 close: unrelated issue files reference missing artifacts/coverage/results/test262-results.jsonl
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`
- `829946d`

Validation result:

```text
cargo nextest run -E 'test(bigint) or test(node_diff)'
PASS (12 tests)
2026-04-29

cargo nextest run -E 'test(bigint)'
PASS (14 tests)
2026-04-29

cargo fmt --all --check
PASS

cargo nextest run -E 'test(bigint_runtime_div_zero_traps_after_successful_build) or test(bigint_runtime_rem_zero_traps_after_successful_build)'
PASS (2 tests)

cargo nextest run -E 'test(bigint) or test(node_diff)'
PASS (30 tests)

mise run update-issue-index -- --check
PASS

mise run check issues
PASS

cargo nextest run
PASS (500 tests, 4 skipped)
2026-04-29
```

Remaining risks:

- Dynamic unary/add/sub/mul/div/rem helpers are signed-i64-backed and do not yet provide full canonical multi-limb arithmetic correctness.
- Dynamic division/remainder by zero now reaches a runtime trap for the signed-i64-backed helper slice, but it does not yet throw a compatible JavaScript `RangeError` object.
- Bitwise/exponentiation policy and complete runtime TypeError behavior for non-statically-known mixed Number/BigInt operands remain incomplete.
