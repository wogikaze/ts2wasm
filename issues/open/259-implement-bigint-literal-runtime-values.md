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

Supported BigInt literals lower to heap BigInt objects through the runtime ABI described in `docs/14-runtime-abi.md`. This closes the observable literal slice: `console.log(1n)`, `String(1n)`, `typeof 1n`, and truthiness for `0n` / non-zero BigInt have Node differential coverage. Full arbitrary-precision multi-limb arithmetic/storage correctness is owned by issue 260.

## Scope

In scope:

- [x] Add a BigInt heap object kind and the literal-slice sign/first-limb payload plus cached decimal bytes.
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

Use the heap object representation accepted by issue 250. Do not add a new `RawValue` low-bit tag. Issue 259 only claims observable literal construction and conversion through the current first-limb-plus-decimal-cache payload; issue 260 owns full canonical multi-limb storage/operation correctness.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `f99a564`

Validation result:

```text
cargo fmt --all --check
PASS
mise run update-issue-index -- --check
PASS
mise run check issues
PASS
cargo nextest run -E 'test(bigint) or test(node_diff)'
PASS (11 tests)
cargo nextest run
PASS (464 tests; 4 skipped)
cargo run -q -p ts2wasm-cli -- build /tmp/ts2wasm-review-neg-bigint.ts -o /tmp/ts2wasm-review-neg-bigint.wasm
PASS: build fails with issue-260 for -1n
2026-04-29
```

Remaining risks:

- BigInt unary/binary arithmetic, full canonical multi-limb operation/storage correctness, equality/comparison/coercion, and broader builtins remain unsupported and tracked by issues 260-262.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/259-implement-bigint-literal-runtime-values.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
