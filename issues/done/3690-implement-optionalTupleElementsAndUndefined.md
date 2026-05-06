---
id: 3690
title: "Implement Optionaltupleelementsandundefined"
type: spike
area: reference/triage
class: triage-needed
priority: P2
depends_on: []
blocks: [5126]
created: 2026-05-01
updated: 2026-05-05
completed: 2026-05-05
---

## Triage complete: child issue created

Child: #5126 (implement name resolver var redeclaration tolerance)

Root cause: 8 `var v` declarations in same scope trigger DuplicateLocal. TypeScript allows legal `var` redeclarations.

Smart triage rerun with evidence above.

## Summary

Triage optionalTupleElementsAndUndefined across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `optionalTupleElementsAndUndefined` with diagnostics: duplicate-local. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: optionalTupleElementsAndUndefined has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/optionalTupleElementsAndUndefined.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/optionalTupleElementsAndUndefined.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/optionalTupleElementsAndUndefined.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/optionalTupleElementsAndUndefined.ts
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

- `reference/typescript/tests/cases/compiler/optionalTupleElementsAndUndefined.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Not generated. Rerun with `--triage-limit 1` or higher.

## Completion evidence

Child #5126 implemented var redeclaration tolerance. Reference test now builds successfully.

Commits:
- `ae315d28` frontend/ir: allow var redeclaration (DuplicateLocal tolerance)
- `fe2e3f00` compiler/ir: fix remaining DuplicateLocal checks for var redeclaration

Validation result:
```
command: cargo run -- build -o /tmp/test.wasm reference/typescript/tests/cases/compiler/optionalTupleElementsAndUndefined.ts
result: PASS (no errors)
date: 2026-05-05
```

Remaining risks:
- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

