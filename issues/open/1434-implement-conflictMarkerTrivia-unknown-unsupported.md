---
id: 1434
title: "Implement Conflictmarkertrivia Unknown Unsupported"
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
> Evidence: Empty completion evidence. No feat/fix commit for #1434.

## Summary

Triage conflictMarkerTrivia-unknown-unsupported across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases failing in directory
`conflictMarkerTrivia-unknown-unsupported` with diagnostics:
unknown-unsupported. Fresh triage on 2026-05-07 shows both affected paths are
the same missing merge conflict marker diagnostic tracked by issue 5305.

Problem: conflictMarkerTrivia-unknown-unsupported has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conflictMarkerTrivia4.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conflictMarkerTrivia4.ts --detail
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
- [x] Coverage names the exact reference window and current stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conflictMarkerTrivia4.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conflictMarkerTrivia4.ts
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

- `reference/typescript/tests/cases/compiler/conflictMarkerTrivia4.ts`
- `reference/typescript/tests/cases/compiler/conflictMarkerTrivia2.ts`

## Duplicate detection

- Superseded by `issues/open/5305-report-merge-conflict-marker-diagnostics.md`.
  Both affected paths hit merge conflict marker lines before any unrelated
  unknown-unsupported behavior.
- Generic unknown-unsupported candidates are no-match because this bucket needs
  a specific conflict-marker diagnostic.

## Smart triage

Generated on 2026-05-07.

Representative path `conflictMarkerTrivia4.ts`:

```text
Feature label: unknown-unsupported
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Path: reference/typescript/tests/cases/compiler/conflictMarkerTrivia4.ts
Failure: unsupported expression: Some(SpannedToken { kind: LeftShift, span: Span { start: 40, end: 42 } }) at 42..44
line: 3, column: 7
Visible symbols before failure:
- binding x initializer "<div>"
```

Source context:

```text
1 | // @target: es2015
2 | const x = <div>
3 | <<<<<<< HEAD
```

Second affected path `conflictMarkerTrivia2.ts`:

```text
Feature label: unknown-unsupported
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Path: reference/typescript/tests/cases/compiler/conflictMarkerTrivia2.ts
Failure: unsupported expression: Some(SpannedToken { kind: LeftShift, span: Span { start: 42, end: 44 } }) at 44..46
line: 4, column: 6
```

Compiler evidence:

```text
tokens: ok; markers tokenize as LeftShift/StrictEqual/UnsignedRightShift groups
ast: false; unsupported expression at LeftShift
resolved: false; same parser diagnostic
```

TypeScript oracle evidence:

```text
conflictMarkerTrivia4.ts: TypeScript reports TS1185 at line 3, plus preceding TS2304/TS1109 for `<div>`.
conflictMarkerTrivia2.ts: TypeScript reports TS1185 at lines 4, 7, and 10, and recovers to ClassDeclaration C, MethodDeclaration foo, and ExpressionStatement a().
```

Coverage result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/conflictMarkerTrivia --detail --no-dashboard-data
result: executed=3, unsupported=3, unsupported_diagcodes=UnsupportedSyntax:3, unsupported_features=unknown-unsupported:3, build_pass=0, semantic_pass=0, blocked=0
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
