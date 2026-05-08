---
id: 384
title: "Multi-limb BigInt division and remainder"
type: feature
area: runtime/semantics
class: done
priority: P2
depends_on: [259, 260, 391, 392]
blocks: []
created: 2026-05-01
updated: 2026-05-01
completed: 2026-05-01
---

## Summary

This issue was split into focused child issues, then closed for the validated known-BigInt division/remainder operand slice. Remaining control-flow-assigned operand tracking is split to issue 398.

## Problem

The current dynamic helpers reconstruct operands through signed i64 and the issue-259 first-limb/cached-decimal constructor. This is safe only when the resolver proves operands and results fit the signed-i64 helper slice, but dynamic BigInt division/remainder outside that slice is rejected with issue-369 diagnostics instead of matching Node for arbitrary BigInt magnitudes.

## Current failure

Representative unsupported cases:

```sh
cargo test -p ts2wasm-cli bigint_runtime_large_div_reports_issue_369
```

Result:

```text
error: issue-369: dynamic BigInt division outside signed-i64 helper slice
```

```sh
cargo test -p ts2wasm-cli bigint_runtime_large_rem_reports_issue_369
```

Result:

```text
error: issue-369: dynamic BigInt remainder outside signed-i64 helper slice
```

These fixtures currently build-fail with an issue-369 diagnostic because the operands or results are outside the signed-i64-backed dynamic helper slice.

## Desired final state

Dynamic BigInt `/` and `%` operate on the canonical heap BigInt limb representation for arbitrary supported BigInt magnitudes and match Node output.

## Scope

In scope:

- [x] Implement cached-decimal truncating division and remainder for known dynamic BigInt operands and results outside signed i64.
- [x] Preserve canonical zero and sign behavior for division and remainder.
- [x] Keep source-backed diagnostics for remaining unsupported control-flow-assigned BigInt tracking in issue 398 and exception parity in issue 370.
- [x] Add Node/iwasm differential fixtures for values larger than signed i64.
- [x] Existing runtime linker structure tests still cover BigIntDiv/BigIntRem catalog selection; deps now route BigIntRem through BigIntDiv helper emission.
- [x] Update `docs/14-runtime-abi.md`, `docs/language-reference/javascript-features.md`, and `current-state.md`.

Out of scope:

- Parser BigInt literal syntax.
- Literal-only arithmetic folding already closed by issue 260.
- Addition, subtraction, multiplication; issues 382, 383.
- BigInt bitwise/exponentiation; issue 371.
- BigInt equality/comparison/builtins except where tests need arithmetic setup.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `docs/14-runtime-abi.md`
- `docs/language-reference/javascript-features.md`
- `current-state.md`

Do not touch:

- Parser BigInt literal syntax.
- Unrelated number arithmetic behavior.
- Addition, subtraction, multiplication helpers.

## Acceptance criteria

- [x] Node/iwasm differential fixtures cover dynamic BigInt div/rem with operands or results outside signed i64 for known locals/literal operands; branch/control-flow-assigned locals are split to issue 398 with evidence.
- [x] Existing signed-i64 slice fixtures from issue 260 continue to match Node.
- [x] Runtime linker structure tests cover BigIntDiv/BigIntRem runtime selection; no new public RuntimeFn variant was required.
- [x] Docs/current-state/issues state the new division/remainder boundary.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo test -p ts2wasm-cli --test m2_node_diff bigint_large_div_rem
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

- [x] updated: `docs/14-runtime-abi.md`
- [x] updated: `docs/language-reference/javascript-features.md`

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] issue 398 tracks branch/control-flow-assigned BigInt div/rem locals

## Notes

This is a focused split from issue 369, covering the validated division and remainder slice. Do not implement this by widening the signed-i64 conversion path. The compatibility target is the canonical heap BigInt limb representation.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `8c4976b4` issue-384: add cached-decimal bigint div rem slice

Validation result:

```text
PASS: cargo fmt --all --check
PASS: cargo test -p ts2wasm-cli --test m2_node_diff bigint_large_div_rem
PASS: cargo test -p ts2wasm-cli --test m2_node_diff bigint_runtime_mul_div_rem
PASS: mise run update-issue-index -- --check
PASS: mise run check issues
```

Close note (2026-05-01): issue 384 is closed for the validated known-BigInt operand slice. The remaining branch/control-flow-assigned local tracking gap is split to issue 398 rather than hidden under this close.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/384-multilimb-bigint-division-remainder.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
