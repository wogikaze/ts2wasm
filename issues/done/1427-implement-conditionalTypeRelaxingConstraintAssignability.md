---
id: 1427
title: "Implement Conditionaltyperelaxingconstraintassignability"
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

## Summary

Triage conditionalTypeRelaxingConstraintAssignability across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory
`conditionalTypeRelaxingConstraintAssignability` with diagnostics:
import-export. Fresh triage on 2026-05-07 shows the current first blocker is the
entry-module `export class` boundary already tracked by issue 5232.

Problem: conditionalTypeRelaxingConstraintAssignability has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conditionalTypeRelaxingConstraintAssignability.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conditionalTypeRelaxingConstraintAssignability.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with existing issue 5232
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
- [x] Superseding issue 5232 contains an exact export-class boundary and acceptance names the issue-5005 diagnostic change
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Coverage names the exact reference path and current stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conditionalTypeRelaxingConstraintAssignability.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conditionalTypeRelaxingConstraintAssignability.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; superseded by `issues/done/5232-w0-fixture-ize-runtimelinkplan-linker-structure-tests.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/conditionalTypeRelaxingConstraintAssignability.ts`

## Duplicate detection

- Superseded by `issues/done/5232-w0-fixture-ize-runtimelinkplan-linker-structure-tests.md`.
  The current 1427 first blocker is the same entry-module `ExportDecl(ClassDecl)`
  issue-5005 boundary that 5232 owns.
- Generic import/export duplicate candidates are no-match because they cover
  unrelated module/export forms or older issue-055 parser boundaries.

## Smart triage

Generated on 2026-05-07:

```text
Feature label: import-export
Diagnostic: UnsupportedModule / unsupported-feature-boundary
Path: reference/typescript/tests/cases/compiler/conditionalTypeRelaxingConstraintAssignability.ts
Failure: issue-5005: entry module `export Elem` uses a declaration form outside the current static export slice; only export const and export default are supported at 300..408
line: 14, column: 1
Visible symbols before failure: class Elem at line 13, column 8
```

Source context:

```text
13 | export class Elem<
14 |   C extends ElChildren,
15 |   > {
16 |   constructor(
17 |     private children_: Relax<C>,
```

Compiler evidence:

```text
tokens: ok
ast: ok; contains ExportDecl(ClassDecl { name: "Elem", ... })
module build: UnsupportedModule issue-5005 for entry-module export class
resolved dump also exposes a later UnresolvedName for ElChildren after the module boundary
```

TypeScript oracle evidence:

```text
TypeScript diagnostics: none
Top-level AST includes:
- TypeAliasDeclaration export type ElChildren
- ModuleDeclaration export namespace ElChildren
- TypeAliasDeclaration Relax
- ClassDeclaration export class Elem<...>
- later expressions, interface, type alias, and functions
```

Coverage result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/conditionalTypeRelaxingConstraintAssignability.ts --detail --no-dashboard-data
result: executed=1, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=type-system:1, build_pass=0, semantic_pass=0, blocked=0
date: 2026-05-07
note: smart triage identifies the first concrete compiler diagnostic as UnsupportedModule issue-5005 at entry-module export class.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Closed as superseded by issue 5232; no child issue created.

Validation result:

```text
command: python scripts/manager.py update-issue-index && python scripts/manager.py update-issue-index --check && python scripts/manager.py check-issue-health && python scripts/manager.py check-issue-readiness -- --fail-ready-below 80 && git diff --check
result: pass
date: 2026-05-07
```

Remaining risks:

- none
