---
id: 3607
title: "Implement Normalizedintersectiontoocomplex"
type: spike
area: frontend/resolver
class: superseded
priority: P1
depends_on: []
blocks: [5161]
created: 2026-05-01
updated: 2026-05-08
status: done
---

## Summary

Triage normalizedIntersectionTooComplex across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `normalizedIntersectionTooComplex` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: normalizedIntersectionTooComplex has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/normalizedIntersectionTooComplex.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/normalizedIntersectionTooComplex.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

Current close decision: superseded by
`issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`.

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/normalizedIntersectionTooComplex.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/normalizedIntersectionTooComplex.ts
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

- Fresh triage shows the current blocker is
  `UnresolvedName: unresolved name: \`all\` at 1979..1982` in
  `const ctor = getCtor(all);`.
- The source declares `declare var all: keyof Big;` immediately before the
  call; smart triage visible-symbol extraction sees `all`, but resolver-visible
  runtime metadata is still missing after ambient declaration erasure.
- TypeScript oracle reaches the intended TS2590 diagnostic on the later
  `ref: x => console.log(x)` object-literal callback, but that type-system
  behavior remains unreachable until the ambient `all` reference resolves.
- Existing issue 5161 already owns declaration-only ambient `declare var` /
  `declare let` / `declare const` expression references, so this generated
  bucket is folded into that issue rather than split into a new child.

## Affected test files

- `reference/typescript/tests/cases/compiler/normalizedIntersectionTooComplex.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh smart triage on 2026-05-08:

```text
Diagnostic: UnresolvedName / resolver-symbol
Feature label: name-resolution
Path: reference/typescript/tests/cases/compiler/normalizedIntersectionTooComplex.ts
Failure location: unresolved name: `all` at 1979..1982
Source context: declare var all: keyof Big; const ctor = getCtor(all);
Visible symbols before failure: all, ctor, comp
TypeScript oracle: TS2590 at line 39, column 40
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

- `b72e19700`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/normalizedIntersectionTooComplex.ts
result: pass; reproduced current UnresolvedName blocker for ambient var `all`
date: 2026-05-08

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/normalizedIntersectionTooComplex.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnresolvedName:1
date: 2026-05-08
```

Remaining risks:

- Issue 5161 still needs implementation. After ambient var references resolve,
  the file is expected to expose the intended TS2590 normalized-intersection
  diagnostic.
