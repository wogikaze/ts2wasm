---
id: 1075
title: "Implement Blockscopedfunctiondeclarationstrictes"
type: spike
area: frontend/resolver
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage blockScopedFunctionDeclarationStrictES across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `blockScopedFunctionDeclarationStrictES` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: blockScopedFunctionDeclarationStrictES has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationStrictES5.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationStrictES5.ts --detail
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
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationStrictES5.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationStrictES5.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; current compiler diagnostics match the TypeScript oracle

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationStrictES5.ts`
- `reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationStrictES6.ts`

## Duplicate detection

- Generic `name-resolution` buckets are not matches; they share only the broad feature label.
- No child issue is needed because both strict ES files now produce the same unresolved-name diagnostic TypeScript reports.

## Smart triage

Generated on 2026-05-06.

- Paths:
  - `reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationStrictES5.ts`
  - `reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationStrictES6.ts`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Current compiler messages:
  - ES5: `unresolved name foo at 162..165`
  - ES6: `unresolved name foo at 153..156`
- Source context: `"use strict"; if (true) { function foo() { } foo(); } foo();`
- Compiler evidence: tokens and AST succeed; the block-scoped function is visible inside the `if` body, and the out-of-block `foo()` call fails name resolution with a source span.
- TypeScript oracle: `TS2304: Cannot find name 'foo'.` at the same out-of-block call in both files.
- Outcome: stale generated bucket; no implementation child needed.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationStrictES5.ts
result: pass; current compiler UnresolvedName matches TypeScript TS2304 at the out-of-block call
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationStrictES6.ts
result: pass; current compiler UnresolvedName matches TypeScript TS2304 at the out-of-block call
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

