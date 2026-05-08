---
id: 2050
title: "Implement Duplicatelocalvariable Duplicate Local"
type: spike
area: reference/triage
class: superseded
priority: P2
depends_on: [5162]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage duplicateLocalVariable-duplicate-local across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `duplicateLocalVariable-duplicate-local` with diagnostics: duplicate-local. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: duplicateLocalVariable-duplicate-local has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/duplicateLocalVariable3.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/duplicateLocalVariable3.ts --detail
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

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/duplicateLocalVariable3.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/duplicateLocalVariable3.ts
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

## Affected test files

- `reference/typescript/tests/cases/compiler/duplicateLocalVariable3.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated on 2026-05-06.

- Path: `reference/typescript/tests/cases/compiler/duplicateLocalVariable3.ts`
- Diagnostic: `DuplicateLocal` / `compiler-diagnostic`
- Failure: `duplicate local binding: \`x\` at 32..42`
- Source context: `var x = 1;` followed by `var x = 2;`
- TypeScript oracle: compatible `var x` and `var y` redeclarations are accepted; incompatible `var z` later reports TS2403.
- Split child: `issues/done/5162-allow-compatible-var-redeclarations.md`

## Completion evidence

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/duplicateLocalVariable3.ts
result: pass; current blocker identified as duplicate `var` redeclaration handling, split to issue 5162
date: 2026-05-06
```

Remaining risks:

- Incompatible redeclaration type diagnostics remain future work after issue 5162 advances past the current duplicate-local blocker.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

