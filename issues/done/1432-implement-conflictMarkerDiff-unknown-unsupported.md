---
id: 1432
title: "Implement Conflictmarkerdiff Unknown Unsupported"
type: spike
area: frontend/syntax
class: done
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1432.

## Summary

Triage conflictMarkerDiff-unknown-unsupported across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory
`conflictMarkerDiff-unknown-unsupported` with diagnostics:
unknown-unsupported. Fresh triage on 2026-05-07 shows the concrete blocker is
the same missing merge conflict marker diagnostic tracked by issue 5305.

Problem: conflictMarkerDiff-unknown-unsupported has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conflictMarkerDiff3Trivia2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conflictMarkerDiff3Trivia2.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conflictMarkerDiff3Trivia2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conflictMarkerDiff3Trivia2.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; superseded by `issues/done/5305-report-merge-conflict-marker-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/conflictMarkerDiff3Trivia2.ts`

## Duplicate detection

- Superseded by `issues/done/5305-report-merge-conflict-marker-diagnostics.md`.
  The current first blocker is the same merge conflict marker diagnostic gap,
  with markers inside a method body rather than directly in a class body.
- Generic unknown-unsupported candidates are no-match because this bucket needs
  a specific conflict-marker diagnostic.

## Smart triage

Generated on 2026-05-07:

```text
Feature label: unknown-unsupported
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Path: reference/typescript/tests/cases/compiler/conflictMarkerDiff3Trivia2.ts
Failure: unsupported expression: Some(SpannedToken { kind: LeftShift, span: Span { start: 42, end: 44 } }) at 44..46
line: 4, column: 6
Visible symbols before failure:
- class C
```

Source context:

```text
2 | class C {
3 |   foo() {
4 | <<<<<<< B
5 |      a();
6 |   }
7 | ||||||| merged common ancestors
```

Compiler evidence:

```text
tokens: ok; markers tokenize as LeftShift/OrOr/StrictEqual/UnsignedRightShift groups
ast: false; unsupported expression at LeftShift inside method body
resolved: false; same parser diagnostic
```

TypeScript oracle evidence:

```text
TypeScript reports TS1185 "Merge conflict marker encountered" at lines 4, 7, 10, and 13.
TypeScript also reports TS2304 for unresolved `a` after recovering past the marker.
TypeScript AST still contains ClassDeclaration C, MethodDeclaration foo, and ExpressionStatement a().
```

Coverage result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/conflictMarkerDiff3Trivia2.ts --detail --no-dashboard-data
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
