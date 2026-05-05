---
id: 1045
title: "Implement Bettererrorforaccidentalcall"
type: spike
area: frontend/syntax
class: superseded
priority: P2
depends_on: [5163]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage betterErrorForAccidentalCall across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `betterErrorForAccidentalCall` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: betterErrorForAccidentalCall has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/betterErrorForAccidentalCall.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/betterErrorForAccidentalCall.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/betterErrorForAccidentalCall.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/betterErrorForAccidentalCall.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created/updated: `issues/open/5163-lower-nested-call-expression-callees.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/betterErrorForAccidentalCall.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated on 2026-05-06.

- Path: `reference/typescript/tests/cases/compiler/betterErrorForAccidentalCall.ts`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Failure: `only identifier calls are supported in expression context at 52..70`
- Source context: `foo()(1 as number).toString();`
- TypeScript AST path: `ExpressionStatement -> CallExpression -> PropertyAccessExpression -> CallExpression -> CallExpression -> Identifier(foo)`
- Split child: `issues/open/5163-lower-nested-call-expression-callees.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/betterErrorForAccidentalCall.ts
result: pass; current blocker identified as lowering rejection of non-identifier call callees, split to issue 5163
date: 2026-05-06
```

Remaining risks:

- TypeScript TS2349 call-signature diagnostics remain future work after issue 5163 advances past the current generic lowerer unsupported diagnostic.
