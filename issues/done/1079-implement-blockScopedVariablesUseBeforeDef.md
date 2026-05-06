---
id: 1079
title: "Implement Blockscopedvariablesusebeforedef"
type: spike
area: frontend/resolver
class: superseded
priority: P1
depends_on: [5189]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage blockScopedVariablesUseBeforeDef across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `blockScopedVariablesUseBeforeDef` with diagnostics: scope-analysis. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: blockScopedVariablesUseBeforeDef has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/blockScopedVariablesUseBeforeDef.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/blockScopedVariablesUseBeforeDef.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/blockScopedVariablesUseBeforeDef.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/blockScopedVariablesUseBeforeDef.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] added: `issues/done/5189-parse-asi-after-class-expression-variable-initializer.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/blockScopedVariablesUseBeforeDef.ts`

## Duplicate detection

- `issues/done/5169-parse-asi-after-expression-statement.md` is not a match; this failure is a variable declaration initializer, not a completed expression statement.
- Generic scope-analysis buckets are not matches because the current blocker is parser ASI before scope diagnostics.

## Smart triage

Generated on 2026-05-06.

- Path: `reference/typescript/tests/cases/compiler/blockScopedVariablesUseBeforeDef.ts`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Current compiler message: `expected Semicolon, got Some(Let) at 718..721`
- Source context: `function foo9() { let y = class { static a = x; } let x; }`
- Compiler evidence: tokens succeed through the class expression and following `Let`; AST/resolved construction fails at the following `let x;`.
- TypeScript oracle: `TS2448: Block-scoped variable 'x' used before its declaration.` at the `x` inside the static class field initializer.
- Superseding child: `issues/done/5189-parse-asi-after-class-expression-variable-initializer.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedVariablesUseBeforeDef.ts
result: pass; current blocker is parser ASI after an anonymous class-expression variable initializer, split to issue 5189
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

