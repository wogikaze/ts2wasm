---
id: 1097
title: "Implement Calloverloadviaelementaccessexpression"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5198]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Triage callOverloadViaElementAccessExpression across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `callOverloadViaElementAccessExpression` with diagnostics: call-expression. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: callOverloadViaElementAccessExpression has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOverloadViaElementAccessExpression.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/callOverloadViaElementAccessExpression.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/callOverloadViaElementAccessExpression.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOverloadViaElementAccessExpression.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5198-support-class-method-overload-signatures-for-element-access-calls.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/callOverloadViaElementAccessExpression.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage duplicate function: callOverloadViaElementAccessExpression

- Issue class: `triage-needed`
- Feature label: `duplicate-function`
- Diagnostic: `DuplicateFunction` / `compiler-diagnostic`
- Path: `reference/typescript/tests/cases/compiler/callOverloadViaElementAccessExpression.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOverloadViaElementAccessExpression.ts
```

Failure:

```text
error: [DuplicateFunction] duplicate method definition: `C.foo`
```

Source context:

```ts
class C {
    foo(x: number): number;
    foo(x: string): string;
    foo(x: any): any {
        return null;
    }
}
var c = new C();
var r: string = c["foo"](1);
var r2: number = c["foo"]("");
```

Evidence:

- Tokens and AST succeed.
- AST contains three `foo` class members; the first two are bodyless overload
  signatures and the third has the implementation body.
- AST contains element-access call callees for `c["foo"](1)` and
  `c["foo"]("")`.
- TypeScript oracle accepts the overload declarations and reports TS2322 for
  the two intentionally mismatched assignments.
- Duplicate candidates `issues/open/2043-implement-duplicateIdentifierRelatedSpans-duplicate-function.md`,
  `issues/open/2600-implement-getAndSetNotIdenticalType-duplicate-function.md`,
  and `issues/open/4258-implement-staticVisibility-duplicate-function.md` are
  no-match buckets for other duplicate-function windows.
- Child issue `issues/open/5198-support-class-method-overload-signatures-for-element-access-calls.md`
  owns the exact blocker.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...` pending

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/callOverloadViaElementAccessExpression.ts
result: pass; reproduced DuplicateFunction for class method overload declarations and captured TypeScript oracle diagnostics
date: 2026-05-06
```

Remaining risks:

- none
