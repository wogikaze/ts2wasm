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

## Desired final state

BigInt unary minus and binary `+`, `-`, `*`, `/`, and `%` work for BigInt operands with Node differential evidence. Mixed Number/BigInt arithmetic reports or raises the ECMAScript TypeError path, not silent coercion.

## Scope

In scope:

- [ ] Add runtime helpers for BigInt unary minus and core arithmetic.
- [ ] Preserve canonical zero for `-0n`.
- [ ] Implement truncating BigInt division/remainder semantics compatible with Node.
- [ ] Add diagnostics or runtime TypeError handling for Number/BigInt arithmetic mixing.

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
- [ ] Mixed Number/BigInt arithmetic is issue-linked or TypeError-compatible; it is not compiled as number arithmetic.
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
