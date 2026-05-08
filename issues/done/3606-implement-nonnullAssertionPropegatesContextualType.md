---
id: 3606
title: "Implement Nonnullassertionpropegatescontextualtype"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: [5491]
created: 2026-05-01
updated: 2026-05-08
status: done
---

## Summary

Triage nonnullAssertionPropegatesContextualType across 1 failing reference test
case and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `nonnullAssertionPropegatesContextualType` with diagnostics: type-assertion. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: nonnullAssertionPropegatesContextualType has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nonnullAssertionPropegatesContextualType.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nonnullAssertionPropegatesContextualType.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

Current close decision: superseded by child issue
`issues/open/5491-bind-dom-document-queryselector-global.md`.

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nonnullAssertionPropegatesContextualType.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nonnullAssertionPropegatesContextualType.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Close note, 2026-05-08:

- Fresh triage shows the original generated `type-assertion` bucket is stale.
- Tokens and AST now parse `document.querySelector('.svg-rectangle')!`; the
  runtime AST erases the non-null assertion to a call expression.
- Name resolution stops on DOM global receiver `document`:
  `UnresolvedName: unresolved name: \`document\` at 66..74`.
- TypeScript oracle accepts the reference with no diagnostics.
- The concrete remaining blocker is split to
  `issues/open/5491-bind-dom-document-queryselector-global.md`.
- Existing DOM issues were checked and are narrower: issue 5386 covers
  `setTimeout`, and issue 5479 covers worker `self/importScripts`.

## Affected test files

- `reference/typescript/tests/cases/compiler/nonnullAssertionPropegatesContextualType.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh smart triage on 2026-05-08:

```text
Diagnostic: UnresolvedName / resolver-symbol
Feature label: name-resolution
Path: reference/typescript/tests/cases/compiler/nonnullAssertionPropegatesContextualType.ts
Failure location: unresolved name: `document` at 66..74
Visible symbols before failure: binding rect2
TypeScript oracle: ok, diagnostics []
```

Focused coverage on 2026-05-08:

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `d5925ca3e`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonnullAssertionPropegatesContextualType.ts
result: pass; reproduced current UnresolvedName blocker for DOM global `document`
date: 2026-05-08

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonnullAssertionPropegatesContextualType.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnresolvedName:1
date: 2026-05-08
```

Remaining risks:

- Child issue 5491 still needs implementation.
