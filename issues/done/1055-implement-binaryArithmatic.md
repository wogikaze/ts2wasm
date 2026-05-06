---
id: 1055
title: "Implement Binaryarithmatic"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5170]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage binaryArithmatic across 4 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 4 cases fail in directory `binaryArithmatic` with diagnostics: operator. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: binaryArithmatic has 4 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/binaryArithmatic3.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/binaryArithmatic3.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 8
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/binaryArithmatic3.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/binaryArithmatic3.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] added: `issues/open/5170-support-bitwise-or-binary-lowering.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/binaryArithmatic3.ts`
- `reference/typescript/tests/cases/compiler/binaryArithmatic4.ts`
- `reference/typescript/tests/cases/compiler/binaryArithmatic2.ts`
- `reference/typescript/tests/cases/compiler/binaryArithmatic1.ts`

## Duplicate detection

- `issues/done/5153-erase-union-types-in-as-assertions.md` is not a match: it owns TypeScript `as A | B` union erasure and explicitly excludes runtime bitwise-or implementation.
- BigInt bitwise OR issues/code paths are not a match: the affected files use ordinary `number`, `null`, and `undefined` operands.

## Smart triage

Generated on 2026-05-06.

- Paths:
  - `reference/typescript/tests/cases/compiler/binaryArithmatic1.ts`
  - `reference/typescript/tests/cases/compiler/binaryArithmatic2.ts`
  - `reference/typescript/tests/cases/compiler/binaryArithmatic3.ts`
  - `reference/typescript/tests/cases/compiler/binaryArithmatic4.ts`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Failure: `binary operator BitwiseOr not yet supported`
- Source contexts:
  - `var v = 4 | null;`
  - `var v = 4 | undefined;`
  - `var v = undefined | undefined;`
  - `var v = null | null;`
- Visible symbols before failure: `v`, `null`, `undefined`
- Compiler evidence: lexer and parser succeed; AST contains `Binary { op: BitwiseOr }`; lowering rejects `BinaryOp::BitwiseOr`.
- TypeScript oracle: accepts the ES2015 reference cases; ordinary JavaScript bitwise OR applies ToInt32-style coercion to null and undefined operands.
- Superseding child: `issues/open/5170-support-bitwise-or-binary-lowering.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binaryArithmatic1.ts
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binaryArithmatic2.ts
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binaryArithmatic3.ts
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binaryArithmatic4.ts
result: pass; current blocker identified as ordinary BitwiseOr lowering, split to issue 5170
date: 2026-05-06
```

Remaining risks:

- TypeScript may expose later reference-specific diagnostics after issue 5170 advances these files past the current lowering blocker.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

