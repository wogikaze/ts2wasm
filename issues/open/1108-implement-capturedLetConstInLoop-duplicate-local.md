---
id: 1108
title: "Implement Capturedletconstinloop Duplicate Local"
type: spike
area: reference/triage
class: triage-needed
priority: P2
depends_on: [5205]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1108.

## Summary

Triage capturedLetConstInLoop-duplicate-local across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `capturedLetConstInLoop-duplicate-local` with diagnostics: duplicate-local. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: capturedLetConstInLoop-duplicate-local has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop14.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/capturedLetConstInLoop14.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/capturedLetConstInLoop14.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop14.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/done/5205-restore-backend-residual-expression-rejection.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/capturedLetConstInLoop14.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Build pass: capturedLetConstInLoop14

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/capturedLetConstInLoop14.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop14.ts
```

Current compiler result:

```text
ts2wasm build succeeded
```

TypeScript oracle diagnostic:

```text
TS2403: Subsequent variable declarations must have the same type.
Variable 'v' must be of type 'number', but here has type 'any'.
```

Evidence:

- Tokens, AST, and resolved dumps succeed.
- The reference file contains `var v = 1`, then a nested `do` body with
  bodyless `var v` and `var v = 2`.
- Existing issue `issues/done/5162-allow-compatible-var-redeclarations.md` is
  related but no-match: it removes false duplicate-local blockers for
  compatible redeclarations, while this bucket now needs the incompatible
  redeclaration diagnostic after build pass.
- Child issue
  `issues/done/5205-restore-backend-residual-expression-rejection.md`
  owns the diagnostic implementation slice.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...` pending

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop14.ts
result: pass; compiler build succeeded while TypeScript reports TS2403 for incompatible `var v`
date: 2026-05-06
```

Remaining risks:

- none
