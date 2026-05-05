---
id: 3996
title: "Implement Compiler (dup) (audit reopened #3996)"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-05status: open
---

## Summary

Triage reference/typescript/tests/cases/compiler across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `reference/typescript/tests/cases/compiler` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: reference/typescript/tests/cases/compiler has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/2dArrays.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/2dArrays.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [ ] Inspect the smart triage report below
- [ ] Confirm whether existing open/done issues already cover this bucket
- [ ] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [ ] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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

- [ ] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [ ] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [ ] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [ ] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/2dArrays.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/2dArrays.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/2dArrays.ts`

## Duplicate detection

- `issues/done/069-implement-APILibCheck.md` - Implement Apilibcheck (same feature label, same group key, title overlap)
- `issues/open/070-implement-APISample.md` - Implement Apisample (same feature label, same group key, title overlap)
- `issues/done/071-implement-ArrowFunctionExpression.md` - Implement Arrowfunctionexpression (same feature label, same group key, title overlap)
- `issues/done/072-implement-ClassDeclaration.md` - Implement Classdeclaration (same feature label, same group key, title overlap)
- `issues/done/073-implement-ClassDeclarationWithInvalidConstOnPropertyDeclaration.md` - Implement Classdeclarationwithinvalidconstonpropertydeclaration (same feature label, same group key, title overlap)
- `issues/open/076-implement-FunctionDeclaration.md` - Implement Functiondeclaration (same feature label, same group key, title overlap)
- `issues/done/079-implement-ParameterList.md` - Implement Parameterlist (same feature label, same group key, title overlap)
- `issues/done/081-implement-TransportStream.md` - Implement Transportstream (same feature label, same group key, title overlap)
- `issues/done/084-implement-abstractClassUnionInstantiation.md` - Implement Abstractclassunioninstantiation (same feature label, same group key, title overlap)
- `issues/done/086-implement-abstractPropertyBasics.md` - Implement Abstractpropertybasics (same feature label, same group key, title overlap)

## Smart triage

Not generated. Rerun with `--triage-limit 1` or higher.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none

## Status

Superseded by issue #199. Duplicate from separate coverage run.

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: blocked`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/3996-implement-reference-typescript-tests-cases-compiler.md` before this move
- `issues/open/3996-implement-reference-typescript-tests-cases-compiler.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
