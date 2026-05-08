---
id: 1035
title: "Implement Baseconstraintofdecorator"
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

Triage baseConstraintOfDecorator across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `baseConstraintOfDecorator` with diagnostics: import-export. Fresh smart triage shows the current blocker is the entry-module `export function classExtender(...)` boundary already split to issue 5144.

Problem: `baseConstraintOfDecorator` is not a standalone implementation order; the observed `export function` module boundary is owned by issue 5144.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseConstraintOfDecorator.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/baseConstraintOfDecorator.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by `issues/open/5144-support-entry-export-function-declarations.md`. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5144's entry-module export function slice
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
- [x] Existing child issue 5144 contains an exact reference-triage command for the same export-function boundary
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/baseConstraintOfDecorator.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/baseConstraintOfDecorator.ts
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

- [x] existing: `issues/open/5144-support-entry-export-function-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/baseConstraintOfDecorator.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: import export: baseConstraintOfDecorator

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/baseConstraintOfDecorator.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseConstraintOfDecorator.ts
```

Source context:

```text
1 | // @target: es2015
2 | // @strict: false
3 | export function classExtender<TFunction>(superClass: TFunction, _instanceModifier: (instance: any, args: any[]) => void): TFunction {
4 |     return class decoratorFunc extends superClass {
5 |         constructor(...args: any[]) {
6 |             super(...args);
```

Current compiler failure:

```text
error: [UnsupportedModule] issue-5005: entry module `export classExtender` uses a declaration form outside the current static export slice; only export const and export default are supported at 44..348
```

TypeScript oracle evidence:

```text
TypeScript parses the top-level declaration as an exported FunctionDeclaration and then reports type-system diagnostics for the decorator/mixin shape.
```

Resolution:

```text
Issue 5144 already owns the entry-module `ExportDecl(Function)` implementation slice. This bucket contributes another representative path for the same module boundary, so no duplicate child is created.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- superseded by `issues/open/5144-support-entry-export-function-declarations.md`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseConstraintOfDecorator.ts
result: pass; reproduced issue-5005 entry-module export function boundary
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

