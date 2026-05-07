---
id: 1070
title: "Implement Blockscopedenumvariablesusebeforedef Enum"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5184]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage blockScopedEnumVariablesUseBeforeDef-enum across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `blockScopedEnumVariablesUseBeforeDef-enum` with diagnostics: enum. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: blockScopedEnumVariablesUseBeforeDef-enum has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/blockScopedEnumVariablesUseBeforeDef.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/blockScopedEnumVariablesUseBeforeDef.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/blockScopedEnumVariablesUseBeforeDef.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/blockScopedEnumVariablesUseBeforeDef.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] added: `issues/open/5184-parse-const-enum-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/blockScopedEnumVariablesUseBeforeDef.ts`
- `reference/typescript/tests/cases/compiler/blockScopedEnumVariablesUseBeforeDef_preserve.ts`

## Duplicate detection

- `issues/open/428-implement-enum.md` - Implement enum support (same feature label, title overlap)
- `issues/open/1446-implement-constEnumDeclarations.md`, `issues/open/633-implement-amdModuleConstEnumUsage.md`, and `issues/open/737-implement-assignmentNonObjectTypeConstraints.md` show the same `const enum` parser boundary, but they are generated buckets rather than a focused implementation-ready child.

## Smart triage

Generated on 2026-05-06.

- Path: `reference/typescript/tests/cases/compiler/blockScopedEnumVariablesUseBeforeDef.ts`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Current compiler message: `const declarations require an initializer at 128..132`
- First failing source line: `const enum E { A }`
- Visible symbols before failure: functions `foo1`, `foo2`, and a bogus binding named `enum`
- Compiler evidence: token dump includes `Const`, `Ident("enum")`, `Ident("E")`, `{`, member `A`, and `}`; AST/resolved construction fails before representing the enum declaration.
- TypeScript oracle: `TS2450: Enum 'E' used before its declaration.`
- TypeScript AST path at the current blocker: `FunctionDeclaration -> Block -> EnumDeclaration`
- Superseding child: `issues/open/5184-parse-const-enum-declarations.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedEnumVariablesUseBeforeDef.ts
result: pass; current blocker identified as const enum parser support, split to issue 5184
date: 2026-05-06
```

Remaining risks:

- Later triage may expose ordinary enum parsing, enum used-before-declaration diagnostics, const-enum inlining, or export/module handling after issue 5184 advances past this parser boundary.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

