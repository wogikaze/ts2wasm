---
id: 3609
title: "Implement Nullablefunctionerror"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: [5163]
created: 2026-05-01
updated: 2026-05-08
status: done
---

## Summary

Triage nullableFunctionError across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `nullableFunctionError` with diagnostics: call-expression. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: nullableFunctionError has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nullableFunctionError.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nullableFunctionError.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

Current close decision: superseded by
`issues/open/5163-lower-nested-call-expression-callees.md`.

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nullableFunctionError.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nullableFunctionError.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Close note, 2026-05-08:

- Fresh triage shows tokens and AST are ok for `null()`, `undefined()`, and
  `f()` where `f: null | undefined`.
- The pipeline reaches lowering and reports
  `UnsupportedSyntax: only identifier calls are supported in expression context
  at 47..53` for the `Call(Null)` expression.
- TypeScript oracle reports TS2721 for `null()`, TS2722 for `undefined()`, and
  TS2723 for `f()`, but those callability diagnostics remain unreachable until
  non-identifier callee lowering/classification advances.
- Existing issue 5163 already owns the generic non-identifier callee lowering
  boundary, so this generated bucket is folded there rather than split into a
  new child.

## Affected test files

- `reference/typescript/tests/cases/compiler/nullableFunctionError.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh smart triage on 2026-05-08:

```text
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Feature label: call-expression
Path: reference/typescript/tests/cases/compiler/nullableFunctionError.ts
Failure location: only identifier calls are supported in expression context at 47..53
Source context: null(); undefined(); let f: null | undefined; f();
AST: Expr Call(Null), Expr Call(Undefined), Let f, Expr Call(Ident f)
TypeScript oracle: TS2721, TS2722, TS2723
```

Focused coverage on 2026-05-08:

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending local commit`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nullableFunctionError.ts
result: pass; reproduced current non-identifier callee lowering blocker for `null()`
date: 2026-05-08

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nullableFunctionError.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1
date: 2026-05-08
```

Remaining risks:

- Issue 5163 still needs implementation. After literal/nullish callee
  classification advances, this file should expose TS2721/TS2722/TS2723-style
  strict-null callability diagnostics.
