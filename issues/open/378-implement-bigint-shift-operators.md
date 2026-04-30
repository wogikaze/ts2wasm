---
id: 378
title: "Implement BigInt shift operators and unsigned-right-shift policy"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P2
depends_on: [260]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Implement BigInt `<<` and `>>`, and preserve the ECMAScript `>>>` TypeError boundary.

Problem: BigInt shift operators currently report issue-378 diagnostics and must not lower through ordinary number shifts; BigInt `>>>` is invalid in JavaScript.

## Problem

BigInt shifts require BigInt-specific width/sign semantics. Unsigned right shift is not defined for BigInt and should become a compatible TypeError path rather than number coercion.

Problem: BigInt shift operators and BigInt `>>>` remain unsupported after issue 371 and need a precise runtime/diagnostic slice.

## Current failure

```sh
cargo test -p ts2wasm-cli --test m2_node_diff bigint_shift_reports_issue_374
```

Representative fixture:

```ts
console.log(1n << 2n);
```

Current result: source-backed `issue-378` diagnostic.

## Desired final state

Supported BigInt `<<` and `>>` forms produce Node/iwasm-matching output. BigInt `>>>` reports or throws the compatible unsupported-TypeError path without number coercion.

## Scope

In scope:

- [ ] Implement a first BigInt `<<` / `>>` slice with Node/iwasm coverage.
- [ ] Add negative coverage for BigInt `>>>` showing it is not lowered through number unsigned-shift semantics.
- [ ] Preserve diagnostics for unsupported dynamic, out-of-slice, or mixed Number/BigInt cases.

Out of scope:

- BigInt exponentiation; issue 376.
- BigInt NOT/AND/OR/XOR; issue 377.
- Ordinary number shift behavior.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `docs/14-runtime-abi.md`
- `current-state.md`

Do not touch:

- Parser BigInt literal syntax.
- Ordinary number shift lowering unless a shared diagnostic must stay coherent.

## Acceptance criteria

- [ ] Implemented BigInt shift forms have Node/iwasm differential fixtures.
- [ ] BigInt `>>>` has source-backed diagnostic or compatible TypeError coverage.
- [ ] No BigInt shift path lowers through ordinary number shift operators.
- [ ] Docs/current-state/issues are synchronized.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo test -p ts2wasm-cli --test m2_node_diff bigint
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo nextest run -E 'test(bigint) or test(node_diff)'
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] update if runtime ABI or supported subset changes

Current state:

- [ ] update `current-state.md` when behavior changes

Follow-up issues:

- [ ] none yet

## Notes

BigInt `>>>` is a JavaScript TypeError path, not an arithmetic shift.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- none yet; issue is open
