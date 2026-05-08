---
id: 1095
title: "Implement Callonclass"
type: spike
area: frontend/resolver
class: blocked
priority: P1
depends_on: [5197]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Triage callOnClass across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `callOnClass` with diagnostics: function-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: callOnClass has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOnClass.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/callOnClass.ts --detail
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/callOnClass.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOnClass.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5197-report-class-called-without-new.md`

## Notes

Superseded by `issues/open/5197-report-class-called-without-new.md`. Fresh
triage shows the source parses to `ClassDecl C` and `Call(Ident C)`, but name
resolution reports the broad `issue-5011` class-value diagnostic instead of the
TypeScript oracle's class-not-callable TS2348 diagnostic.

## Affected test files

- `reference/typescript/tests/cases/compiler/callOnClass.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated on 2026-05-06:

- command: `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/callOnClass.ts`
- diagnostic: `UnsupportedSyntax`, `issue-5011: class C cannot be used as a value`
- AST: `ClassDecl C`; `Let c = Call(Ident C, [])`
- TypeScript oracle: TS2348 `Value of type 'typeof C' is not callable. Did you mean to include 'new'?`
- follow-up: `issues/open/5197-report-class-called-without-new.md`

## Completion evidence

Closed as a generated triage bucket. The actionable class-call diagnostic gap is
tracked by `issues/open/5197-report-class-called-without-new.md`.

Commits:

- this split commit

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/callOnClass.ts
result: fail with issue-5011 class-value diagnostic; TypeScript reports TS2348 class-not-callable
date: 2026-05-06
```

Remaining risks:

- Follow-up issue 5197 still needs implementation.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

