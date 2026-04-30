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
completed: 2026-05-01
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

- [x] Define the runtime/static proof boundary for dynamic BigInt `**`.
- [x] Preserve `RangeError` ownership for negative BigInt exponents through issue 370 unless this issue implements compatible throwing.
- [x] Add Node/iwasm differential coverage for each implemented exponentiation slice.
- [x] Keep non-implemented exponentiation forms diagnosed as issue 376 or a narrower child issue.

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

- [x] Implemented BigInt `**` forms have Node/iwasm differential fixtures.
- [x] Unsupported BigInt `**` forms do not lower through `Math.pow` or ordinary number helpers.
- [x] Negative exponent behavior is either compatible with Node `RangeError` or explicitly remains tracked by issue 370.
- [x] Docs/current-state/issues are synchronized.

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

- [x] update if runtime ABI or supported subset changes

Current state:

- [x] update `current-state.md` when behavior changes

Follow-up issues:

- [x] none yet

## Notes

Issue 371 added the narrow literal-folding slice for non-negative literal exponents `0..=64`.

## Completion evidence

Implemented the signed-i64-safe dynamic BigInt exponentiation slice:

- Known BigInt local/literal `**` operands lower through the BigInt-specific `BigIntPow` runtime helper.
- Implemented forms have Node/iwasm differential coverage in `fixtures/core-semantics/bigint-runtime-pow.ts`.
- Out-of-slice dynamic exponentiation remains diagnosed as issue 376.
- Negative exponent RangeError parity remains tracked by issue 370 and is covered by `fixtures/core-semantics/bigint-exponentiation-negative-unsupported.ts`.
- Runtime helper is cataloged through `RuntimeFn`, has no host imports, and is covered by the runtime link plan test path.

Validation:

- `cargo fmt --all --check`: pass
- `cargo test -p ts2wasm-cli --test m2_node_diff bigint`: pass (46 passed; 0 failed; 141 filtered out)
- `mise run update-issue-index -- --check`: pass
- `mise run check issues`: pass

Commits:

- `0c7876f6` issue-376: implement dynamic bigint exponentiation slice
