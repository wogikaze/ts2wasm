---
id: 1404
title: "Implement Compoundvardecl"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1404.

## Summary

Closed as a stale generated bucket after fresh triage showed the representative
now builds successfully.

`compoundVarDecl1.ts` no longer reports the generated `import-export` blocker.
Current focused coverage reports `build_pass=1`, `unsupported=0`, and
`blocked=0`.

## Problem

Reference test results originally showed 1 case failing in directory
`compoundVarDecl` with diagnostics: import-export. Fresh focused triage on
2026-05-07 reports `BuildPass`.

Problem: the generated blocker is stale; there is no current compiler or
TypeScript-oracle diagnostic to split into an implementation-ready child issue.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/compoundVarDecl1.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/compoundVarDecl1.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
Smart triage: Build pass: compoundVarDecl1
coverage: executed=1, build_pass=1, unsupported=0, blocked=0
semantic_enabled=0
```

TypeScript oracle evidence:

```text
typescriptVersion: 6.0.3
diagnostics: []
```

## Desired final state

This generated bucket is closed. No implementation issue is created because the
current compiler build and TypeScript oracle both accept the representative
source.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close this stale generated bucket
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Creating a child issue without a current failing diagnostic

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
- [x] Focused triage reports `BuildPass`
- [x] Focused coverage reports `build_pass=1`
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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/compoundVarDecl1.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/compoundVarDecl1.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/compoundVarDecl1.ts`

## Duplicate detection

Fresh smart triage found only this same issue as a duplicate candidate. No
matching implementation-ready issue is needed because the current build passes.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Build pass: compoundVarDecl1

- Issue class: none
- Feature label: build-pass
- Diagnostic: BuildPass / pass
- Path: reference/typescript/tests/cases/compiler/compoundVarDecl1.ts
```

Source context:

```ts
namespace Foo { var a = 1, b = 1; a = b + 2; }

var foo = 4, bar = 5;
```

Compiler evidence:

```text
tokens: ok; includes namespace Foo, compound var declarations, assignment, and top-level var foo/bar
ast: ok; top-level compound var decl lowers to Let foo and Let bar
resolved: ok; Let foo and Let bar
```

TypeScript oracle evidence:

```text
diagnostics: []
hints include bindings a, b, foo, bar and binary expression b + 2
```

Coverage result:

```text
executed=1
build_pass=1
unsupported=0
blocked=0
semantic_enabled=0
```

## Completion evidence

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/compoundVarDecl1.ts
result: pass; BuildPass, no compiler blocker found
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/compoundVarDecl1.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0, blocked=0
date: 2026-05-07
```

Remaining risks:

- `semantic_enabled=0` for this focused tsc coverage run, but the TypeScript
  oracle has no diagnostics for this representative path.
