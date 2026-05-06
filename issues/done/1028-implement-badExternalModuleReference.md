---
id: 1028
title: "Implement Badexternalmodulereference"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage badExternalModuleReference across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `badExternalModuleReference` with diagnostics: import-export. Fresh smart triage shows the compiler reaches the module graph and emits the existing issue-232 unsupported non-local module specifier diagnostic for `require("garbage")`.

Problem: `badExternalModuleReference` is not a standalone implementation order; the observed behavior is covered by issue 232's accepted non-local module boundary.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badExternalModuleReference.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/badExternalModuleReference.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by `issues/done/232-resolve-local-relative-es-module-graph.md`. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 232's non-local module specifier boundary
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue

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
- [x] This closed issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] This closed issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/badExternalModuleReference.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/badExternalModuleReference.ts
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

- `reference/typescript/tests/cases/compiler/badExternalModuleReference.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: import export: badExternalModuleReference

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/badExternalModuleReference.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badExternalModuleReference.ts
```

Source context:

```text
1 | // @target: es2015
2 | //@module: commonjs
3 | import a1 = require("garbage");
4 | export declare var a: {
5 |     test1: a1.connectModule;
6 |     (): a1.connectExport;
```

Current compiler failure:

```text
[pipeline] validate_ast
[pipeline] module_graph
error: [UnsupportedModule] issue-232: unsupported non-local module specifier `garbage`; package resolution, import maps, and absolute specifiers are not implemented at 61..70
```

TypeScript oracle evidence:

```text
TS2307: Cannot find module 'garbage' or its corresponding type declarations.
AST path: ImportEqualsDeclaration -> ExternalModuleReference -> StringLiteral `"garbage"`.
```

Resolution:

```text
Issue 232 deliberately rejects bare/non-local module specifiers with source-spanned UnsupportedModule diagnostics. The current reference path hits that existing policy boundary, so no child implementation slice is created from this generated bucket.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- superseded by `issues/done/232-resolve-local-relative-es-module-graph.md`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badExternalModuleReference.ts
result: pass; reproduced issue-232 unsupported non-local module specifier diagnostic
date: 2026-05-06
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

