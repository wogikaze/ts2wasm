---
id: 1505
title: "Implement Contextualtupletypeparameterreadonly"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1505.

## Summary

Triage contextualTupleTypeParameterReadonly across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualTupleTypeParameterReadonly` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualTupleTypeParameterReadonly has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTupleTypeParameterReadonly.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTupleTypeParameterReadonly.ts --detail
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
- [x] Existing issue 5281 contains an exact `reference-triage` command and now names this path
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTupleTypeParameterReadonly.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTupleTypeParameterReadonly.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5281-resolve-commented-arrow-rest-parameters.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualTupleTypeParameterReadonly.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage on 2026-05-07 shows this generated blocked bucket is now the same
arrow rest parameter resolver binding gap owned by
`issues/open/5281-resolve-commented-arrow-rest-parameters.md`.

Current diagnostic:

```text
UnresolvedName: unresolved name: `args` at 397..401
```

Source context:

```ts
eacher((...args) => {
    const [a, b] = args;
    a;
    b;
});
```

Compiler evidence:

```text
tokens: ok through ReadonlyArray annotations, `as const`, and arrow rest parameter
ast: ok; second eacher call contains ArrowFn with rest parameter args and body_stmts
resolved: fail in resolve_names for `args` inside `const [a, b] = args`
TypeScript oracle: `args` parameter has readonly tuple union type and reports earlier TS2345
```

No new child issue was created because issue 5281 already scopes resolver
binding for arrow rest parameters. This file confirms the same resolver gap
without comments between `...` and the parameter name.

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

- Readonly tuple contextual typing remains hidden until issue 5281 advances the
  file past the current arrow rest parameter resolver boundary.
