---
id: 1374
title: "Implement Commentsonobjectliteral Name Resolution"
type: spike
area: frontend/resolver
class: blocked
priority: P1
depends_on: [5005]
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1374.

## Summary

Closed as superseded by `issues/open/056-implement-name-resolution.md`.

Fresh triage shows both representatives now fail with an oracle-matching
`UnresolvedName` diagnostic for the genuinely missing helper `makeClass`.
TypeScript reports TS2304 for the same identifier in both files.

## Problem

Reference test results originally showed 2 cases failing in directory
`commentsOnObjectLiteral-name-resolution` with diagnostics: name-resolution.
Fresh focused triage on 2026-05-07 shows the object literal syntax parses and
the current failure is the expected missing-name diagnostic.

Problem: `commentsOnObjectLiteral1.ts` and `commentsOnObjectLiteral2.ts` are
invalid-source references in the current runner view; both call `makeClass`
without declaring it.

## Current failure

Representative reproductions:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOnObjectLiteral1.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOnObjectLiteral2.ts
```

Coverage windows:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsOnObjectLiteral1.ts --detail --no-dashboard-data
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsOnObjectLiteral2.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
commentsOnObjectLiteral1.ts: UnresolvedName: unresolved name: `makeClass` at 60..69
commentsOnObjectLiteral2.ts: UnresolvedName: unresolved name: `makeClass` at 79..88
coverage: unsupported_diagcodes=UnresolvedName:1 for each representative
```

TypeScript oracle:

```text
commentsOnObjectLiteral1.ts: TS2304 Cannot find name 'makeClass'.
commentsOnObjectLiteral2.ts: TS2304 Cannot find name 'makeClass'.
```

## Desired final state

This generated bucket is closed as superseded by issue 056's name-resolution
behavior: unresolved identifiers should produce source-spanned `UnresolvedName`
diagnostics when they are genuinely missing. Do not implement directly from
this bucket.

## Scope

In scope:

- [x] Inspect the smart triage reports below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 056's unresolved-name diagnostic behavior
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

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
- [x] Superseding issue contains the exact unresolved-name diagnostic family
- [x] This issue includes failing paths, diagnostic code, source context, visible symbols, parser/TypeScript AST evidence, and TypeScript oracle evidence
- [x] Completion evidence names the exact reference paths and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsOnObjectLiteral1.ts --detail --no-dashboard-data
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsOnObjectLiteral2.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOnObjectLiteral1.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOnObjectLiteral2.ts
```

Not run:

- `cargo fmt --all --check` (issue metadata only)
- `cargo nextest run` (issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentsOnObjectLiteral1.ts`
- `reference/typescript/tests/cases/compiler/commentsOnObjectLiteral2.ts`

## Duplicate detection

- `issues/open/056-implement-name-resolution.md` established source-spanned
  `UnresolvedName` as the correct compiler diagnostic for genuinely unresolved
  identifiers.
- Broad generated buckets `issues/open/064-implement-name-resolution.md`,
  `issues/open/437-implement-name-resolution.md`, and meta issue 5005 are not
  narrower implementation orders for this oracle-matching invalid-source case.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage name resolution: commentsOnObjectLiteral1

- Issue class: triage-needed
- Feature label: name-resolution
- Diagnostic: UnresolvedName / resolver-symbol
- Path: reference/typescript/tests/cases/compiler/commentsOnObjectLiteral1.ts
- Failure: unresolved name: `makeClass` at 60..69
```

Source context:

```text
1 | // @target: es2015
2 | // @removeComments: false
3 | var Person = makeClass(
4 |    /**
5 |      @scope Person
6 |    */
```

Compiler evidence:

```text
tokens: ok through var Person, call makeClass, and empty object literal
ast: ok; Let Person = Call(Ident("makeClass"), [Object props=[]])
resolved: UnresolvedName for makeClass
TypeScript oracle: TS2304 Cannot find name 'makeClass'
```

```text
### Smart triage: Triage name resolution: commentsOnObjectLiteral2

- Issue class: triage-needed
- Feature label: name-resolution
- Diagnostic: UnresolvedName / resolver-symbol
- Path: reference/typescript/tests/cases/compiler/commentsOnObjectLiteral2.ts
- Failure: unresolved name: `makeClass` at 79..88
```

Source context:

```text
4 | var Person = makeClass(
5 |    {
6 |        /**
7 |         This is just another way to define a constructor.
```

Compiler evidence:

```text
tokens: ok through object literal method-like function expression
ast: ok; Object has initialize: FunctionExpr with this.name = name
resolved: UnresolvedName for makeClass
TypeScript oracle: TS2304 Cannot find name 'makeClass'
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsOnObjectLiteral1.ts --detail --no-dashboard-data
result: build_pass=0, unsupported=1, unsupported_diagcodes=UnresolvedName:1
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsOnObjectLiteral2.ts --detail --no-dashboard-data
result: build_pass=0, unsupported=1, unsupported_diagcodes=UnresolvedName:1
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOnObjectLiteral1.ts
result: oracle-matching UnresolvedName/TS2304 for makeClass
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOnObjectLiteral2.ts
result: oracle-matching UnresolvedName/TS2304 for makeClass
date: 2026-05-07
```

Remaining risks:

- Coverage still counts oracle-matching TypeScript diagnostics as unsupported
  in this build-only view; this issue closure only removes the stale generated
  implementation blocker.
