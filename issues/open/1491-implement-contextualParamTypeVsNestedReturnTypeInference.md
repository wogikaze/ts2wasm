---
id: 1491
title: "Implement Contextualparamtypevsnestedreturntypeinference"
type: spike
area: frontend/semantics
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
> Evidence: Empty completion evidence. No feat/fix commit for #1491.

## Summary

Triage contextualParamTypeVsNestedReturnTypeInference across 4 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 4 cases fail in directory `contextualParamTypeVsNestedReturnTypeInference` with diagnostics: type-system. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualParamTypeVsNestedReturnTypeInference has 4 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualParamTypeVsNestedReturnTypeInference1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualParamTypeVsNestedReturnTypeInference1.ts --detail
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
mise run reference-coverage -- tsc --limit 8
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualParamTypeVsNestedReturnTypeInference1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualParamTypeVsNestedReturnTypeInference1.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] split/superseded: `issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`, `issues/open/5369-parse-call-expression-type-arguments-in-class-heritage.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualParamTypeVsNestedReturnTypeInference1.ts`
- `reference/typescript/tests/cases/compiler/contextualParamTypeVsNestedReturnTypeInference4.ts`
- `reference/typescript/tests/cases/compiler/contextualParamTypeVsNestedReturnTypeInference2.ts`
- `reference/typescript/tests/cases/compiler/contextualParamTypeVsNestedReturnTypeInference3.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage on 2026-05-07 splits this generated type-system bucket into two
current blockers.

Case 1 is blocked by ambient value declaration name resolution:

```text
contextualParamTypeVsNestedReturnTypeInference1.ts
UnresolvedName: unresolved name: `Foo` at 483..486
```

Source context:

```ts
declare const Foo: Tag<{
  fn: (a: string) => unknown;
}>;

layerEffect(
  Foo,
```

Smart triage lists `Foo` as a visible binding before failure. This is the same
declaration-only ambient value visibility gap already owned by
`issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`.

Cases 2, 3, and 4 are blocked by a parser boundary in class heritage:

```text
UnsupportedSyntax: expected LeftBrace, got Some(Comma)
```

Representative source:

```ts
class Foo extends Tag("Foo")<
  Foo,
  {
    fn: (a: string) => Effect<void>;
  }
>() {}
```

TypeScript accepts all four files with no diagnostics. The call-expression
heritage type-argument blocker was split to
`issues/open/5369-parse-call-expression-type-arguments-in-class-heritage.md`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...` (filled by commit that moves this issue)

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter contextualParamTypeVsNestedReturnTypeInference --detail --no-dashboard-data
result: pass; executed=4, unsupported=4, UnsupportedSyntax=3, UnresolvedName=1
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualParamTypeVsNestedReturnTypeInference1.ts
result: pass; ambient `declare const Foo` name-resolution blocker mapped to issue 5161
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualParamTypeVsNestedReturnTypeInference2.ts
result: pass; class heritage call-expression type-argument parser blocker split to issue 5369
date: 2026-05-07
```

Remaining risks:

- The intended contextual parameter vs nested return type inference behavior
  remains hidden until issue 5161 and issue 5369 advance these files past their
  current parser/resolver boundaries.
