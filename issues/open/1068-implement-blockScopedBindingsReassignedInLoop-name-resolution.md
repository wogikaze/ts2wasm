---
id: 1068
title: "Implement Blockscopedbindingsreassignedinloop Name Resolution"
type: spike
area: frontend/resolver
class: superseded
priority: P1
depends_on: [5181]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage blockScopedBindingsReassignedInLoop-name-resolution across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `blockScopedBindingsReassignedInLoop-name-resolution` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: blockScopedBindingsReassignedInLoop-name-resolution has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/blockScopedBindingsReassignedInLoop1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/blockScopedBindingsReassignedInLoop1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/blockScopedBindingsReassignedInLoop1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/blockScopedBindingsReassignedInLoop1.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] added: `issues/open/5181-support-prefix-update-expressions-in-call-arguments.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/blockScopedBindingsReassignedInLoop1.ts`

## Duplicate detection

- `issues/open/064-implement-name-resolution.md` - Implement name resolution (triaged - superseded by test262 metadata issues) (same feature label, title overlap)
- `issues/open/437-implement-name-resolution.md` - Implement name resolution (same feature label, title overlap)
- `issues/open/648-implement-argumentsAsPropertyName-name-resolution.md` - Implement Argumentsaspropertyname Name Resolution (same feature label, title overlap)
- `issues/open/654-implement-argumentsReferenceInConstructor-name-resolution.md` - Implement Argumentsreferenceinconstructor Name Resolution (same feature label, title overlap)
- `issues/open/657-implement-argumentsReferenceInMethod-name-resolution.md` - Implement Argumentsreferenceinmethod Name Resolution (same feature label, title overlap)
- `issues/open/693-implement-arrayToLocaleStringES-name-resolution.md` - Implement Arraytolocalestringes Name Resolution (same feature label, title overlap)
- `issues/open/733-implement-assignmentCompatability-name-resolution.md` - Implement Assignmentcompatability Name Resolution (same feature label, title overlap)
- `issues/open/268-implement-for-loop-increment-operator.md` is related but not a match. It completed for-loop update-slot support, while the current blocker is `++i` in a call argument expression.
- `issues/open/1069-implement-blockScopedBindingsReassignedInLoop-scope-analysis.md` is a sibling generated bucket, not an implementation-ready child for the current issue-268 diagnostic.

## Smart triage

Generated on 2026-05-06.

- Path: `reference/typescript/tests/cases/compiler/blockScopedBindingsReassignedInLoop1.ts`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Current compiler message: `issue-268: for-loop increment/decrement updates currently require an identifier target at 140..143`
- First failing source line: `(() => use(++i))();`
- Visible symbols before failure: ambient function `use`, loop binding `i`
- Compiler evidence: tokens and AST succeed; the for-loop update `++i` is represented in `For.update`, and the call argument is represented as `Unary { op: PreIncrement, expr: Ident("i") }`; resolved pipeline fails before lowering.
- TypeScript oracle: no diagnostics for the representative file.
- TypeScript AST path at the failure: `ExpressionStatement -> CallExpression -> ParenthesizedExpression -> ArrowFunction -> CallExpression -> PrefixUnaryExpression`
- Superseding child: `issues/open/5181-support-prefix-update-expressions-in-call-arguments.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedBindingsReassignedInLoop1.ts
result: pass; current blocker identified as prefix update expression support in a call argument, split to issue 5181
date: 2026-05-06
```

Remaining risks:

- Later triage may expose closure or block-scoped loop reassignment semantics after issue 5181 advances past `use(++i)`.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/open/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

