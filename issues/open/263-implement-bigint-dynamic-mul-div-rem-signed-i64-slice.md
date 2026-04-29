---
id: 263
title: "Implement BigInt dynamic mul/div/rem signed-i64 runtime slice"
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

Finish the next narrow issue-260 arithmetic slice by adding dynamic BigInt `*`, `/`, and `%` helpers for the same proven-safe signed-i64 boundary used by the current unary/add/sub progress slice.

Problem: issue 260 now has literal folding for `*`, `/`, `%` and dynamic signed-i64-backed helpers for unary minus and `+` / `-`, but dynamic multiplication, truncating division, and remainder still diagnose as issue 260. The next child cycle needs a closable, non-silent slice rather than another broad issue-260 progress pass.

## Current failure

```sh
tmp=/tmp/ts2wasm-263-bigint-mul-div-rem.ts
printf 'let a = 6n;\nlet b = 4n;\nconsole.log(a * b);\nconsole.log(a / b);\nconsole.log(a %% b);\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-263-bigint-mul-div-rem.wasm
```

Expected current result: issue-260 unsupported diagnostic for dynamic BigInt multiplication/division/remainder.

## Desired final state

For proven-safe signed-i64-backed BigInt operands and results, dynamic `*`, `/`, and `%` lower to BigInt-specific runtime helpers and match Node output. Out-of-slice values must produce issue-linked diagnostics before lowering rather than silently miscomputing.

## Scope

In scope:

- [ ] Add `BigIntMul`, `BigIntDiv`, and `BigIntRem` runtime helpers for the signed-i64-backed slice.
- [ ] Reuse or extend the issue-260 pre-lowering guard so operands and results must be proven signed-i64-safe.
- [ ] Preserve Node truncating division and remainder sign semantics for the supported slice.
- [ ] Reject division/remainder by zero with an explicit issue-260 diagnostic unless a compatible runtime throw path is implemented.
- [ ] Add Node/iwasm differential fixtures for supported dynamic `*`, `/`, `%`, including negative operands and canonical zero behavior.
- [ ] Add negative fixtures proving large dynamic operands/results and zero division do not silently compile to wrong output.
- [ ] Update issue 260 progress notes, `current-state.md`, `docs/14-runtime-abi.md`, and language reference boundaries.

Out of scope:

- Full canonical multi-limb BigInt arithmetic.
- Equality/comparison/coercion; issue 261.
- BigInt builtins/string conversion; issue 262.
- Bitwise or exponentiation operators.

## Affected paths

Expected:

- `crates/ir/src/builtin_resolver.rs`
- `crates/ir/src/lowered/resolver.rs`
- `crates/backend-wasm/src/runtime_core.rs`
- `crates/backend-wasm/src/runtime_fn.rs`
- `crates/backend-wasm/src/runtime_builder.rs`
- `crates/backend-wasm/src/runtime_link_plan.rs`
- `crates/cli/tests/m2_node_diff.rs`
- `fixtures/core-semantics/`
- `docs/14-runtime-abi.md`
- `docs/language-reference/javascript-features.md`
- `current-state.md`
- `issues/open/260-implement-bigint-arithmetic-operators.md`
- `issues/open/263-implement-bigint-dynamic-mul-div-rem-signed-i64-slice.md`

Do not touch:

- BigInt parser syntax.
- issue 261/262 comparison or builtin behavior.
- Optional chaining or class runtime files.

## Acceptance criteria

- [ ] Node/iwasm differential fixture covers dynamic BigInt `*`, `/`, `%` for supported signed-i64-safe operands.
- [ ] Negative fixture covers dynamic large operand/result rejection before lowering.
- [ ] Negative fixture covers dynamic division/remainder by zero boundary.
- [ ] Runtime linker structure tests cover selected helper deps.
- [ ] issue 260 remains open unless the full issue-260 acceptance criteria are independently satisfied.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(bigint) or test(node_diff)'
cargo nextest run
mise run update-issue-index -- --check
mise run check issues
```

## Notes

This is a child implementation slice split from issue 260 after two mergeable progress passes. The purpose is to produce a closable work item without claiming full BigInt arithmetic compatibility.
