---
id: 1197
title: "Implement Classextendsclauseclassmergedwithmodulenotreferingconstructor"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1197.

## Summary

Triage classExtendsClauseClassMergedWithModuleNotReferingConstructor across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `classExtendsClauseClassMergedWithModuleNotReferingConstructor` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: classExtendsClauseClassMergedWithModuleNotReferingConstructor has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExtendsClauseClassMergedWithModuleNotReferingConstructor.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsClauseClassMergedWithModuleNotReferingConstructor.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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
- [x] At least one child issue contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsClauseClassMergedWithModuleNotReferingConstructor.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExtendsClauseClassMergedWithModuleNotReferingConstructor.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5314-report-non-constructor-local-class-heritage.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classExtendsClauseClassMergedWithModuleNotReferingConstructor.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated on 2026-05-07.

Fresh focused coverage shows this generated import/export bucket is stale:

```text
executed=1
build_pass=1
unsupported=0
reference/typescript/tests/cases/compiler/classExtendsClauseClassMergedWithModuleNotReferingConstructor.ts: build_pass
```

Representative triage:

```text
ts2wasm: BuildPass
TypeScript oracle:
TS2564 Property 'a' has no initializer and is not definitely assigned in the constructor.
TS2507 Type 'number' is not a constructor function type.
TS2564 Property 'b' has no initializer and is not definitely assigned in the constructor.
```

Source context:

```ts
class A {
    a: number;
}
namespace A {
    export var v: string;
}

namespace Foo {
    var A = 1;
    class B extends A {
        b: string;
    }
}
```

Compiler evidence:

```text
tokens: ok for class A, namespace A export var v, namespace Foo var A, and class B extends A
ast: ok; current AST dump contains only top-level ClassDecl A after namespace erasure
resolved: ok for top-level ClassDecl A
```

Split child: `issues/open/5314-report-non-constructor-local-class-heritage.md`.

Related issues are no-match for this exact residual semantic diagnostic:

- Issue 5256 handles member-expression heritage diagnostics such as `extends "".bogus`.
- Issue 5225 handles qualified class heritage implementation such as `extends Foo.Object`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsClauseClassMergedWithModuleNotReferingConstructor.ts
result: pass; current compiler build-passes, TypeScript oracle reports TS2507 for local non-constructor heritage, split to issue 5314
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsClauseClassMergedWithModuleNotReferingConstructor.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0
date: 2026-05-07
```

Remaining risks:

- Strict property initialization diagnostics TS2564 remain outside issue 5314.
