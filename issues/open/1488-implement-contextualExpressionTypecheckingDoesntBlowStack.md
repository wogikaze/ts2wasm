---
id: 1488
title: "Implement Contextualexpressiontypecheckingdoesntblowstack"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1488.

## Summary

Triage contextualExpressionTypecheckingDoesntBlowStack across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualExpressionTypecheckingDoesntBlowStack` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualExpressionTypecheckingDoesntBlowStack has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualExpressionTypecheckingDoesntBlowStack.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualExpressionTypecheckingDoesntBlowStack.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualExpressionTypecheckingDoesntBlowStack.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualExpressionTypecheckingDoesntBlowStack.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] split to `issues/open/5367-support-named-default-class-export-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualExpressionTypecheckingDoesntBlowStack.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage on 2026-05-07 shows this generated bucket is blocked before the
contextual typing stress case by a named default class export boundary.

Current diagnostic:

```text
error: [UnsupportedModule] issue-055: unsupported default class export; module resolution and loading are not implemented at 191..197
```

Source context:

```ts
export default class Operation {
    validateParameters(parameterValues: any) : IValidationError[] | null {
```

Visible symbols before failure: none.

TypeScript oracle: accepted with no diagnostics. AST top level includes
`InterfaceDeclaration` followed by a named default-exported `ClassDeclaration`
for `Operation`.

This bucket was split to
`issues/open/5367-support-named-default-class-export-declarations.md`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...` (filled by commit that moves this issue)

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualExpressionTypecheckingDoesntBlowStack.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, diagnostic UnsupportedModule
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualExpressionTypecheckingDoesntBlowStack.ts
result: pass; reproduced issue-055 default class export boundary and split named default class export work to issue 5367
date: 2026-05-07
```

Remaining risks:

- The original contextual typing stack behavior remains hidden until named
  default class exports advance past the issue-055 module-syntax boundary.
