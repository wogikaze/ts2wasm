---
id: 1078
title: "Implement Blockscopedsamenamefunctiondeclarationstrictes"
type: spike
area: reference/triage
class: superseded
priority: P1
depends_on: [5188]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage blockScopedSameNameFunctionDeclarationStrictES across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `blockScopedSameNameFunctionDeclarationStrictES` with diagnostics: arity. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: blockScopedSameNameFunctionDeclarationStrictES has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/blockScopedSameNameFunctionDeclarationStrictES5.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/blockScopedSameNameFunctionDeclarationStrictES5.ts --detail
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
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/blockScopedSameNameFunctionDeclarationStrictES5.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/blockScopedSameNameFunctionDeclarationStrictES5.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] folded into `issues/open/5188-report-block-scoped-function-call-arity-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/blockScopedSameNameFunctionDeclarationStrictES5.ts`
- `reference/typescript/tests/cases/compiler/blockScopedSameNameFunctionDeclarationStrictES6.ts`

## Duplicate detection

- `issues/open/5188-report-block-scoped-function-call-arity-diagnostics.md` is an exact implementation-ready owner for this strict-mode sibling.
- Builtin arity buckets are not matches; these calls resolve to user-defined block-scoped functions.

## Smart triage

Generated on 2026-05-06.

- Paths:
  - `reference/typescript/tests/cases/compiler/blockScopedSameNameFunctionDeclarationStrictES5.ts`
  - `reference/typescript/tests/cases/compiler/blockScopedSameNameFunctionDeclarationStrictES6.ts`
- Diagnostics:
  - Strict ES5: `BuildPass` / `pass`
  - Strict ES6: `BackendIo` / `backend-io` after WAT generation
- Source context: `"use strict"; function foo(a: number) { ... function foo() { } ... foo(); foo(10); }`.
- Compiler evidence: tokens, AST, and resolved IR succeed for both files; resolved IR contains source spans for the wrong-arity call sites. Strict ES5 emits no diagnostic, and strict ES6 reaches a later backend failure.
- TypeScript oracle: four `TS2554` diagnostics per file for `Expected 0 arguments, but got 1.` and `Expected 1 arguments, but got 0.`
- Superseded by child: `issues/open/5188-report-block-scoped-function-call-arity-diagnostics.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedSameNameFunctionDeclarationStrictES5.ts
result: pass; current blocker is the same missing user-defined function arity diagnostics tracked by issue 5188
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedSameNameFunctionDeclarationStrictES6.ts
result: pass; current blocker should surface TS2554 before the later backend failure, tracked by issue 5188
date: 2026-05-06
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

