---
id: 1064
title: "Implement Binopassignmentshouldhavetype"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5176, 5177]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage binopAssignmentShouldHaveType across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `binopAssignmentShouldHaveType` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: binopAssignmentShouldHaveType has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/binopAssignmentShouldHaveType.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/binopAssignmentShouldHaveType.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/binopAssignmentShouldHaveType.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/binopAssignmentShouldHaveType.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] added: `issues/open/5176-report-ambient-var-lib-redeclaration-diagnostics.md`
- [x] added: `issues/open/5177-report-strict-null-in-erased-namespace-methods.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/binopAssignmentShouldHaveType.ts`

## Duplicate detection

- `issues/open/432-implement-import-export.md` is not a match for the current blocker: the reference case now builds successfully, and current evidence is about hidden TypeScript oracle diagnostics.
- `issues/open/5162-allow-compatible-var-redeclarations.md` is related but not an exact match. It covers compatible duplicate `var` declarations; the current first oracle diagnostic is an ambient `declare var console;` conflict with a lib global.
- No open issue was found for the namespace class method `var name: string = null` diagnostic hidden by namespace erasure.

## Smart triage

Generated on 2026-05-06.

- Path: `reference/typescript/tests/cases/compiler/binopAssignmentShouldHaveType.ts`
- Diagnostic: `BuildPass` / `pass`
- Failure: ts2wasm builds successfully while TypeScript reports two diagnostics.
- Source context begins with `declare var console;`, then `"use strict";`, then `namespace Test { export class Bug { ... } }`.
- Visible symbols before mismatch: binding `console`, class `Bug`, binding `name`.
- Compiler evidence: tokens include `declare`, `var`, `console`, `namespace`, `export`, `class`, method `bug`, typed local `name`, and `null`; AST and resolved dumps contain only `"use strict"` because the ambient declaration and namespace body are erased.
- TypeScript oracle:
  - `TS2403`: `Variable 'console' must be of type 'Console', but here has type 'any'.`
  - `TS2322`: `Type 'null' is not assignable to type 'string'.`
- Superseding children:
  - `issues/open/5176-report-ambient-var-lib-redeclaration-diagnostics.md`
  - `issues/open/5177-report-strict-null-in-erased-namespace-methods.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binopAssignmentShouldHaveType.ts
result: pass; current mismatch identified as a false build pass with hidden TS2403 and TS2322 oracle diagnostics, split to issues 5176 and 5177
date: 2026-05-06
```

Remaining risks:

- The representative case has multiple TypeScript diagnostics; issue 5176 owns the earlier ambient redeclaration diagnostic, and issue 5177 owns the namespace method strict-null diagnostic.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

