---
id: 1471
title: "Implement Constructorastype"
type: spike
area: frontend/resolver
class: superseded
priority: P1
depends_on: [5005]
blocks: [5161]
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Closed as superseded by
`issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`.

Fresh triage shows `constructorAsType.ts` currently stops at the same ambient
value name-resolution boundary: `declare var Person2` is erased from the runtime
AST, then the later assignment `Person = Person2` reports `UnresolvedName` for
`Person2`.

## Problem

Reference test results originally showed a generated name-resolution bucket.
Fresh focused triage on 2026-05-07 confirms the concrete blocker is not
constructor type syntax parsing; tokens and AST succeed, and the missing name is
the declaration-only ambient value `Person2`.

Problem: `constructorAsType.ts` is blocked by ambient `declare var`
name-resolution, already owned by issue 5161.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorAsType.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorAsType.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
constructorAsType.ts: UnresolvedName for `Person2`
coverage: executed=1, build_pass=0, unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

Source context:

```text
3 |
4 | declare var Person2:{new() : {name:string;};};
5 |
6 | Person = Person2;
```

Compiler evidence:

```text
tokens: ok through constructor function type, ambient declare var Person2, and assignment
ast: ok; retained runtime AST has Let Person and Assign Person = Ident Person2
visible symbols: Person and ambient binding Person2 are listed before failure
resolved: UnresolvedName for Person2 during resolve_names at line 6 column 15
```

TypeScript oracle evidence:

```text
TypeScript parses the declare var Person2 construct signature type and later
reports TS2322 on the initial Person assignment, not an unresolved Person2 name.
```

## Desired final state

This generated bucket is closed. Implementation proceeds through issue 5161,
which owns declaration-only ambient value declarations being resolver-visible
without emitting runtime declarations.

## Scope

In scope:

- [x] Inspect the smart triage report
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5161's ambient value name-resolution work
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Constructor/construct signature type checking after name resolution advances
- TS2322 construct-signature assignability diagnostics

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
- [x] Existing issue 5161 covers declaration-only ambient `declare var` name resolution
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorAsType.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorAsType.ts
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

- [x] superseded by: `issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constructorAsType.ts`

## Duplicate detection

- `issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`
  owns declaration-only ambient `declare var` / `declare let` / `declare const`
  names being visible to expression name resolution.
- `issues/done/5193-parse-asi-after-ambient-variable-declarations.md` is
  related to ambient variable parsing, but this file has explicit semicolons and
  already parses to AST.
- Construct-signature parser issues are related by syntax text only; this
  representative already parses the constructor type syntax successfully.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage name resolution: constructorAsType

- Issue class: triage-needed
- Feature label: name-resolution
- Diagnostic: UnresolvedName / resolver-symbol
- Path: reference/typescript/tests/cases/compiler/constructorAsType.ts
```

Resolved evidence:

```text
[pipeline] validate_ast
[pipeline] module_graph
[pipeline] resolve_names
error: [UnresolvedName] unresolved name: `Person2` at 157..164
```

## Completion evidence

Commits:

- filled by local commit that moves this issue to `done/`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorAsType.ts --detail --no-dashboard-data
result: pass; reproduced executed=1 build_pass=0 unsupported=1 UnresolvedName=1
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorAsType.ts
result: pass; reproduced UnresolvedName Person2 with parser evidence and TypeScript oracle context
date: 2026-05-07
```

Remaining risks:

- implementation remains tracked by issue 5161
