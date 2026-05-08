---
id: 803
title: "Implement Accessordeclarationorder (audit reopened #803)"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage accessorDeclarationOrder across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `accessorDeclarationOrder` with diagnostics: class-accessor. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: accessorDeclarationOrder has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorDeclarationOrder.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorDeclarationOrder.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorDeclarationOrder.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorDeclarationOrder.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/accessorDeclarationOrder.ts`

## Duplicate detection

- `issues/done/098-implement-accessorDeclarationOrder.md` - Implement Accessordeclarationorder (same reference path, same feature label, same group key, title overlap)
- `issues/done/422-implement-class-accessor.md` - Implement class-accessor support (same feature label, same group key, title overlap)
- `issues/done/482-implement-accessorDeclarationOrder.md` - Implement Accessordeclarationorder (same reference path, same feature label, same group key, title overlap)
- `issues/done/568-implement-accessorDeclarationOrder.md` - Implement Accessordeclarationorder (same reference path, same feature label, same group key, title overlap)

## Smart triage

Not generated. Rerun with `--triage-limit 1` or higher.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending closure commit

Validation result:

```text
command: python scripts/manager.py update-issue-index --check
result: pass
date: 2026-05-06
```

Remaining risks:

- none

## Status

Superseded by issue #098. Duplicate from separate coverage run.

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: blocked`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- open issue file before this move
- `issues/done/803-implement-accessorDeclarationOrder.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
