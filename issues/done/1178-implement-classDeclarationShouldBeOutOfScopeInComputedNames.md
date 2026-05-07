---
id: 1178
title: "Implement Classdeclarationshouldbeoutofscopeincomputednames"
type: spike
area: frontend/resolver
class: blocked
priority: P2
depends_on: [5251]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Triage classDeclarationShouldBeOutOfScopeInComputedNames across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `classDeclarationShouldBeOutOfScopeInComputedNames` with diagnostics: scope-analysis. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: classDeclarationShouldBeOutOfScopeInComputedNames has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classDeclarationShouldBeOutOfScopeInComputedNames.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classDeclarationShouldBeOutOfScopeInComputedNames.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classDeclarationShouldBeOutOfScopeInComputedNames.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classDeclarationShouldBeOutOfScopeInComputedNames.ts
```

Not run:

- cargo gates; issue split only, no implementation changes

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5251-parse-computed-class-member-names-in-class-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classDeclarationShouldBeOutOfScopeInComputedNames.ts`

## Duplicate detection

- `issues/done/5214-computed-symbol-iterator-prerequisite-for-spread.md` is related but limited to class expressions in default parameter initializers; no exact owner found for computed class member names in class declarations.
- `issues/done/5087-implement-scope-analysis.md` had a stale generated-bucket link to this issue; it now points at this closed bucket and child issue 5251.

## Smart triage

Fresh triage on 2026-05-06:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classDeclarationShouldBeOutOfScopeInComputedNames.ts
```

Result:

```text
Smart triage class: scope-analysis
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Current error: expected property name, got Equal at 263..264
First failing form: static readonly [A.p1] = 0;
```

TypeScript oracle diagnostics:

```text
TS2449: Class 'A' used before its declaration.
```

The oracle reports TS2449 for all four computed `A.p1` / `A.p2` names in the
class body. The current compiler stops earlier in the parser, so the executable
child issue is parser support for computed class member names in declarations.

## Completion evidence

Commits:

- local split commit for issue 1178 / child 5251

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classDeclarationShouldBeOutOfScopeInComputedNames.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_features=scope-analysis:1
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classDeclarationShouldBeOutOfScopeInComputedNames.ts
result: pass; current blocker is parser/frontend UnsupportedSyntax at computed static readonly field
date: 2026-05-06
```

Remaining risks:

- Issue 5251 only removes the parser blocker; TS2449 class-name use before declaration in computed names may need a follow-up once parsing succeeds.
