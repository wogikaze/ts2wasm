---
id: 1500
title: "Implement Contextualsignatureinstantiation Duplicate Local"
type: spike
area: reference/triage
class: done
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Triage contextualSignatureInstantiation-duplicate-local across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualSignatureInstantiation-duplicate-local` with diagnostics: duplicate-local. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualSignatureInstantiation-duplicate-local has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualSignatureInstantiation3.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualSignatureInstantiation3.ts --detail
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

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualSignatureInstantiation3.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualSignatureInstantiation3.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5234-track-array-typed-parameters-for-callback-methods.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualSignatureInstantiation3.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage on 2026-05-07 shows the original duplicate-local bucket has
advanced to the existing array-typed parameter receiver blocker owned by
`issues/open/5234-track-array-typed-parameters-for-callback-methods.md`.

Current diagnostic:

```text
UnsupportedSyntax: issue-211: unknown receiver class for method `map` at 85..97
```

Source context:

```ts
function map<T, U>(items: T[], f: (x: T) => U): U[]{
    return items.map(f);
}
```

Compiler evidence:

```text
tokens: ok through generic function, T[] parameter annotation, and items.map(f)
ast: ok; function body contains Return(Call(Member(Ident("items"), "map"), [Ident("f")]))
resolved/lowered: fails at lower_program with issue-211 unknown receiver class for method map
TypeScript oracle: diagnostics=[]; parameter items has type T[]
```

No new child issue was created because issue 5234 already scopes function and
class-method parameters annotated as `Array<T>`, `ReadonlyArray<T>`, or `T[]`
so supported array callback methods reach the known-array receiver path.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...` (filled by commit that moves this issue)

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualSignatureInstantiation3.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnsupportedSyntax=1, unsupported_features=unknown-unsupported:1
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualSignatureInstantiation3.ts
result: pass; issue-211 unknown receiver class for items.map(f), folded into issue 5234
date: 2026-05-07
```

Remaining risks:

- Contextual signature instantiation semantics remain hidden until issue 5234
  advances this file past the current array receiver tracking boundary.
