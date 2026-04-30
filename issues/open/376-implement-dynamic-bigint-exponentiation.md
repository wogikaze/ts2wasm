---
id: 376
title: "Implement dynamic BigInt exponentiation"
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

Implement BigInt `**` beyond the issue-371 literal-folding slice.

Problem: BigInt exponentiation where either operand is not a literal-foldable BigInt expression still reports issue-376 diagnostics and must not lower through ordinary number exponentiation.

## Problem

The compiler now folds literal BigInt `**` only when the exponent is a non-negative BigInt literal in `0..=64`. Dynamic operands, identifier-bound operands, and larger exponents need a BigInt-specific runtime/helper policy.

Problem: Dynamic BigInt exponentiation remains unsupported after issue 371 and needs a runtime helper or a broader static fold proof.

## Current failure

```sh
cargo test -p ts2wasm-cli --test m2_node_diff bigint_dynamic_exponentiation_reports_issue_372
```

Representative fixture:

```ts
let base = 2n;
console.log(base ** 3n);
```

Current result: source-backed `issue-376` diagnostic.

## Desired final state

Supported BigInt exponentiation operands produce Node/iwasm-matching output through a BigInt-specific path. Unsupported negative exponent and out-of-slice cases keep source-backed diagnostics or compatible exception behavior.

## Scope

In scope:

- [ ] Define the runtime/static proof boundary for dynamic BigInt `**`.
- [ ] Preserve `RangeError` ownership for negative BigInt exponents through issue 370 unless this issue implements compatible throwing.
- [ ] Add Node/iwasm differential coverage for each implemented exponentiation slice.
- [ ] Keep non-implemented exponentiation forms diagnosed as issue 376 or a narrower child issue.

Out of scope:

- BigInt bitwise operators; issue 377.
- BigInt shift operators and BigInt `>>>` TypeError policy; issue 378.
- Ordinary number exponentiation; issue 296.
- Full arithmetic exception parity unless explicitly coordinated with issue 370.

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
- Ordinary number exponentiation behavior.

## Acceptance criteria

- [ ] Implemented BigInt `**` forms have Node/iwasm differential fixtures.
- [ ] Unsupported BigInt `**` forms do not lower through `Math.pow` or ordinary number helpers.
- [ ] Negative exponent behavior is either compatible with Node `RangeError` or explicitly remains tracked by issue 370.
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

Issue 371 added the narrow literal-folding slice for non-negative literal exponents `0..=64`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- none yet; issue is open
