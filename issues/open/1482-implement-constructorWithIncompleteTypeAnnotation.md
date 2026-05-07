---
id: 1482
title: "Implement Constructorwithincompletetypeannotation"
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
> Evidence: Empty completion evidence. No feat/fix commit for #1482.

## Summary

Triage constructorWithIncompleteTypeAnnotation across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `constructorWithIncompleteTypeAnnotation` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: constructorWithIncompleteTypeAnnotation has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constructorWithIncompleteTypeAnnotation.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constructorWithIncompleteTypeAnnotation.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constructorWithIncompleteTypeAnnotation.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constructorWithIncompleteTypeAnnotation.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] split to `issues/open/5364-report-unterminated-string-literal-at-raw-newline.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constructorWithIncompleteTypeAnnotation.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage on 2026-05-07 shows this generated bucket is stopped before the
constructor declaration. The lexer rejects a raw newline inside a string literal
in `retValue = bfs.OPERATOR ' );`, while TypeScript reports TS1002
`Unterminated string literal`.

Current diagnostic:

```text
error: [UnsupportedSyntax] raw newline in string literal is not allowed at 984..985
```

Source context:

```text
44 |                 retValue = bfs.OPERATOR ' );
45 |                 if (retValue != 0) {
46 |
47 |                     return 1;
```

This bucket was split to `issues/open/5364-report-unterminated-string-literal-at-raw-newline.md`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...` (filled by commit that moves this issue)

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorWithIncompleteTypeAnnotation.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, diagnostic UnsupportedSyntax
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorWithIncompleteTypeAnnotation.ts
result: pass; reproduced raw-newline string literal blocker and split child issue 5364
date: 2026-05-07
```

Remaining risks:

- Later malformed constructs in this intentionally broken reference file remain
  hidden until issue 5364 advances past the first raw-newline string blocker.
