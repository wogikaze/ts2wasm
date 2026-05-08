---
id: 1429
title: "Implement Conditionaltypessimplifywhentrivial"
type: spike
area: frontend/semantics
class: done
priority: P1
depends_on: [5002]
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1429.

## Summary

Triage conditionalTypesSimplifyWhenTrivial across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory
`conditionalTypesSimplifyWhenTrivial` with diagnostics: type-system. Fresh
triage on 2026-05-07 shows the current first blocker is parser support for a
generic arrow function with typed parameters, before conditional type semantics.

Problem: conditionalTypesSimplifyWhenTrivial has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conditionalTypesSimplifyWhenTrivial.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conditionalTypesSimplifyWhenTrivial.ts --detail
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
- [x] Child issue 5304 contains an exact `python scripts/manager.py reference-triage ...` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conditionalTypesSimplifyWhenTrivial.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conditionalTypesSimplifyWhenTrivial.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5304-parse-generic-arrow-functions-with-typed-parameters.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/conditionalTypesSimplifyWhenTrivial.ts`

## Duplicate detection

- No existing open/done issue owns this exact parser gap.
- `issues/open/5154-parse-angle-bracket-type-assertion-statements.md` is related
  but explicitly excludes ambiguous generic arrow parsing such as
  `<T>(x: T) => x`.
- Generic type-system duplicate candidates are no-match because the current
  first failure happens before AST construction at a parameter type colon.

## Smart triage

Generated on 2026-05-07:

```text
Feature label: type-system
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Path: reference/typescript/tests/cases/compiler/conditionalTypesSimplifyWhenTrivial.ts
Failure: expected RightParen, got Some(Colon) at 71..72
line: 4, column: 14
Visible symbols before failure:
- binding fn1 initializer "<Params>("
```

Source context:

```text
3 | const fn1 = <Params>(
4 |     params: Pick<Params, Exclude<keyof Params, never>>,
5 | ): Params => params;
```

Compiler evidence:

```text
tokens: ok
ast: false; fails before AST construction with expected RightParen at the parameter type colon
resolved: false; same parser diagnostic
```

TypeScript oracle evidence:

```text
TypeScript diagnostics: none
binding fn1 type: <Params>(params: Pick<Params, Exclude<keyof Params, never>>) => Params
parameter params type: Pick<Params, Exclude<keyof Params, never>>
AST shape: VariableStatement -> VariableDeclaration fn1 -> ArrowFunction with type parameter Params, typed parameter params, return type Params
```

Coverage result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/conditionalTypesSimplifyWhenTrivial.ts --detail --no-dashboard-data
result: executed=1, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=type-system:1, build_pass=0, semantic_pass=0, blocked=0
date: 2026-05-07
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split concrete parser work to issue 5304 and closed this generated bucket.

Validation result:

```text
command: python scripts/manager.py update-issue-index && python scripts/manager.py update-issue-index --check && python scripts/manager.py check-issue-health && python scripts/manager.py check-issue-readiness -- --fail-ready-below 80 && git diff --check
result: pass
date: 2026-05-07
```

Remaining risks:

- none
