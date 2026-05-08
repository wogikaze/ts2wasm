---
id: 1487
title: "Implement Contextualcomputednonbindablepropertytype"
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
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1487.

## Summary

Triage contextualComputedNonBindablePropertyType across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualComputedNonBindablePropertyType` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualComputedNonBindablePropertyType has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualComputedNonBindablePropertyType.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualComputedNonBindablePropertyType.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualComputedNonBindablePropertyType.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualComputedNonBindablePropertyType.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5209-support-class-instance-method-receiver-calls.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualComputedNonBindablePropertyType.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage on 2026-05-07 shows this generated bucket is blocked by object
literal computed property expression parsing.

Current diagnostic:

```text
error: [UnsupportedSyntax] expected Dot, got Some(LeftParen) at 202..203
```

Source context:

```ts
forceMatch({
  [testD()]: "d",
});
```

TypeScript oracle: accepted with no diagnostics. AST path:
`ObjectLiteralExpression -> PropertyAssignment -> ComputedPropertyName -> CallExpression`.

This bucket was superseded by `issues/open/5209-support-class-instance-method-receiver-calls.md`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...` (filled by commit that moves this issue)

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualComputedNonBindablePropertyType.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, diagnostic UnsupportedSyntax
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualComputedNonBindablePropertyType.ts
result: pass; reproduced `[testD()]` computed object key parser blocker and mapped it to issue 5209
date: 2026-05-07
```

Remaining risks:

- Mapped type/contextual typing behavior remains hidden until issue 5209
  advances past the computed object literal key parser boundary.
