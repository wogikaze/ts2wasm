---
id: 4262
title: "Implement Staticsinconstructorbodies"
type: spike
area: frontend/syntax
class: done
priority: P2
depends_on: [5246]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #4262.

## Summary

Triage staticsInConstructorBodies across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `staticsInConstructorBodies` with diagnostics: unknown-unsupported. Fresh triage shows the current first blocker is invalid `static` declarations inside a constructor body, now split to issue 5246.

Problem: this generated bucket is not a direct implementation order. The first blocker is a focused invalid `static` statement diagnostic split to issue 5246.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/staticsInConstructorBodies.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/staticsInConstructorBodies.ts --detail
```

## Desired final state

This generated bucket is closed after splitting `issues/open/5246-report-static-declarations-inside-constructor-bodies.md`. Do not implement directly from this bucket.

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
- [x] Child issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/staticsInConstructorBodies.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/staticsInConstructorBodies.ts
```

Not run:

- `cargo fmt --all --check`; issue split only, no Rust code changed
- `cargo nextest run`; issue split only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5246-report-static-declarations-inside-constructor-bodies.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/staticsInConstructorBodies.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage shows this generated bucket is currently blocked by the same
invalid constructor-body `static` syntax as issue 1170.

Split result:

- `issues/open/5246-report-static-declarations-inside-constructor-bodies.md`

## Completion evidence

Fill only when moving to `done/`.

The `staticsInConstructorBodies` generated bucket is complete. The current failure is split to issue 5246.

Commits:

- split to `issues/open/5246-report-static-declarations-inside-constructor-bodies.md`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/staticsInConstructorBodies.ts --detail --no-dashboard-data
result: pass on the main checkout; 1 executed, current failure is UnsupportedSyntax/unknown-unsupported
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/staticsInConstructorBodies.ts
result: pass; AST construction reports unsupported expression at constructor-body `static p1 = 0`, split to issue 5246
date: 2026-05-06
```

Remaining risks:

- none
