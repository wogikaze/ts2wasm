---
id: 1058
title: "Implement Binderbinaryexpressionstress"
type: spike
area: reference/triage
class: superseded
priority: P2
depends_on: [5173]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage binderBinaryExpressionStress across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `binderBinaryExpressionStress` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: binderBinaryExpressionStress has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/binderBinaryExpressionStress.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/binderBinaryExpressionStress.ts --detail
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

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/binderBinaryExpressionStress.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/binderBinaryExpressionStress.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] added: `issues/done/5173-avoid-stack-overflow-on-deep-binary-expressions.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/binderBinaryExpressionStress.ts`

## Duplicate detection

- No open issue was found for `resolve_builtins` stack overflow on deep binary expression chains.

## Smart triage

Generated on 2026-05-06.

- Path: `reference/typescript/tests/cases/compiler/binderBinaryExpressionStress.ts`
- Diagnostic: `Unknown` / `unknown`
- Failure: `thread 'main' has overflowed its stack`
- Visible symbols before failure: `caps`
- Compiler evidence: tokens succeed; resolved dump reaches `resolve_builtins` and aborts with stack overflow.
- TypeScript oracle: timed out for this large file during triage.
- Superseding child: `issues/done/5173-avoid-stack-overflow-on-deep-binary-expressions.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binderBinaryExpressionStress.ts
result: pass; current blocker identified as builtin resolver stack overflow on deep binary expressions, split to issue 5173
date: 2026-05-06
```

Remaining risks:

- Later semantic or emit gaps may appear after issue 5173 prevents the abort.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

