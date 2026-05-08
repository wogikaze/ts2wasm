---
id: 1483
title: "Implement Constructorwithparameterpropertiesandprivatefields"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1483.

## Summary

Triage constructorWithParameterPropertiesAndPrivateFields across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `constructorWithParameterPropertiesAndPrivateFields` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: constructorWithParameterPropertiesAndPrivateFields has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constructorWithParameterPropertiesAndPrivateFields.es2015.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constructorWithParameterPropertiesAndPrivateFields.es2015.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constructorWithParameterPropertiesAndPrivateFields.es2015.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constructorWithParameterPropertiesAndPrivateFields.es2015.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] split to `issues/done/5365-parse-readonly-private-field-type-annotations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constructorWithParameterPropertiesAndPrivateFields.es2015.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage on 2026-05-07 shows this generated bucket is stopped before
constructor parameter properties. Tokens succeed, but AST construction rejects
the `readonly #privateField: string;` declaration at the type-annotation colon.

Current diagnostic:

```text
error: [UnsupportedSyntax] expected property name, got PrivateIdentifier("privateField") at 114..115
```

Source context:

```ts
class A {
  readonly #privateField: string;

  constructor(arg: { key: string }, public exposedField: number) {
    ({ key: this.#privateField } = arg);
  }
}
```

TypeScript oracle: accepted with no diagnostics.

This bucket was split to `issues/done/5365-parse-readonly-private-field-type-annotations.md`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...` (filled by commit that moves this issue)

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorWithParameterPropertiesAndPrivateFields.es2015.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, diagnostic UnsupportedSyntax
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorWithParameterPropertiesAndPrivateFields.es2015.ts
result: pass; reproduced readonly private typed field parser blocker and split child issue 5365
date: 2026-05-07
```

Remaining risks:

- Parameter property/private-field assignment behavior remains hidden until
  issue 5365 advances past the first class field parser blocker.
