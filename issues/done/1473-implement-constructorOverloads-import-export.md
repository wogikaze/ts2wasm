---
id: 1473
title: "Implement Constructoroverloads Import Export"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [432]
blocks: [5357, 5232, 5334]
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Closed by splitting the current representative blocker to
`issues/open/5357-avoid-eval-diagnostic-for-qualified-function-constructors.md`
and mapping the remaining affected files to existing owners.

Fresh coverage shows the original import/export bucket is stale: one affected
file now build-passes, one exposes a false eval diagnostic for `M.Function`, and
one reaches the existing `export class` / constructor-overload boundaries.

## Problem

Reference test results originally showed 3 import/export failures. Fresh
focused triage on 2026-05-07 shows mixed current blockers:

- `constructorOverloads5.ts`: build pass, TypeScript oracle ok
- `constructorOverloads4.ts`: false issue-062 dynamic Function diagnostic for `new M.Function(...)`
- `constructorOverloads9.ts`: issue-5005 entry-module `export class`, then resolved dump reaches `DuplicateFunction` for constructor overload signatures

Problem: `constructorOverloads-import-export` needed focused current ownership
instead of a broad generated import/export bucket.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads5.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorOverloads --detail --no-dashboard-data
```

Observed 2026-05-07 for the affected files:

```text
constructorOverloads5.ts: build_pass
constructorOverloads4.ts: UnsupportedEval issue-062 at new M.Function("return 5")
constructorOverloads9.ts: UnsupportedModule issue-5005 for entry-module export class
```

Additional dump for `constructorOverloads9.ts`:

```text
ast: ExportDecl(ClassDecl C) with bodyless constructor signature plus implementation
resolved/module build: issue-5005 entry-module export class
resolved dump after module path: DuplicateFunction duplicate constructor definition
```

## Desired final state

This generated bucket is closed. Implementation proceeds through the focused
child and existing owners named below.

## Scope

In scope:

- [x] Inspect fresh triage for representative and affected files
- [x] Confirm whether existing open/done issues already cover each current blocker
- [x] Split the false eval diagnostic into child issue 5357
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Broad import/export module support
- Dynamic eval implementation

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
- [x] Child issue 5357 contains exact `reference-triage` command for `constructorOverloads4.ts`
- [x] Existing issue 5232 owns the entry-module `export class` boundary for `constructorOverloads9.ts`
- [x] Existing issue 5334 owns the constructor overload signature `DuplicateFunction` boundary after export-class advances
- [x] `constructorOverloads5.ts` is recorded as stale build-pass with TypeScript oracle ok

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorOverloads --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads4.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads5.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads9.ts
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

- [x] created: `issues/open/5357-avoid-eval-diagnostic-for-qualified-function-constructors.md`
- [x] existing: `issues/done/5232-w0-fixture-ize-runtimelinkplan-linker-structure-tests.md`
- [x] existing: `issues/open/5334-parse-class-constructor-overload-signatures.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constructorOverloads4.ts`
- `reference/typescript/tests/cases/compiler/constructorOverloads5.ts`
- `reference/typescript/tests/cases/compiler/constructorOverloads9.ts`

## Duplicate detection

- `issues/done/5232-w0-fixture-ize-runtimelinkplan-linker-structure-tests.md` owns the
  issue-5005 entry-module `export class C` blocker.
- `issues/open/5334-parse-class-constructor-overload-signatures.md` owns the
  `DuplicateFunction: duplicate constructor definition` blocker exposed after
  the module-export boundary.
- No exact open issue was found for the qualified `M.Function` false eval
  diagnostic, so issue 5357 was created.

## Smart triage

Generated 2026-05-07 for files 4, 5, and 9.

```text
constructorOverloads4.ts: UnsupportedEval issue-062 for new M.Function
constructorOverloads5.ts: BuildPass and TypeScript oracle ok
constructorOverloads9.ts: UnsupportedModule issue-5005 for export class
```

## Completion evidence

Commits:

- filled by local commit that moves this issue to `done/`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorOverloads --detail --no-dashboard-data
result: pass; group evidence collected, affected files mapped to build_pass / issue-062 / issue-5005
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads4.ts
result: pass; reproduced false issue-062 for qualified M.Function
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads5.ts
result: pass; BuildPass with TypeScript oracle ok
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads9.ts
result: pass; reproduced issue-5005 export class and DuplicateFunction constructor overload evidence
date: 2026-05-07
```

Remaining risks:

- implementation remains tracked by issues 5357, 5232, and 5334
