---
id: 1433
title: "Implement Conflictmarkertrivia Parser Syntax"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1433.

## Summary

Triage conflictMarkerTrivia-parser-syntax across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory
`conflictMarkerTrivia-parser-syntax` with diagnostics: parser-syntax. Fresh
triage on 2026-05-07 shows the concrete blocker is the same missing merge
conflict marker diagnostic tracked by issue 5305.

Problem: conflictMarkerTrivia-parser-syntax has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conflictMarkerTrivia1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conflictMarkerTrivia1.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with existing issue 5305
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

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
- [x] Superseding issue 5305 contains exact conflict-marker diagnostic acceptance
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Coverage names the exact reference path and current stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conflictMarkerTrivia1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conflictMarkerTrivia1.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; superseded by `issues/open/5305-report-merge-conflict-marker-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/conflictMarkerTrivia1.ts`

## Duplicate detection

- Superseded by `issues/open/5305-report-merge-conflict-marker-diagnostics.md`.
  The current first blocker is the same merge conflict marker diagnostic gap,
  with regular conflict markers in a class body.
- Generic parser-syntax buckets are no-match because this bucket needs a
  specific source diagnostic for conflict marker lines, not broad parser
  extension work.

## Smart triage

Generated on 2026-05-07:

```text
Feature label: parser-syntax
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Path: reference/typescript/tests/cases/compiler/conflictMarkerTrivia1.ts
Failure: expected property name, got LeftShift at 33..35
line: 3, column: 5
Visible symbols before failure:
- class C
```

Source context:

```text
2 | class C {
3 | <<<<<<< HEAD
4 |     v = 1;
5 | =======
6 |     v = 2;
```

Compiler evidence:

```text
tokens: ok; markers tokenize as LeftShift/StrictEqual/UnsignedRightShift groups
ast: false; expected property name, got LeftShift
resolved: false; same parser diagnostic
```

TypeScript oracle evidence:

```text
TypeScript reports TS1185 "Merge conflict marker encountered" at lines 3, 5, and 7.
TypeScript AST still contains ClassDeclaration C and PropertyDeclaration v = 1.
```

Coverage result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/conflictMarkerTrivia1.ts --detail --no-dashboard-data
result: executed=1, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=unknown-unsupported:1, build_pass=0, semantic_pass=0, blocked=0
date: 2026-05-07
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Closed as superseded by issue 5305; no child issue created.

Validation result:

```text
command: python scripts/manager.py update-issue-index && python scripts/manager.py update-issue-index --check && python scripts/manager.py check-issue-health && python scripts/manager.py check-issue-readiness -- --fail-ready-below 80 && git diff --check
result: pass
date: 2026-05-07
```

Remaining risks:

- none
