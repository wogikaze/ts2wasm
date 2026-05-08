---
id: 1073
title: "Implement Blockscopedfunctiondeclarationinstrictclass"
type: spike
area: frontend/resolver
class: superseded
priority: P1
depends_on: [5185]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage blockScopedFunctionDeclarationInStrictClass across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `blockScopedFunctionDeclarationInStrictClass` with diagnostics: function-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: blockScopedFunctionDeclarationInStrictClass has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationInStrictClass.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationInStrictClass.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationInStrictClass.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationInStrictClass.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] added: `issues/open/5185-source-span-unresolved-class-method-function-calls.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationInStrictClass.ts`

## Duplicate detection

- Generic `function-resolution` buckets are not matches; they share only the broad feature label.
- Existing completed function-resolution issues do not cover this unspanned class-method unresolved-call diagnostic.

## Smart triage

Generated on 2026-05-06.

- Path: `reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationInStrictClass.ts`
- Diagnostic: `UnresolvedFunction` / `resolver-symbol`
- Current compiler message: `unresolved function: `foo``
- Source context: `class c { method() { if (true) { function foo() { } foo(); } foo(); } }`
- Compiler evidence: tokens and AST succeed; the in-block `foo()` call is represented inside the `if`, and the out-of-block `foo()` call is represented at byte span `150..155`; lowering later emits an unspanned `UnresolvedFunction`.
- TypeScript oracle: `TS2304: Cannot find name 'foo'.` at the out-of-block `foo` identifier.
- Superseding child: `issues/open/5185-source-span-unresolved-class-method-function-calls.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationInStrictClass.ts
result: pass; current blocker identified as unspanned unresolved function diagnostic in a class method, split to issue 5185
date: 2026-05-06
```

Remaining risks:

- none for this generated bucket; issue 5185 owns the diagnostic quality gap.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

