---
id: 1147
title: "Implement Checkingobjectdefinepropertyonfunctionnonexistentpropertynocrash"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5236]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1147.

## Summary

Triage checkingObjectDefinePropertyOnFunctionNonexistentPropertyNoCrash across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `checkingObjectDefinePropertyOnFunctionNonexistentPropertyNoCrash` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: checkingObjectDefinePropertyOnFunctionNonexistentPropertyNoCrash has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkingObjectDefinePropertyOnFunctionNonexistentPropertyNoCrash1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkingObjectDefinePropertyOnFunctionNonexistentPropertyNoCrash1.ts --detail
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
- [x] At least one child issue contains an exact `reference-triage` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkingObjectDefinePropertyOnFunctionNonexistentPropertyNoCrash1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkingObjectDefinePropertyOnFunctionNonexistentPropertyNoCrash1.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5236-w1-implement-wasi-args-and-environment-variable-lowering.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/checkingObjectDefinePropertyOnFunctionNonexistentPropertyNoCrash1.ts`

## Duplicate detection

Fresh duplicate scan found related runtime-subset and closure buckets, but no
exact implementation-ready issue for nested function expressions with rest or
default parameters:

- `issues/open/445-implement-runtime-subset.md` is a broad generated
  runtime-subset bucket and includes the same diagnostic family, but is not an
  implementation-ready child.
- `issues/done/062e-function-closures.md` implemented the base closure slice
  and intentionally left unsupported closure forms on issue-linked diagnostics.
- `issues/open/212-implement-rest-parameter-argument-collection.md` and
  `issues/done/040-implement-default-parameters.md` cover ordinary function
  rest parameter lowering, not the nested closure guard.

Split result:

- `issues/open/5236-w1-implement-wasi-args-and-environment-variable-lowering.md`

## Smart triage

Fresh triage shows the generated import-export bucket is stale for this
representative. `export function`, the object literal descriptor, and
`Object.defineProperty(...)` parse into AST. The current blocker is lowering
the nested function expression `function (...args) { }`.

### Smart triage: checkingObjectDefinePropertyOnFunctionNonexistentPropertyNoCrash1

- Issue class: `triage-needed`
- Feature label: `runtime-subset`
- Diagnostic: `UnsupportedRuntimeSubset` / `unsupported-feature-boundary`
- Current compiler message: `issue-062e: nested function `` closure parameters with defaults or rest are not supported in this slice`
- Path: `reference/typescript/tests/cases/compiler/checkingObjectDefinePropertyOnFunctionNonexistentPropertyNoCrash1.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkingObjectDefinePropertyOnFunctionNonexistentPropertyNoCrash1.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkingObjectDefinePropertyOnFunctionNonexistentPropertyNoCrash1.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=object-literal:1
```

Source context:

```ts
export function test(fn) {
  const composed = function (...args) { }

  Object.defineProperty(composed, 'name', {
    value: composed.fn + '_test'
  })

  return composed
}
```

Compiler evidence:

```text
tokens: ok; includes export function, nested Function, DotDotDot args, Object.defineProperty, and descriptor object literal
ast: ok; Let composed = FunctionExpr(params=[args is_rest=true], body=[])
resolved/lowered: UnsupportedRuntimeSubset issue-062e nested function closure parameters with defaults or rest
TypeScript oracle: TS2339 Property 'fn' does not exist on type '(...args: any[]) => void'
```

Split result:

- `issues/open/5236-w1-implement-wasi-args-and-environment-variable-lowering.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkingObjectDefinePropertyOnFunctionNonexistentPropertyNoCrash1.ts --detail --no-dashboard-data
result: pass; reproduced current unsupported bucket
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkingObjectDefinePropertyOnFunctionNonexistentPropertyNoCrash1.ts
result: pass; reproduced issue-062e nested function rest-parameter closure boundary and split to issue 5236
date: 2026-05-06
```

Remaining risks:

- Implementing issue 5236 may expose the intended TS2339 property diagnostic, `Object.defineProperty` metadata behavior, or function metadata limitations as the next blocker.
