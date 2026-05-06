---
id: 5221
title: "Support bitwise AND/XOR binary lowering"
type: feature
area: ir/lowering
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

After bitwise compound assignment syntax is parsed, the
`bitwiseCompoundAssignmentOperators.ts` reference case advances to IR lowering
and stops at unsupported `BitwiseXor`.

## Problem

Problem: ordinary bitwise XOR and AND binary expressions can now be produced by
the frontend, but lowering rejects them before TypeScript operand diagnostics or
runtime semantics can be compared.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bitwiseCompoundAssignmentOperators.ts
```

Current diagnostic after issue 5178 syntax support:

```text
UnsupportedSyntax: binary operator BitwiseXor not yet supported
```

The AST now contains `Assign` statements whose values are `Binary { op:
BitwiseXor | BitwiseAnd | BitwiseOr, ... }`.

## Desired final state

Ordinary bitwise XOR and AND expressions lower or report source-backed semantic
diagnostics in the same phase as existing bitwise OR work.

## Scope

In scope:

- [ ] Define the ordinary number lowering behavior for `BitwiseXor` and `BitwiseAnd`.
- [ ] Add focused lowering/runtime or diagnostic coverage for `a ^ b` and `c & d`.
- [ ] Re-run the representative bitwise compound assignment triage.

Out of scope:

- BigInt bitwise semantics outside the existing BigInt bitwise issues.
- Parser tokenization for `^=`, `&=`, and `|=`; issue 5178 owns that syntax slice.
- TypeScript boolean operand diagnostic parity for every combination in the reference file.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/frontend/src/lexer.rs`
- `crates/frontend/src/parser/`

## Acceptance criteria

- [ ] `BinaryOp::BitwiseXor` no longer reports `binary operator BitwiseXor not yet supported`.
- [ ] `BinaryOp::BitwiseAnd` is covered with the same ordinary-number policy.
- [ ] The representative `bitwiseCompoundAssignmentOperators.ts` triage advances past the current lowering diagnostic or records the next narrower blocker.
- [ ] Regression coverage proves `^` and `&` behavior or source-backed diagnostics.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli bitwise
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bitwiseCompoundAssignmentOperators.ts
python scripts/manager.py update-issue-index --check
python scripts/manager.py check issues
```

Impacted commands:

```sh
python scripts/manager.py check issue-readiness -- --fail-ready-below 80
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split while closing issue 5178. Issue 5170 already owns `BitwiseOr`; this issue
tracks the newly exposed AND/XOR lowering gap.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
