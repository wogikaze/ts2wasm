---
id: 3542
title: "Implement Noimplicitanyindexing"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5284]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as a generated bucket. Fresh evidence shows the current blocker is the
existing plain enum binding issue 5284.

## Problem

Fresh triage shows this fixture parses through the plain enum declaration,
enum-indexing expressions, object indexing, and index signature interface. Name
resolution then fails at the first use of the plain enum:

```text
UnresolvedName: unresolved name: `MyEmusEnum` at 147..157
```

Problem: this generated bucket is superseded by issue 5284, which owns plain
enum declaration binding before member or index access.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyIndexing.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyIndexing.ts --detail --no-dashboard-data
```

Observed 2026-05-08:

```text
coverage: executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnresolvedName:1 unsupported_features=name-resolution:1
triage: UnresolvedName unresolved name: `MyEmusEnum` at 147..157
```

## Desired final state

This generated bucket is closed as superseded by
`issues/open/5284-bind-plain-enum-declarations-before-member-access.md`. Do not
implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Fold into existing issue 5284 for the same observable behavior
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
- [x] Existing issue 5284 contains the implementation owner; this done issue contains the exact focused triage command
- [x] Evidence includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Issue 5284 acceptance covers the enum-binding diagnostic family; indexing follow-up must be re-triaged after 5284 advances

## Validation

Required commands:

```sh
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyIndexing.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyIndexing.ts
```

Not run:

- `cargo fmt --all --check`; metadata-only issue fold.
- `cargo nextest run`; metadata-only issue fold.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] folded into: `issues/open/5284-bind-plain-enum-declarations-before-member-access.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noImplicitAnyIndexing.ts`

## Duplicate detection

- `issues/open/5284-bind-plain-enum-declarations-before-member-access.md` is
  the exact owner for plain enum declarations being omitted before later
  enum member or index access.
- `issues/open/5184-parse-const-enum-declarations.md` covers `const enum`, not
  this plain enum declaration.
- `issues/open/5277-parse-export-enum-declarations-to-enum-boundary.md` covers
  `export enum`, not this plain top-level enum declaration.
- Folded into issue 5284.

## Smart triage

### Smart triage: Triage name resolution: noImplicitAnyIndexing

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/noImplicitAnyIndexing.ts`

Current compiler message:

```text
unresolved name: `MyEmusEnum` at 147..157
```

Source context:

```text
4 | enum MyEmusEnum {
5 |     emu
6 | }
9 | var strRepresentation1 = MyEmusEnum[0]
```

Compiler evidence:

```text
tokens: ok through enum declaration and later enum/object/index-signature indexing forms
ast: ok but enum declaration is omitted; first statement is strRepresentation1 = MyEmusEnum[0]
resolved: UnresolvedName for MyEmusEnum at first use
```

TypeScript oracle:

```text
diagnostics=[]
AST includes EnumDeclaration `enum MyEmusEnum { emu }`
strRepresentation1: string
```

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

- none
