---
id: 1102
title: "Implement Callbackargsdifferbyoptionality"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5200]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Triage callbackArgsDifferByOptionality across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `callbackArgsDifferByOptionality` with diagnostics: arguments-object. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: callbackArgsDifferByOptionality has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callbackArgsDifferByOptionality.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/callbackArgsDifferByOptionality.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/callbackArgsDifferByOptionality.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callbackArgsDifferByOptionality.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5200-validate-top-level-function-overload-implementations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/callbackArgsDifferByOptionality.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage duplicate function: callbackArgsDifferByOptionality

- Issue class: `triage-needed`
- Feature label: `duplicate-function`
- Diagnostic: `DuplicateFunction` / `compiler-diagnostic`
- Path: `reference/typescript/tests/cases/compiler/callbackArgsDifferByOptionality.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callbackArgsDifferByOptionality.ts
```

Failure:

```text
error: [DuplicateFunction] duplicate function definition: `x3` at 85..93
```

Source context:

```ts
function x3(callback: (x?: 'hi') => number);
function x3(callback: (x: string) => number);
function x3(callback: (x: any) => number) {
    cb();
}
```

Evidence:

- Tokens and AST succeed.
- AST contains two bodyless `Function x3` overload signatures followed by one
  implemented `Function x3`.
- TypeScript oracle accepts the overload group and reports TS2304 for unresolved
  `cb` in the implementation body.
- Duplicate-function candidates
  `issues/open/2043-implement-duplicateIdentifierRelatedSpans-duplicate-function.md`,
  `issues/open/2600-implement-getAndSetNotIdenticalType-duplicate-function.md`,
  and `issues/open/4258-implement-staticVisibility-duplicate-function.md` are
  no-match buckets for other duplicate-function windows.
- Existing child issue
  `issues/open/5200-validate-top-level-function-overload-implementations.md`
  owns the top-level overload implementation grouping blocker and has been
  expanded with this reference path.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...` pending

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/callbackArgsDifferByOptionality.ts
result: pass; reproduced DuplicateFunction for valid top-level overload signatures before unresolved `cb`
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

