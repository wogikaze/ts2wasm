---
id: 3573
title: "Implement Nounusedlocals Name Resolution"
type: spike
area: frontend/resolver
class: blocked
priority: P1
depends_on: [5005]
blocks: [5482]
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage noUnusedLocals-name-resolution across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh triage shows the fixture tokenizes and builds an AST, but the first
destructuring assignment is represented as assignment to a synthetic name:

```text
Assign { name: "[x]", expr: Array([1]) }
UnresolvedName: unresolved name: `[x]` at 214..224
```

Problem: noUnusedLocals-name-resolution has an array destructuring assignment
representation gap now tracked by issue 5482.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnly.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnly.ts --detail
```

## Desired final state

This generated bucket is closed as folded into
`issues/open/5482-represent-array-destructuring-assignment-statements.md`. Do not
implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split the array destructuring assignment behavior into child issue 5482
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

- [x] Duplicate candidates below are confirmed as no-match or superseded
- [x] Existing issue 5482 contains the implementation owner
- [x] Evidence includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Issue 5481 acceptance names the exact fixture/reference path and diagnostic/stdout change

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnly.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnly.ts
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

- [x] folded into: `issues/open/5482-represent-array-destructuring-assignment-statements.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnly.ts`

## Duplicate detection

- `issues/open/064-implement-name-resolution.md` - Implement name resolution (triaged - superseded by test262 metadata issues) (same feature label, title overlap)
- `issues/open/437-implement-name-resolution.md` - Implement name resolution (same feature label, title overlap)
- `issues/open/648-implement-argumentsAsPropertyName-name-resolution.md` - Implement Argumentsaspropertyname Name Resolution (same feature label, title overlap)
- `issues/open/654-implement-argumentsReferenceInConstructor-name-resolution.md` - Implement Argumentsreferenceinconstructor Name Resolution (same feature label, title overlap)
- `issues/open/657-implement-argumentsReferenceInMethod-name-resolution.md` - Implement Argumentsreferenceinmethod Name Resolution (same feature label, title overlap)
- `issues/open/693-implement-arrayToLocaleStringES-name-resolution.md` - Implement Arraytolocalestringes Name Resolution (same feature label, title overlap)
- `issues/open/733-implement-assignmentCompatability-name-resolution.md` - Implement Assignmentcompatability Name Resolution (same feature label, title overlap)

## Smart triage

### Smart triage: Triage name resolution: noUnusedLocals writeOnly

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnly.ts`

Current compiler message:

```text
unresolved name: `[x]` at 214..224
```

Source context:

```text
 8 |     x = 1;
 9 |     ([x] = [1]);
10 |     ({ x } = { x: 1 });
11 |     ({ x: x } = { x: 1 });
12 |     ({ a: [{ b: x }] } = { a: [{ b: 1 }] });
```

Compiler evidence:

```text
tokens: ok through array, object, aliased, nested, and default destructuring assignments
ast: ok but `([x] = [1])` becomes Assign { name: "[x]", expr: Array([1]) }
resolved: UnresolvedName for synthetic name `[x]`
```

TypeScript oracle:

```text
diagnostics=[]
parameter x: number
parameter b: boolean
```

## Completion evidence

Status: done

Commits:

- this local issue-cleanup commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnly.ts --detail --no-dashboard-data
result: pass; representative path reports UnresolvedName/name-resolution for synthetic destructuring target `[x]`
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnly.ts
result: pass; fresh triage split the array destructuring assignment representation gap to issue 5482
date: 2026-05-08
```

Remaining risks:

- Later object, nested, default destructuring writes and noUnusedLocals semantic checks remain hidden until issue 5481 advances.
