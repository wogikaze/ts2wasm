---
id: 1422
title: "Implement Conditionaltypeassignabilitywhendeferred"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [432]
blocks: [5302]
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1422.

## Summary

Triage conditionalTypeAssignabilityWhenDeferred across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case in directory
`conditionalTypeAssignabilityWhenDeferred`. Fresh triage on 2026-05-07 shows
the current first blocker is not conditional type semantics; parsing stops on
the fractional numeric literal token sequence in `Math.random() > 0.5`.

Problem: conditionalTypeAssignabilityWhenDeferred has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conditionalTypeAssignabilityWhenDeferred.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conditionalTypeAssignabilityWhenDeferred.ts --detail
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
- [x] Child issue 5302 contains an exact `mise run reference-triage -- ...` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conditionalTypeAssignabilityWhenDeferred.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conditionalTypeAssignabilityWhenDeferred.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] split to issue 5302

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/conditionalTypeAssignabilityWhenDeferred.ts`

## Duplicate detection

- Issue 5191 is related but not a duplicate: it owns leading-decimal `.5`
  numeric literals. This bucket fails on `0.5`, tokenized as integer, dot, and
  fractional part.
- Issue 5296 is related but not a duplicate: it owns double-dot numeric member
  access such as `2..toFixed(0)`.
- Issue 680 shows the same `Math.random() > 0.5` parser blocker inside another
  generated bucket, but is not a small implementation-ready owner.

## Smart triage

Generated on 2026-05-07:

```text
Feature label: type-system
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Path: reference/typescript/tests/cases/compiler/conditionalTypeAssignabilityWhenDeferred.ts
Failure: expected member property name, got Number(5) at 946..947
line 35, column 22
Source context:
33 | function f<T>(t: T) {
34 |   var x: T | null = Math.random() > 0.5 ? null : t;
35 |   onlyNullablePlease(x); // should work
36 |   onlyNullablePlease2(x); // should work
Visible symbols:
- binding x
```

Compiler evidence:

```text
tokens: ok; source contains Math.random() > 0.5 ? null : t
ast: fails before resolved IR with expected member property name, got Number(5)
TypeScript oracle: parses the file and reports later type diagnostics, not a numeric literal parse error
```

Coverage result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/conditionalTypeAssignabilityWhenDeferred.ts --detail --no-dashboard-data
result: executed=1, build_pass=0, unsupported=1, blocked=0, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=type-system:1
date: 2026-05-07
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split to `issues/open/5302-parse-fractional-number-literals-in-expressions.md`.

Validation result:

```text
command: python scripts/manager.py update-issue-index && python scripts/manager.py update-issue-index --check && python scripts/manager.py check-issue-health && python scripts/manager.py check-issue-readiness -- --fail-ready-below 80 && git diff --check
result: pass
date: 2026-05-07
```

Remaining risks:

- Child issue 5302 still needs implementation.
