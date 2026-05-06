---
id: 1105
title: "Implement Cannotinvokenewonindexexpression"
type: spike
area: frontend/resolver
class: blocked
priority: P1
depends_on: [5203]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Triage cannotInvokeNewOnIndexExpression across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `cannotInvokeNewOnIndexExpression` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: cannotInvokeNewOnIndexExpression has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/cannotInvokeNewOnIndexExpression.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/cannotInvokeNewOnIndexExpression.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/cannotInvokeNewOnIndexExpression.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/cannotInvokeNewOnIndexExpression.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/done/5203-report-indexed-new-type-only-callee-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/cannotInvokeNewOnIndexExpression.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage class: cannotInvokeNewOnIndexExpression

- Issue class: `triage-needed`
- Feature label: `class`
- Diagnostic: `UnsupportedSyntax` / `issue-062`
- Path: `reference/typescript/tests/cases/compiler/cannotInvokeNewOnIndexExpression.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/cannotInvokeNewOnIndexExpression.ts
```

Failure:

```text
error: [UnsupportedSyntax] issue-062: new requires a class name identifier at 37..47
```

Source context:

```ts
var test: any[] = new any[1];
```

Evidence:

- Tokens and AST succeed.
- AST contains `New { expr: Index { object: Ident any, index: Number 1 } }`.
- TypeScript oracle reports TS2693: `'any' only refers to a type, but is being
  used as a value here.`
- Existing class buckets are no-match owners because this blocker is the
  indexed `new` callee hiding a type-only value-use diagnostic.
- Child issue
  `issues/done/5203-report-indexed-new-type-only-callee-diagnostics.md` owns
  the implementation slice.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...` pending

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cannotInvokeNewOnIndexExpression.ts
result: pass; reproduced issue-062 generic new callee guard for indexed type-only callee
date: 2026-05-06
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

