---
id: 371
title: "Define BigInt bitwise and exponentiation policy"
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

Define and implement the BigInt bitwise and exponentiation operator policy that was deliberately left outside issue 260's arithmetic slice.

Problem: issue 260 closed unary minus and binary `+`, `-`, `*`, `/`, `%` for the current supported arithmetic slice, while BigInt bitwise operators and `**` still report diagnostics.

## Problem

BigInt supports several bitwise operators and exponentiation with semantics that differ from the current integer-number helper paths. The compiler must not route these through number operations or leave them attached to a closed issue.

Problem: BigInt bitwise and exponentiation operators remain unsupported and need a precise compatibility policy and implementation slices.

## Current failure

Representative unsupported cases:

```sh
cargo test -p ts2wasm-cli bigint_bitwise_unary_reports_issue_371
cargo test -p ts2wasm-cli bigint_exponentiation_reports_issue_371
```

These fixtures currently prove the operators are rejected with issue-371 diagnostics instead of being silently compiled as number operations.

## Desired final state

BigInt bitwise and exponentiation operators are either implemented with Node/iwasm differential evidence or intentionally split into narrower operator-specific issues with documented diagnostics.

## Scope

In scope:

- [x] Decide the implementation order for BigInt `~`, `&`, `|`, `^`, `<<`, `>>`, unsupported `>>>`, and `**`.
- [x] Implement at least one mergeable operator slice or split precise implementation-ready child issues.
- [x] Preserve diagnostics for operators not implemented in the selected slice.
- [x] Add Node/iwasm differential coverage for implemented operators and negative coverage for intentionally unsupported operators.
- [x] Update docs/current-state/issues with the policy and supported subset.

Out of scope:

- Dynamic unary/add/sub/mul/div/rem arithmetic already closed by issue 260 for the supported slice.
- Full multi-limb arithmetic; issue 369.
- BigInt arithmetic exception parity; issue 370.
- Small-int number exponentiation; issue 296.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `docs/language-reference/javascript-features.md`
- `current-state.md`

Do not touch:

- Parser BigInt literal syntax.
- Ordinary number bitwise/exponentiation behavior unless a shared diagnostic must stay coherent.
- Broad exception machinery except for BigInt exponent negative-exponent policy notes.

## Acceptance criteria

- [x] The BigInt bitwise/exponentiation policy is documented in `current-state.md` and the language reference.
- [x] Implemented operators have Node/iwasm differential fixtures.
- [x] Unsupported operators have source-backed diagnostics that reference narrower child issues 376, 377, and 378.
- [x] No BigInt bitwise or exponentiation path silently lowers to ordinary number arithmetic.
- [x] Follow-up issues 376, 377, and 378 own the remaining unsupported operators.

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

- [x] updated: `docs/language-reference/javascript-features.md`

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] created: `issues/done/376-implement-dynamic-bigint-exponentiation.md`, `issues/open/377-implement-bigint-bitwise-not-and-or-xor.md`, `issues/done/378-implement-bigint-shift-operators.md`

## Notes

Unsigned right shift `>>>` is not a valid BigInt operation in JavaScript and should be handled as a compatibility error, not as number coercion.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `ca42ba2b` implements literal BigInt exponentiation folding, retargets unsupported diagnostics, adds follow-up issues, and updates docs/fixtures.

Validation result:

```text
cargo fmt --all --check: pass
cargo test -p ts2wasm-cli --test m2_node_diff bigint: pass (44 passed; 141 filtered out)
mise run update-issue-index -- --check: pass
mise run check issues: pass
```

Remaining risks:

- Dynamic BigInt exponentiation remains issue 376.
- BigInt bitwise NOT/AND/OR/XOR signed-i64 slice is closed by issue 377; out-of-slice bitwise remains issue 387.
- BigInt shift operators and BigInt >>> TypeError policy remain issue 378.
- Negative BigInt exponent compatible RangeError throwing remains issue 370.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/371-define-bigint-bitwise-and-exponentiation-policy.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
