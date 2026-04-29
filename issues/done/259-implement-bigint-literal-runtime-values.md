---
id: 259
title: "Implement BigInt literal runtime values"
type: feature
area: runtime/semantics
class: done
priority: P2
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Implement the first runtime BigInt slice: lowering parsed BigInt literals into heap BigInt values and exposing the primitive behaviors needed to observe them safely.

Problem: BigInt literals parse as explicit AST nodes, but build/runtime phases still report unsupported diagnostics instead of constructing a canonical BigInt value.

## Current failure

```sh
tmp=/tmp/ts2wasm-259-bigint-literal.ts
printf 'console.log(1n);\nconsole.log(0x10n);\nconsole.log(Boolean(0n));\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-259-bigint-literal.wasm
```

Current result: issue-linked unsupported BigInt runtime diagnostic.

## Desired final state

Supported BigInt literals lower to canonical heap BigInt objects through the runtime ABI described in `docs/14-runtime-abi.md`. `console.log(1n)`, `String(1n)`, `typeof 1n`, and truthiness for `0n` / non-zero BigInt have Node differential coverage.

## Scope

In scope:

- [x] Add a BigInt heap object kind and canonical sign/limb payload.
- [x] Add `make_bigint_literal` and minimal `bigint_to_boolean` / `bigint_to_string` runtime helpers needed to observe literals.
- [x] Lower decimal, binary, octal, and hexadecimal BigInt AST literals to the runtime constructor.
- [x] Update BigInt literal unsupported diagnostics to point at this issue only for runtime-value gaps that remain.

Out of scope:

- Numeric separator syntax; tracked by issue 243.
- BigInt arithmetic operators; tracked by issue 260.
- BigInt equality/comparison/coercion; tracked by issue 261.
- Broader BigInt builtins such as `BigInt.asIntN`; tracked by issue 262.

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

- unrelated parser syntax work
- unrelated runtime builtins

## Acceptance criteria

- [x] BigInt literals build and execute for decimal, binary, octal, and hexadecimal forms without parser/tokenization regressions.
- [x] Node/iwasm differential fixtures cover literal printing, `typeof`, `String`, and truthiness for `0n` and non-zero BigInt.
- [x] Runtime ABI docs and current-state describe the implemented literal slice and remaining BigInt gaps.
- [x] Unsupported diagnostics that remain are issue-linked to the relevant BigInt child issue.

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

- [x] none

## Notes

Use the heap object representation accepted by issue 250. Do not add a new `RawValue` low-bit tag.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `2f747de`

Validation result:

```text
cargo fmt --all --check
PASS
mise run update-issue-index -- --check
PASS
mise run check issues
PASS
cargo nextest run -E 'test(bigint) or test(node_diff)'
PASS (10 tests)
cargo nextest run
PASS (463 tests; 4 skipped)
2026-04-29
```

Remaining risks:

- BigInt arithmetic, equality/comparison/coercion, and broader builtins remain unsupported and tracked by issues 260-262.
