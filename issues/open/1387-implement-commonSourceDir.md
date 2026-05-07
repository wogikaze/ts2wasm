---
id: 1387
title: "Implement Commonsourcedir"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: [5285]
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1387.

## Summary

Closed as superseded by
`issues/done/5285-support-export-var-initializer-declarations.md`.

Fresh focused triage shows both `commonSourceDir5.ts` and
`commonSourceDir6.ts` currently stop at the same initialized `export var`
issue-055 boundary already owned by issue 5285.

## Problem

Reference test results originally showed 2 cases failing in directory
`commonSourceDir` with diagnostics: import-export. Fresh focused triage on
2026-05-07 shows both representative paths stop before sourceDir/module layout
semantics: the parser/module frontend rejects `export var x = z + z;`.

Problem: `commonSourceDir5.ts` and `commonSourceDir6.ts` currently report
`issue-055: unsupported variable export` for an initialized `export var`
declaration. This is the same observable behavior tracked by issue 5285.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonSourceDir5.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonSourceDir6.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonSourceDir5.ts --detail --no-dashboard-data
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonSourceDir6.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
commonSourceDir5.ts: UnsupportedModule issue-055 unsupported variable export at `export var x = z + z;`
commonSourceDir6.ts: UnsupportedModule issue-055 unsupported variable export at `export var x = z + z;`
coverage: build_pass=0, unsupported=1 for each focused path
```

## Desired final state

This generated bucket is closed. Implementation proceeds through issue 5285.
After initialized export-var parsing advances, these reference paths may need
fresh triage for virtual sourceDir/outFile or module-specifier behavior.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with the existing initialized export-var issue
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
- [x] Existing issue 5285 contains the exact initialized `export var name = expr;` boundary
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonSourceDir5.ts --detail --no-dashboard-data
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonSourceDir6.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonSourceDir5.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonSourceDir6.ts
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

- [x] superseded by: `issues/done/5285-support-export-var-initializer-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commonSourceDir5.ts`
- `reference/typescript/tests/cases/compiler/commonSourceDir6.ts`

## Duplicate detection

- `issues/done/5285-support-export-var-initializer-declarations.md` owns
  initialized `export var name = expr;` declarations that currently stop at
  issue-055 unsupported variable export.
- `issues/done/5283-support-entry-export-var-declarations.md` is related but
  covers simple typed `export var name: type;`, not initialized exports.
- `issues/done/055-implement-import-export.md` is the closed import/export
  umbrella; remaining executable work is tracked by narrower child issues.

## Smart triage

Generated 2026-05-07 for both representative paths.

```text
### Smart triage: Triage import export: commonSourceDir5

- Issue class: triage-needed
- Feature label: import-export
- Diagnostic: UnsupportedModule / unsupported-feature-boundary
- Path: reference/typescript/tests/cases/compiler/commonSourceDir5.ts

### Smart triage: Triage import export: commonSourceDir6

- Issue class: triage-needed
- Feature label: import-export
- Diagnostic: UnsupportedModule / unsupported-feature-boundary
- Path: reference/typescript/tests/cases/compiler/commonSourceDir6.ts
```

Source context:

```text
// @Filename: A:/bar.ts
import {z} from "./foo";
export var x = z + z;

// @Filename: a/bar.ts
import {z} from "./foo";
export var x = z + z;
```

Compiler evidence:

```text
tokens: ok; Import, Export, Var, Ident("x"), Equal, Ident("z"), Plus, Ident("z")
ast: UnsupportedModule issue-055 unsupported variable export
resolved: same UnsupportedModule
visible symbols before failure: none
```

TypeScript oracle:

```text
TypeScript reports later sourceDir/module diagnostics such as TS2307 for
`./foo`, `B:/baz`, `A:/bar`, `a/bar`, and `a/foo`, plus merged declaration
diagnostics. The current compiler stops before those later checks because it
rejects the initialized export-var declaration.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonSourceDir5.ts --detail --no-dashboard-data
result: build_pass=0, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonSourceDir6.ts --detail --no-dashboard-data
result: build_pass=0, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonSourceDir5.ts
result: issue-055 unsupported variable export; superseded by issue 5285
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonSourceDir6.ts
result: issue-055 unsupported variable export; superseded by issue 5285
date: 2026-05-07
```

Remaining risks:

- After issue 5285 advances this path, fresh triage may reveal sourceDir,
  outFile/outDir, non-local specifier, or virtual-section import resolution
  blockers.
