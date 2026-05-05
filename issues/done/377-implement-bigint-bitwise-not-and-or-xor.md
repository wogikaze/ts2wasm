---
id: 377
title: "Implement BigInt bitwise NOT/AND/OR/XOR"
type: feature
area: runtime/semantics
class: done
priority: P2
depends_on: [260]
blocks: []
created: 2026-05-01
updated: 2026-05-01
completed: 2026-05-01
---

## Summary

Implement BigInt `~`, `&`, `|`, and `^` through BigInt-specific semantics.

Problem: BigInt bitwise NOT/AND/OR/XOR currently report issue-377 diagnostics and must not reuse ordinary number bitwise lowering.

## Problem

BigInt bitwise operators use arbitrary-width two's-complement semantics. The current runtime representation and arithmetic helpers do not yet provide that operation family.

Problem: BigInt bitwise NOT/AND/OR/XOR remain unsupported after issue 371 and need a BigInt-specific implementation slice.

## Current failure

```sh
cargo test -p ts2wasm-cli --test m2_node_diff bigint_bitwise_unary_reports_issue_373
cargo test -p ts2wasm-cli --test m2_node_diff bigint_bitwise_binary_reports_issue_373
```

Representative fixtures:

```ts
console.log(~1n);
console.log(1n & 3n);
```

Current result: source-backed `issue-377` diagnostic.

## Desired final state

Supported BigInt bitwise NOT/AND/OR/XOR forms produce Node/iwasm-matching output through BigInt-specific lowering/runtime helpers.

## Scope

In scope:

- [x] Define the first supported BigInt bitwise NOT/AND/OR/XOR slice.
- [x] Add Node/iwasm differential fixtures for implemented operators.
- [x] Preserve diagnostics for unsupported dynamic, out-of-slice, or mixed Number/BigInt cases; remaining out-of-slice forms are tracked by issue 387.

Out of scope:

- BigInt exponentiation; issue 376.
- BigInt shift operators and unsigned right shift; issue 378.
- Ordinary number bitwise behavior.

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
- Ordinary number bitwise lowering unless a shared diagnostic must stay coherent.

## Acceptance criteria

- [x] Implemented BigInt bitwise NOT/AND/OR/XOR forms have Node/iwasm differential fixtures.
- [x] Unsupported forms keep source-backed issue diagnostics through issue 387.
- [x] No BigInt bitwise path lowers through ordinary number bitwise operators.
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

- [x] updated for the signed-i64-safe helper slice

Current state:

- [x] updated `current-state.md` for the signed-i64-safe helper slice

Follow-up issues:

- [x] created issue 387 for out-of-slice BigInt bitwise

## Notes

Unsigned right shift is intentionally not part of this issue.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending final commit hash

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/377-implement-bigint-bitwise-not-and-or-xor.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
