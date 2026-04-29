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

Current result: dynamic BigInt unary minus and binary `+` / `-` over the current signed-i64-backed helper slice match Node/iwasm output only when the builtin resolver proves the operands/results fit that slice. Out-of-slice dynamic BigInt values, such as `18446744073709551616n` stored in a local and then added, now emit spanned issue-260 diagnostics instead of silently routing through first-limb reconstruction. The guard also invalidates tracked BigInt locals assigned inside branches, loops, switch cases, and try/catch/finally blocks so a later use cannot rely on stale pre-branch safe state. BigInt `*`, `/`, `%`, full multi-limb dynamic arithmetic, and complete runtime TypeError behavior for mixed Number/BigInt operands remain issue-260 work.

Progress result (2026-04-29): BigInt arithmetic where both operands are literal-foldable is resolved at compile time with arbitrary-size decimal math and Node/iwasm coverage. This did not close the runtime helper requirement.

Progress result (2026-04-29, dynamic runtime slice): BigInt locals now lower unary minus and binary `+` / `-` to BigInt runtime helpers instead of generic number operations when a pre-lowering guard proves the runtime helper slice is safe. This helper slice converts through signed i64 and the existing issue-259 first-limb/cached-decimal constructor, so it is not the full canonical multi-limb operation implementation required for final closure.

## Desired final state

BigInt unary minus and binary `+`, `-`, `*`, `/`, and `%` work for BigInt operands with Node differential evidence. Mixed Number/BigInt arithmetic reports or raises the ECMAScript TypeError path, not silent coercion.

## Scope

In scope:

- [ ] Add runtime helpers for BigInt unary minus and core arithmetic.
- [x] Add a dynamic runtime helper slice for BigInt unary minus and binary `+` / `-` over current signed-i64-backed heap BigInt operands.
- [x] Preserve canonical zero for `-0n` in the implemented literal and dynamic unary-minus slices.
- [ ] Implement truncating BigInt division/remainder semantics compatible with Node.
- [x] Add diagnostics for statically visible Number/BigInt arithmetic mixing.
- [x] Add diagnostics for dynamic values outside the signed-i64-backed helper slice so they do not silently miscompile, including after possible control-flow assignments.
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
- [x] Negative fixtures cover large dynamic BigInt add and sub operands, branch-assigned out-of-slice BigInt locals, and dynamic mixed Number/BigInt arithmetic as spanned issue-260 diagnostics.
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

- [ ] create narrower bitwise/exponentiation follow-up if those operators are left unsupported

## Notes

Arithmetic helpers operate on canonical BigInt heap objects and must not depend on JavaScript `number` fast paths. Issue 259 only implemented the observable literal slice using a sign/first-limb prefix plus cached decimal bytes; this issue owns full canonical multi-limb storage/operation correctness before arithmetic can be claimed compatible.

2026-04-29 progress slice: literal-only BigInt arithmetic now folds in the resolver using arbitrary-size decimal math and then emits an ordinary BigInt literal heap object.

2026-04-29 dynamic runtime progress slice: `let x = 1n; console.log(x + 2n);`, subtraction, and unary minus now route through BigInt-specific runtime helpers with Node/iwasm coverage. A pre-lowering guard rejects dynamic helper use when operands/results are not proven signed-i64-safe and reports dynamic mixed Number/BigInt arithmetic with the original source span. The guard conservatively invalidates tracked locals assigned inside nested control-flow bodies to avoid stale safe-state assumptions. The runtime helpers intentionally operate through a signed-i64 reconstruction of the existing first-limb heap payload plus cached decimal construction; full canonical multi-limb runtime storage/operation correctness, multiplication, division, remainder, bitwise/exponentiation policy, and complete mixed Number/BigInt TypeError behavior remain before this issue can close.

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
cargo nextest run -E 'test(bigint) or test(node_diff)'
PASS (18 tests)
mise run update-issue-index -- --check
mise run check issues
cargo nextest run
PASS (477 tests, 4 skipped)
2026-04-29
```

Remaining risks:

- Runtime helpers for dynamic BigInt multiplication, division, and remainder remain unimplemented.
- Dynamic unary/add/sub helpers are signed-i64-backed and do not yet provide full canonical multi-limb arithmetic correctness.
- Complete runtime TypeError behavior for non-statically-known mixed Number/BigInt operands remains incomplete.
