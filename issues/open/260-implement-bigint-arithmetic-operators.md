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

Current result: unsupported BigInt runtime/operator diagnostics.

Progress result (2026-04-29): BigInt arithmetic where both operands are literal-foldable is resolved at compile time with arbitrary-size decimal math and Node/iwasm coverage. Dynamic BigInt operands still report issue-260 because runtime arithmetic helpers are not implemented yet.

## Desired final state

BigInt unary minus and binary `+`, `-`, `*`, `/`, and `%` work for BigInt operands with Node differential evidence. Mixed Number/BigInt arithmetic reports or raises the ECMAScript TypeError path, not silent coercion.

## Scope

In scope:

- [ ] Add runtime helpers for BigInt unary minus and core arithmetic.
- [ ] Preserve canonical zero for `-0n`.
- [ ] Implement truncating BigInt division/remainder semantics compatible with Node.
- [ ] Add diagnostics or runtime TypeError handling for Number/BigInt arithmetic mixing.
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
- [x] Node/iwasm differential fixture covers literal addition, subtraction, multiplication, division, remainder, unary minus, canonical zero, and values larger than the issue-259 first-limb cache.
- [x] Mixed Number/BigInt arithmetic is issue-linked for the current static slice; it is not compiled as number arithmetic.
- [ ] Runtime linker structure tests cover the selected BigInt arithmetic helpers and their deps.
- [ ] Docs/current-state/issues remain synchronized with the operation boundary.

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

- [ ] updated: `docs/14-runtime-abi.md`
- [ ] updated: `docs/language-reference/javascript-features.md`

Current state:

- [ ] updated: `current-state.md`

Follow-up issues:

- [ ] create narrower bitwise/exponentiation follow-up if those operators are left unsupported

## Notes

Arithmetic helpers operate on canonical BigInt heap objects and must not depend on JavaScript `number` fast paths. Issue 259 only implemented the observable literal slice using a sign/first-limb prefix plus cached decimal bytes; this issue owns full canonical multi-limb storage/operation correctness before arithmetic can be claimed compatible.

2026-04-29 progress slice: literal-only BigInt arithmetic now folds in the resolver using arbitrary-size decimal math and then emits an ordinary BigInt literal heap object. This intentionally does not close the runtime helper requirement: `let x = 1n; console.log(x + 2n);` remains issue-260 unsupported until dynamic BigInt heap operands can be added/subtracted/multiplied/divided at runtime.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`
- `7bf3a05`

Validation result:

```text
cargo nextest run -E 'test(bigint) or test(node_diff)'
PASS (12 tests)
2026-04-29
```

Remaining risks:

- Runtime helpers for dynamic BigInt operands remain unimplemented.
- Full issue closure still requires runtime linker structure tests for selected BigInt arithmetic helpers.
