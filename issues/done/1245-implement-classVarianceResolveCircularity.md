---
id: 1245
title: "Implement Classvarianceresolvecircularity"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: [5226]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Closed as superseded by
`issues/done/5226-w0-ast-node-span-requirement.md`. Fresh triage
shows both references now parse and stop at the same ambient `declare function`
overload `DuplicateFunction` boundary owned by issue 5226.

## Problem

Reference test results previously showed 2 cases failing in directory
`classVarianceResolveCircularity` with diagnostics: parser-syntax. Fresh triage
shows both cases now parse and reach duplicate-function validation.

Problem: both `classVarianceResolveCircularity` references report
`DuplicateFunction` for ambient `declare function callme(...)` overload
signatures, already tracked by issue 5226.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classVarianceResolveCircularity1.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classVarianceResolveCircularity --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing issue 5226 owns ambient `declare function` overload declarations
- [x] Close this generated bucket as superseded rather than duplicating a child issue
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

- [x] Duplicate candidates below are confirmed and this issue is superseded by 5226
- [x] No child issue needed because current blocker is already tracked
- [x] This issue includes failing paths, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference paths and superseding issue

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classVarianceResolveCircularity1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classVarianceResolveCircularity1.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/done/5226-w0-ast-node-span-requirement.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classVarianceResolveCircularity1.ts`
- `reference/typescript/tests/cases/compiler/classVarianceResolveCircularity2.ts`

Source context:

```ts
declare function callme(x: Bar<any>): Bar<any>;
declare function callme(x: object): string;
```

```ts
declare function callme(x: Foo<any>): Foo<any>;
declare function callme(x: object): string;
```

## Duplicate detection

- `issues/done/5226-w0-ast-node-span-requirement.md` is the
  exact owner for multiple bodyless ambient `declare function` declarations with
  the same name.
- `issues/done/5200-validate-top-level-function-overload-implementations.md` is
  related but covers non-ambient overload signatures with implementation bodies.
- `issues/done/5199-report-function-overload-list-class-merge-diagnostics.md`
  is related but covers function overload lists merged with classes.

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classVarianceResolveCircularity --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classVarianceResolveCircularity1.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classVarianceResolveCircularity2.ts
```

Observed result on 2026-05-06:

```text
coverage: unsupported=2
unsupported_diagcodes: DuplicateFunction:2
unsupported_features: duplicate-function:2

classVarianceResolveCircularity1.ts:
Diagnostic: DuplicateFunction
Message: duplicate function definition: `callme` at 233..239
Source: declare function callme(x: object): string;
tokens: ok
AST: ok; ClassDecl Bar plus two ambient Function callme declarations
resolved: fails in validate_ast on second ambient declaration
TypeScript oracle: ok, diagnostics=[]

classVarianceResolveCircularity2.ts:
Diagnostic: DuplicateFunction
Message: duplicate function definition: `callme` at 273..279
Source: declare function callme(x: object): string;
tokens: ok
AST: ok; ExportNamed, ClassDecl Bar, two ambient Function callme declarations, ClassDecl Foo
resolved: fails in validate_ast on second ambient declaration
TypeScript oracle: ok, diagnostics=[]

Superseding issue: 5226
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Superseded by `issues/done/5226-w0-ast-node-span-requirement.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classVarianceResolveCircularity1.ts
result: pass; current blocker is DuplicateFunction for ambient declare overloads tracked by issue 5226
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classVarianceResolveCircularity2.ts
result: pass; current blocker is DuplicateFunction for ambient declare overloads tracked by issue 5226
date: 2026-05-06
```

Remaining risks:

- none; remaining implementation work is tracked by issue 5226
