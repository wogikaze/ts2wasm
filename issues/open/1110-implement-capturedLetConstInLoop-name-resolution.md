---
id: 1110
title: "Implement Capturedletconstinloop Name Resolution"
type: spike
area: frontend/resolver
class: triage-needed
priority: P2
depends_on: [5206]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1110.

## Summary

Triage capturedLetConstInLoop-name-resolution across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `capturedLetConstInLoop-name-resolution` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: capturedLetConstInLoop-name-resolution has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop3_ES6.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/capturedLetConstInLoop3_ES6.ts --detail
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
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/capturedLetConstInLoop3_ES6.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop3_ES6.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5206-resolve-issue-id-collisions-and-open-done-conflicts.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/capturedLetConstInLoop3_ES6.ts`
- `reference/typescript/tests/cases/compiler/capturedLetConstInLoop3.ts`

## Duplicate detection

- `issues/open/064-implement-name-resolution.md` - Implement name resolution (triaged - superseded by test262 metadata issues) (same feature label, title overlap)
- `issues/open/437-implement-name-resolution.md` - Implement name resolution (same feature label, title overlap)
- `issues/open/648-implement-argumentsAsPropertyName-name-resolution.md` - Implement Argumentsaspropertyname Name Resolution (same feature label, title overlap)
- `issues/open/654-implement-argumentsReferenceInConstructor-name-resolution.md` - Implement Argumentsreferenceinconstructor Name Resolution (same feature label, title overlap)
- `issues/open/657-implement-argumentsReferenceInMethod-name-resolution.md` - Implement Argumentsreferenceinmethod Name Resolution (same feature label, title overlap)
- `issues/open/693-implement-arrayToLocaleStringES-name-resolution.md` - Implement Arraytolocalestringes Name Resolution (same feature label, title overlap)
- `issues/open/733-implement-assignmentCompatability-name-resolution.md` - Implement Assignmentcompatability Name Resolution (same feature label, title overlap)

## Smart triage

Fresh triage shows both affected files parse and produce AST successfully. The
current blocker is name resolution for `var v` declared inside loop bodies and
read after the loop. Child issue
`issues/open/5206-resolve-issue-id-collisions-and-open-done-conflicts.md`
owns the implementation-ready resolver slice.

### Smart triage: capturedLetConstInLoop3_ES6

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/capturedLetConstInLoop3_ES6.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop3_ES6.ts
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `v` at 232..233",
  "line": 16,
  "column": 6
}
```

Source context:

```text
16 | function foo0_1(x) {
17 |     for (let x in []) {
18 |         var v = x;
19 |         (function() { return x + v });
```

AST evidence:

```text
Function foo0_1 contains a ForOf with var binding `v = x`, then a later
Call `use(v)` in the same function body. The resolved dump fails at
resolve_names with `UnresolvedName`.
```

TypeScript oracle:

```text
TS2454: Variable 'v' is used before being assigned.
```

### Smart triage: capturedLetConstInLoop3

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/capturedLetConstInLoop3.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop3.ts
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `v` at 233..234",
  "line": 15,
  "column": 5
}
```

Source context:

```text
15 | function foo0_1(x) {
16 |     for (let x in []) {
17 |         var v = x;
18 |         (function() { return x + v });
```

AST evidence:

```text
Function foo0_1 contains a ForOf with var binding `v = x`, then a later
Call `use(v)` in the same function body. The resolved dump fails at
resolve_names with `UnresolvedName`.
```

TypeScript oracle:

```text
TS2454: Variable 'v' is used before being assigned.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`
- pending

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop3_ES6.ts
result: fail; UnresolvedName for post-loop read of loop-body `var v`
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop3.ts
result: fail; UnresolvedName for post-loop read of loop-body `var v`
date: 2026-05-06
```

Remaining risks:

- none
