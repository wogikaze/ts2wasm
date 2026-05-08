---
id: 1184
title: "Implement Classexpressionpropertymodifiers"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: [400]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1184.

## Summary

Triage classExpressionPropertyModifiers across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `classExpressionPropertyModifiers` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: classExpressionPropertyModifiers has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExpressionPropertyModifiers.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionPropertyModifiers.ts --detail
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
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command or this bucket is superseded by existing boundary issue 400
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence or existing issue 400 owns the boundary
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change or no child issue is needed

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionPropertyModifiers.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExpressionPropertyModifiers.ts
```

Not run:

- cargo gates; issue close only, no implementation changes

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; current behavior is covered by `issues/done/400-implement-ambient-declaration-erasure-boundary.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classExpressionPropertyModifiers.ts`

## Duplicate detection

- `issues/done/400-implement-ambient-declaration-erasure-boundary.md` owns the current boundary: ambient class element initializers produce source-spanned `UnsupportedTypeScriptSyntax` instead of creating runtime bindings.

## Smart triage

Fresh triage on 2026-05-06:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionPropertyModifiers.ts
```

Result:

```text
Smart triage class: triage-needed
Feature label: parser-syntax
Diagnostic: UnsupportedTypeScriptSyntax / unsupported-feature-boundary
Current error: issue-400: ambient class element initializers would affect runtime bindings at 133..134
```

Compiler dump evidence:

```text
tokens: ok
ast: UnsupportedTypeScriptSyntax at `declare [Symbol.toStringTag] = "uh";`
resolved: same issue-400 boundary
```

TypeScript oracle:

```text
TS1039: Initializers are not allowed in ambient contexts.
TS1031: 'export' modifier cannot appear on class elements of this kind.
```

The current compiler behavior matches the completed issue-400 boundary: class
element `declare` forms with initializers are not erased because doing so could
silently change runtime bindings. This generated bucket is closed without
creating a duplicate child issue.

## Completion evidence

Commits:

- local superseded-close commit for issue 1184

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionPropertyModifiers.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_diagcodes=UnsupportedTypeScriptSyntax:1, unsupported_features=parser-syntax:1
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionPropertyModifiers.ts
result: pass; current blocker matches issue-400 ambient class element initializer boundary
date: 2026-05-06
```

Remaining risks:

- The reference also contains an invalid `export` class element modifier; if semantic diagnostics are implemented later, TS1031 parity can be tracked separately.
