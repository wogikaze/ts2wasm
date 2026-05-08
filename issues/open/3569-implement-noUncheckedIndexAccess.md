---
id: 3569
title: "Implement Nouncheckedindexaccess"
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

Triage noUncheckedIndexAccess across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh triage shows this fixture parses through the plain enum declarations and
indexing forms, then fails name resolution because the first enum declaration
does not create a binding:

```text
UnresolvedName: unresolved name: `Meat` at 164..168
```

Problem: this generated bucket is superseded by issue 5284, which owns plain
enum declaration binding before member or index access.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/noUncheckedIndexAccess.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/noUncheckedIndexAccess.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by
`issues/done/5284-bind-plain-enum-declarations-before-member-access.md`. Do
not implement directly from this bucket.

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
- [x] Issue 5284 acceptance covers the enum-binding diagnostic family; noUncheckedIndexedAccess semantics can be re-triaged after 5284 advances

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noUncheckedIndexAccess.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noUncheckedIndexAccess.ts
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

- [x] folded into: `issues/done/5284-bind-plain-enum-declarations-before-member-access.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noUncheckedIndexAccess.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage name resolution: noUncheckedIndexAccess

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/noUncheckedIndexAccess.ts`

Current compiler message:

```text
unresolved name: `Meat` at 164..168
```

Source context:

```text
 6 | enum Meat {
 7 |     Sausage,
 8 |     Bacon
 9 |   }
10 |   const sausage = Meat.Sausage
11 |   const valueSausage = Meat[sausage]
```

Compiler evidence:

```text
tokens: ok through enum declarations, member accesses, and enum index accesses
ast: ok but enum declarations are omitted; first statement is sausage = Meat.Sausage
resolved: UnresolvedName for Meat at the first enum member access
```

TypeScript oracle:

```text
diagnostics=[]
AST includes EnumDeclaration `enum Meat { Sausage, Bacon }`
valueSausage: string
valueBacon: string
valueUnion: string
value: string
value2: any
value3: string
```

## Completion evidence

Status: done

Commits:

- this local issue-cleanup commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noUncheckedIndexAccess.ts --detail --no-dashboard-data
result: pass; representative path reports UnresolvedName/name-resolution for Meat
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noUncheckedIndexAccess.ts
result: pass; fresh triage shows the same omitted plain enum binding boundary owned by issue 5284
date: 2026-05-08
```

Remaining risks:

- noUncheckedIndexedAccess-specific type behavior remains hidden until plain enum binding advances in issue 5284.
