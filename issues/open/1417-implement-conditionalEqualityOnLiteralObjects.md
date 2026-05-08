---
id: 1417
title: "Implement Conditionalequalityonliteralobjects"
type: spike
area: frontend/semantics
class: done
priority: P1
depends_on: [5002]
blocks: [5301]
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1417.

## Summary

Triage conditionalEqualityOnLiteralObjects across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case in directory
`conditionalEqualityOnLiteralObjects`. Fresh triage on 2026-05-07 shows the
compiler now builds the file successfully; the remaining gap is a hidden
TypeScript oracle diagnostic for object/array literal reference comparisons.

Problem: conditionalEqualityOnLiteralObjects has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conditionalEqualityOnLiteralObjects.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conditionalEqualityOnLiteralObjects.ts --detail
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
- [x] Child issue 5301 contains an exact `mise run reference-triage -- ...` command
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conditionalEqualityOnLiteralObjects.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conditionalEqualityOnLiteralObjects.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] split to issue 5301

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/conditionalEqualityOnLiteralObjects.ts`

## Duplicate detection

- No existing open/done issue matched TS2839 object/array literal reference
  comparison diagnostics for this reference path.
- The numeric issue id 2839 is unrelated; it tracks
  `inferringReturnTypeFromConstructSignatureGeneric`.

## Smart triage

Generated on 2026-05-07:

```text
Feature label: build-pass
Diagnostic: BuildPass / pass
Path: reference/typescript/tests/cases/compiler/conditionalEqualityOnLiteralObjects.ts
Failure: ts2wasm build succeeded
Source overview: 596 bytes, 55 lines
Visible symbols:
- binding a = { a: 1 }
- binding b = [1]
```

Compiler evidence:

```text
tokens: ok
ast: ok; If condition Binary(Object, StrictEqual, Object), Binary(Array, StrictEqual, Array), and local-vs-literal variants
resolved: ok; strict and abstract equality/inequality expressions are preserved
TypeScript oracle: TS2839 at 24 object/array literal comparison conditions
```

Representative TypeScript oracle diagnostic:

```text
TS2839: This condition will always return 'false' since JavaScript compares objects by reference, not value.
line 5, character 5, span 57..78
```

Coverage result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/conditionalEqualityOnLiteralObjects.ts --detail --no-dashboard-data
result: executed=1, build_pass=1, semantic_pass=0, unsupported=0, blocked=0, semantic_enabled=0
date: 2026-05-07
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split to `issues/done/5301-report-literal-reference-comparison-diagnostics.md`.

Validation result:

```text
command: python scripts/manager.py update-issue-index && python scripts/manager.py update-issue-index --check && python scripts/manager.py check-issue-health && python scripts/manager.py check-issue-readiness -- --fail-ready-below 80 && git diff --check
result: pass
date: 2026-05-07
```

Remaining risks:

- Child issue 5301 still needs implementation.
