---
id: 1502
title: "Implement Contextualsignatureinstantiation Unknown Unsupported"
type: spike
area: frontend/syntax
class: done
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1502.

## Summary

Triage contextualSignatureInstantiation-unknown-unsupported across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualSignatureInstantiation-unknown-unsupported` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualSignatureInstantiation-unknown-unsupported has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualSignatureInstantiation1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualSignatureInstantiation1.ts --detail
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
- [x] Existing issue 5304 contains an exact `reference-triage` command and now names this path
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualSignatureInstantiation1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualSignatureInstantiation1.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5304-parse-generic-arrow-functions-with-typed-parameters.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualSignatureInstantiation1.ts`

## Duplicate detection

- `issues/open/694-implement-arrayToLocaleStringES-unknown-unsupported.md` - Implement Arraytolocalestringes Unknown Unsupported (same feature label, title overlap)

## Smart triage

Fresh triage on 2026-05-07 shows this generated unknown-unsupported bucket is
now the same parser boundary already owned by
`issues/open/5304-parse-generic-arrow-functions-with-typed-parameters.md`.

Current diagnostic:

```text
UnsupportedSyntax: expected RightParen, got Some(Colon) at 95..96
```

Source context:

```ts
declare function map<S, T>(f: (x: S) => T): (a: S[]) => T[];
var e = <K>(x: string, y?: K) => x.length;
var r99 = map(e);
```

Compiler evidence:

```text
tokens: ok through declare function map, `var e = <K>(`, and parameter token `x`
ast/resolved: fail at the typed generic arrow parameter colon
visible symbols before failure: binding `e` with initializer prefix `<K>(x:`
TypeScript oracle: diagnostics=[]; binding e has type `<K>(x: string, y?: K | undefined) => number`
```

No new child issue was created because 5304 already scopes generic arrow
function expressions with typed parameters, including erasure of optional
parameter type annotations and return type annotations.

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

- Contextual signature instantiation and generic lambda inference semantics
  remain hidden until issue 5304 advances this file past the current parser
  boundary.
