---
id: 1104
title: "Implement Cannotinvokenewonerrorexpression"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Triage cannotInvokeNewOnErrorExpression across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `cannotInvokeNewOnErrorExpression` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: cannotInvokeNewOnErrorExpression has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/cannotInvokeNewOnErrorExpression.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/cannotInvokeNewOnErrorExpression.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/cannotInvokeNewOnErrorExpression.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/cannotInvokeNewOnErrorExpression.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] Superseded by `issues/done/5150-report-empty-element-access-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/cannotInvokeNewOnErrorExpression.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage unknown unsupported: cannotInvokeNewOnErrorExpression

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `issue-5150`
- Path: `reference/typescript/tests/cases/compiler/cannotInvokeNewOnErrorExpression.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/cannotInvokeNewOnErrorExpression.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] issue-5150: empty element access `expr[]` requires an index expression
```

Source context:

```ts
namespace M
{
    class ClassA {}
}
var t = new M.ClassA[];
```

Evidence:

- Tokens succeed and show `new M.ClassA[]`.
- TypeScript oracle reports TS1011 for the empty element access expression.
- Completed issue `issues/done/5150-report-empty-element-access-diagnostics.md`
  already owns and implements the targeted empty element access diagnostic.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...` pending

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cannotInvokeNewOnErrorExpression.ts
result: pass; reproduced completed issue-5150 empty element access diagnostic
date: 2026-05-06
```

Remaining risks:

- none
