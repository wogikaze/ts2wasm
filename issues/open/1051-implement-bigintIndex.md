---
id: 1051
title: "Implement Bigintindex"
type: spike
area: frontend/resolver
class: superseded
priority: P1
depends_on: [5167]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage bigintIndex across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `bigintIndex` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: bigintIndex has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/bigintIndex.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/bigintIndex.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/bigintIndex.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/bigintIndex.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5167-support-global-symbol-builtin-call.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/bigintIndex.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated on 2026-05-06.

- Path: `reference/typescript/tests/cases/compiler/bigintIndex.ts`
- Diagnostic: `UnresolvedFunction` / `resolver-symbol`
- Failure: `unresolved function: Symbol`
- Source context: `key = Symbol();`
- Visible symbols before failure: `arr`, `num`, `key`, `bigNum`, `typedArray`
- Compiler evidence: tokens and AST succeed; resolved/lowered pipeline fails at builtin/function resolution for global `Symbol()`.
- TypeScript oracle: intended reference diagnostics are BigInt index/type diagnostics, including `Type 'bigint' cannot be used as an index type.`
- Split child: `issues/open/5167-support-global-symbol-builtin-call.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigintIndex.ts
result: pass; current blocker identified as unresolved global `Symbol()` call, split to issue 5167
date: 2026-05-06
```

Remaining risks:

- BigInt index/type diagnostics need follow-up triage after issue 5167 advances the pipeline beyond the current Symbol builtin-call blocker.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/open/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

