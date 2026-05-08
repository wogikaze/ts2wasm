---
id: 397
title: "BigInt add/sub branch-assigned operands"
type: feature
area: runtime/semantics
class: done
priority: P2
depends_on: [382]
blocks: []
created: 2026-05-01
updated: 2026-05-01
completed: 2026-05-01
---

## Summary

Extend the issue 382 BigInt add/sub runtime slice so locals assigned inside
control-flow branches can be used as later BigInt `+` / `-` operands.

## Problem

The first issue 382 progress slice can compute a large BigInt result inside a
branch and print it, but reusing that branch-assigned local in a later add/sub
expression loses enough static BigInt tracking to fall into the mixed
Number/BigInt diagnostic path.

Problem: `branch = branch - 2n` after `branch = base + one` in an `if` branch
reports issue-370 instead of lowering to BigInt subtraction.

## Current failure

Adding the following tail to
`fixtures/core-semantics/bigint-runtime-large-add-sub.ts` fails during build:

```ts
branch = branch - 2n;
console.log(branch);
```

Observed validation failure:

```text
cargo test -p ts2wasm-cli --test m2_node_diff bigint_large_add_sub
error: [UnsupportedSyntax] issue-370: mixed Number/BigInt arithmetic TypeError parity is not implemented in the dynamic BigInt runtime slice
```

## Desired final state

Branch-assigned BigInt locals that are known to carry BigInt values remain
usable as operands for dynamic BigInt `+` and `-`, including values outside the
signed-i64 helper slice.

## Scope

In scope:

- [x] Preserve BigInt local tracking through the supported control-flow
      assignment shape.
- [x] Add a Node/iwasm differential fixture where a branch-assigned large
      BigInt local is used as a later `+` or `-` operand.
- [x] Keep mixed Number/BigInt TypeError parity owned by issue 381.

Out of scope:

- Mixed Number/BigInt arithmetic.
- Multiplication, division, remainder, exponentiation, and bitwise operators.
- Broad unknown dynamic type analysis beyond the supported BigInt local shape.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `current-state.md`

Do not touch:

- Mixed Number/BigInt TypeError parity implementation.
- BigInt multiplication/division/remainder helpers.

## Acceptance criteria

- [x] A branch-assigned large BigInt local used as a later `+` or `-` operand
      matches Node output under `iwasm`.
- [x] `fixtures/core-semantics/bigint-runtime-large-add-sub.ts` includes the
      branch-assigned operand case or an equivalent dedicated fixture.
- [x] Existing issue 382 add/sub progress fixture continues to pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo test -p ts2wasm-cli --test m2_node_diff bigint_large_add_sub
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

- [x] not affected

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

This issue is split from the first issue 382 progress slice because the runtime
add/sub helper works for the selected large operands, while the remaining gap is
IR BigInt local tracking after branch assignment.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `7d27d84a` issue-397: preserve branch bigint add sub tracking

Validation result:

```text
cargo fmt --all --check
cargo test -p ts2wasm-cli --test m2_node_diff bigint_large_add_sub
mise run update-issue-index -- --check
mise run check issues

All required validation passed in child worktree child-397-20260501-051807.
```

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/397-bigint-add-sub-branch-assigned-operands.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
