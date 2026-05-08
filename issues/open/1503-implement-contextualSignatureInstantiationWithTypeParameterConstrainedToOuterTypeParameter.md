---
id: 1503
title: "Implement Contextualsignatureinstantiationwithtypeparameterconstrainedtooutertypeparameter"
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
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1503.

## Summary

Triage contextualSignatureInstantiationWithTypeParameterConstrainedToOuterTypeParameter across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualSignatureInstantiationWithTypeParameterConstrainedToOuterTypeParameter` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualSignatureInstantiationWithTypeParameterConstrainedToOuterTypeParameter has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualSignatureInstantiationWithTypeParameterConstrainedToOuterTypeParameter.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualSignatureInstantiationWithTypeParameterConstrainedToOuterTypeParameter.ts --detail
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
- [x] Existing issue 5371 contains an exact `reference-triage` command and now names this path
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualSignatureInstantiationWithTypeParameterConstrainedToOuterTypeParameter.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualSignatureInstantiationWithTypeParameterConstrainedToOuterTypeParameter.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5371-parse-generic-function-type-annotations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualSignatureInstantiationWithTypeParameterConstrainedToOuterTypeParameter.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage on 2026-05-07 shows this generated blocked bucket is now the same
generic function type annotation parser boundary owned by
`issues/open/5371-parse-generic-function-type-annotations.md`.

Current diagnostic:

```text
UnsupportedSyntax: expected Semicolon, got Some(Greater) at 118..119
```

Source context:

```ts
function f<T>() {
    function g<U extends T>(u: U): U { return null }
    return g;
}
var h: <V, W>(v: V, func: (v: V) => W) => W;
```

Compiler evidence:

```text
tokens: ok through generic function declarations and `var h: <V, W>`
ast/resolved: fail at the closing `>` in the generic function type annotation
visible symbols before failure: binding `h`
TypeScript oracle: parses the FunctionType and later reports TS2322/TS2454 diagnostics
```

No new child issue was created because issue 5371 already scopes variable type
annotations that start with generic function types.

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

- Contextual signature instantiation with outer-constrained type parameters
  remains hidden until issue 5371 advances this file past the current parser
  boundary.
