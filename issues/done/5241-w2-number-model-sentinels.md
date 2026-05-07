---
id: 5241
title: "W2: implement NaN, Infinity, -0 sentinel values in tagged-number encoding"
type: feature
area: ir/runtime
class: implementation-ready
priority: P2
depends_on: [5240]
blocks: [5242]
created: 2026-05-06
updated: 2026-05-06
---

## Summary

The current runtime represents numbers as tagged integer values (small-int subset
plus heap number for i32 range). NaN, Infinity, -Infinity, and -0 are not
representable: NaN is approximated as `Number(0)`, Infinity as the max payload,
and -0 shares the same tag as 0. This issue adds proper sentinel values so that
observable JavaScript semantics for these special IEEE 754 values are correct.

## Problem

The current approximations break observable JavaScript semantics:

| Expression | Expected | Current (approximation) |
|---|---|---|
| `NaN === NaN` | `false` | `true` (both map to 0) |
| `Object.is(NaN, NaN)` | `true` | `false` (not distinguishable) |
| `Object.is(0, -0)` | `false` | `true` (not distinguishable) |
| `1 / 0` | `Infinity` | `NUMBER_PAYLOAD_MAX / 0` → trap or wrong result |
| `Number.NaN` | `NaN` | `Number(0)` |

Problem: The tagged-number encoding lacks sentinel values for the four IEEE 754
special numbers, breaking observable equality and arithmetic at the W2 boundary.

## Desired final state

A narrow sentinel-based number model where:

1. `NaN` is a new `ValueTag` sentinel (or a reserved tagged-number payload)
2. `+Infinity` and `-Infinity` are separate sentinels
3. `-0` is a separate sentinel
4. Arithmetic operations (`$add`, `$sub`, `$mul`, `$div`, `$mod`) propagate NaN
   and Infinity correctly per IEEE 754 rules for the integer/division subset
5. `$strict_equal` returns the correct result for NaN (always false) and -0 (true with 0)
6. `$abstract_equal` has the same NaN/-0 parity as strict equal
7. `Object.is` (or `$same_value`) is a new RuntimeFn that distinguishes NaN/NaN (true) and 0/-0 (false)
8. Literal `NaN`, `Infinity`, `-Infinity` are lowered to the correct sentinel
   values instead of compile-time approximations

Scope is limited to sentinel values and their correct handling in equality
and the four basic arithmetic operations. Full IEEE 754 floating-point
(`-O3` f64 fast path, fractional values, rounding) is NOT in scope.

## Scope

In scope:

- [ ] Add `ValueTag` sentinels for `NaN`, `Infinity`, `-Infinity`, `-0`
- [ ] Update `can_encode_number` / `encode_number` / `decode_number` in `runtime-abi`
- [ ] Add `SameValue` (Object.is) RuntimeFn if not present
- [ ] Update `$strict_equal` and `$abstract_equal` for NaN/-0 parity
- [ ] Update `$add`, `$sub`, `$mul`, `$div`, `$mod` for NaN/Infinity propagation
- [ ] Emit correct sentinel values for literal `NaN`, `Infinity`, `-Infinity` in resolver
- [ ] Add Node differential fixtures for NaN/Infinity/-0 equality and arithmetic
- [ ] Update `docs/14-runtime-abi.md` with new sentinel encoding

Out of scope:

- Full IEEE 754 double representation (fractional, rounding, `-O3` fast path)
- FPU hardware acceleration
- `Math.*` functions beyond the current subset
- `parseInt` / `parseFloat` with Infinity/NaN

## Affected paths

Expected:

- `crates/runtime-abi/src/value.rs` (ValueTag sentinels)
- `crates/backend-wasm/src/runtime_core_emitter_part1.rs` (arithmetic helpers)
- `crates/backend-wasm/src/runtime_core_comparison_alloc.rs` (strict_equal, abstract_equal)
- `crates/ir/src/lowered/resolver_expr.rs` (literal NaN/Infinity lowering)
- `crates/ir/src/builtin_resolver.rs` (Number.NaN / Infinity resolution)
- `fixtures/core-semantics/` (new NaN/Infinity/-0 fixtures)
- `crates/cli/tests/common/m2_node_diff_fixture_tests.rs`
- `docs/14-runtime-abi.md`
- `current-state.md`

Do not touch:

- `crates/frontend/` (parser already parses NaN/Infinity syntax)
- WASI host imports
- Data model / class / module code paths

## Acceptance criteria

- [ ] `NaN === NaN` evaluates to `false` in generated WASM (not compile-time constant)
- [ ] `Object.is(NaN, NaN)` evaluates to `true`
- [ ] `Object.is(0, -0)` evaluates to `false`
- [ ] `1 / 0` evaluates to `Infinity` (not trap or max-int)
- [ ] `Infinity === Infinity` evaluates to `true`
- [ ] `-Infinity < Infinity` evaluates to `true`
- [ ] `cargo nextest run` passes (all existing tests plus new fixtures)
- [ ] New fixtures in `fixtures/core-semantics/` cover NaN/Infinity/-0/Infinity

## Validation

```sh
cargo fmt --all --check
cargo nextest run
# Verify specific fixtures pass
cargo nextest run -p ts2wasm-cli --test m2_node_diff -- nan_infinity
```

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected
- [ ] updated: `docs/14-runtime-abi.md` (new sentinel encoding)

Current state:

- [ ] not affected
- [ ] updated: `current-state.md`

Follow-up issues:

- [ ] none
- [ ] created/updated: none (sentinels unblock W2; full double is W7)

## Notes

ValueTag currently has 3 low bits: 000 undefined, 001 null, 010 false, 011 true,
100 number (small-int), 101 array, 110 string, 111 object.

NaN/Infinity/-0 can be encoded as reserved number payloads:
- `encode_number(NaN)` → a specific out-of-range payload that operations recognize
- Or as object-tagged heap numbers (reusing the HEAP_NUMBER_SENTINEL pattern)

The heap-number approach is preferred since it reuses the existing object-tagged
heap encoding and keeps the small-int fast path unchanged. A heap NaN would have
the same header as a heap number but with a NaN bit in the flags or a special
decimal string. This avoids changing the ValueTag encoding.

Object.is can be a new RuntimeFn that checks for NaN via heap-header flag and
for -0 via the sign of the encoded payload or heap-number cached decimal string.


## False-done audit

Date: 2026-05-07

Classification: truly-done.

Audit result: retained in issues/done/. Implementation commits confirmed.
