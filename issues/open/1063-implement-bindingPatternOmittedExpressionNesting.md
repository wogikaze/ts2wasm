---
id: 1063
title: "Implement Bindingpatternomittedexpressionnesting"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5175]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage bindingPatternOmittedExpressionNesting across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `bindingPatternOmittedExpressionNesting` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: bindingPatternOmittedExpressionNesting has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/bindingPatternOmittedExpressionNesting.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/bindingPatternOmittedExpressionNesting.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/bindingPatternOmittedExpressionNesting.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/bindingPatternOmittedExpressionNesting.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] added: `issues/open/5175-support-export-let-destructuring-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/bindingPatternOmittedExpressionNesting.ts`

## Duplicate detection

- `issues/open/432-implement-import-export.md` is a broad generated import/export bucket, not an implementation-ready owner for this exact `export let` destructuring parser boundary.
- Existing export-function/default/export-const issues do not cover `export let` destructuring.

## Smart triage

Generated on 2026-05-06.

- Path: `reference/typescript/tests/cases/compiler/bindingPatternOmittedExpressionNesting.ts`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Failure: `issue-055: unsupported variable export; module resolution and loading are not implemented at 64..70`
- Source context: `export let [,,[,[],,[],]] = undefined as any;`
- Visible symbols before failure: none
- Compiler evidence: tokens succeed; AST and resolved dumps fail at the `export` token before parsing the declaration.
- TypeScript oracle: accepts the file with no diagnostics.
- Superseding child: `issues/open/5175-support-export-let-destructuring-declarations.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bindingPatternOmittedExpressionNesting.ts
result: pass; current blocker identified as exported `let` destructuring parser/module boundary, split to issue 5175
date: 2026-05-06
```

Remaining risks:

- Declaration emit and module export metadata may need follow-up triage after issue 5175 advances past the parser boundary.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/open/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

