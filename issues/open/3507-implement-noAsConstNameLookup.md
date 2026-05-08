---
id: 3507
title: "Implement Noasconstnamelookup"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5232]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Triage noAsConstNameLookup across 1 failing reference test cases and split this bucket into implementation-ready child issues.

Closed as superseded by
`issues/open/5232-w0-fixture-ize-runtimelinkplan-linker-structure-tests.md`. Fresh triage
shows the current first blocker is the entry-module `export class`
issue-5005 boundary after the parser accepts the `as const` portions.

## Problem

Reference test results show 1 cases fail in directory `noAsConstNameLookup` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: noAsConstNameLookup currently reaches the existing entry-module
`export class` issue-5005 boundary before no-as-const name lookup behavior is
actionable.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/noAsConstNameLookup.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/noAsConstNameLookup.ts --detail
```

## Desired final state

This generated bucket is closed. Implement the current blocker through
`issues/open/5232-w0-fixture-ize-runtimelinkplan-linker-structure-tests.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5232
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
- [x] Existing issue 5232 owns the current entry-module `export class` issue-5005 boundary
- [x] This closure includes failing path, diagnostic code, source context,
  visible symbols, parser/resolved evidence, and TypeScript AST evidence
- [x] No child issue is needed from 3507 because the current blocker is already implementation-ready in issue 5232

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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/noAsConstNameLookup.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/noAsConstNameLookup.ts
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

- [x] none; superseded by issue 5232

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noAsConstNameLookup.ts`

## Duplicate detection

- `issues/open/5232-w0-fixture-ize-runtimelinkplan-linker-structure-tests.md` owns the
  current entry-module `export class` issue-5005 boundary.
- `issues/done/059a-implement-typescript-satisfies-and-const-assertion-erasure.md`
  owns the completed parser erasure for `as const` and `<const>`, so the
  current first blocker is not a new const-assertion parser slice.

## Smart triage

Fresh run on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noAsConstNameLookup.ts
```

Coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noAsConstNameLookup.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=type-assertion:1
reference/typescript/tests/cases/compiler/noAsConstNameLookup.ts: UnsupportedSyntax: type-assertion
```

The coverage feature label follows the path-level `asConst` classifier, but
the smart-triage compiler dump reaches a more precise current first blocker:

```text
UnsupportedModule: issue-5005: entry module `export FeatureRunner` uses a
declaration form outside the current static export slice; only export const and
export default are supported at 182..613
```

Source context:

```ts
export class FeatureRunner<W extends Store> {
    private readonly cleaners: Cleaner[] = []

    async runFeature(): Promise<any> {
        const objectWhichShouldBeConst = {
            flags: {},
            settings: {},
        } as const;
        return objectWhichShouldBeConst
    }
}
```

Compiler evidence:

```text
tokens: ok; includes export type, export class FeatureRunner, `as const`,
export class C, and `new C<string>().f()`
ast: ok; ExportDecl(ClassDecl FeatureRunner), ExportDecl(ClassDecl C), and
call expression `new C<string>().f()`
resolved: ok; ClassDecl FeatureRunner, ClassDecl C with method f, and
MethodCall on New C
module build: issue-5005 entry-module export class boundary
```

TypeScript oracle:

```text
ok=true
diagnostics=[]
AST topLevel includes TypeAliasDeclaration, export type Cleaner,
ClassDeclaration export FeatureRunner, ClassDeclaration export C, and
ExpressionStatement `new C<string>().f();`.
binding hint: objectWhichShouldBeConst has readonly flags/settings from
`as const`; one has literal type 1.
```

## Completion evidence

Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noAsConstNameLookup.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnsupportedSyntax/type-assertion
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noAsConstNameLookup.ts
result: pass; current issue-5005 entry export-class blocker is superseded by issue 5232
date: 2026-05-08
```

Remaining risks:

- After issue 5232 lands, this reference may expose no-as-const name lookup
  parity, async method lowering, readonly/private class member handling, or
  remaining generic-call erasure details.
