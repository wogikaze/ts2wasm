---
id: 1196
title: "Implement Classextendsacrossfiles"
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
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1196.

## Summary

Triage classExtendsAcrossFiles across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `classExtendsAcrossFiles` with diagnostics: arrow-function. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: classExtendsAcrossFiles has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExtendsAcrossFiles.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsAcrossFiles.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with existing issue 5229
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
- [x] This issue contains an exact `reference-triage` command
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsAcrossFiles.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExtendsAcrossFiles.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by existing `issues/open/5229a-resolve-imports-between-filename-sections.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classExtendsAcrossFiles.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated on 2026-05-07.

Fresh triage shows this generated arrow-function bucket has advanced past the
arrow/function parser surface. The first blocker is the known virtual
`@filename` module graph boundary owned by issue 5229.

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsAcrossFiles.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsAcrossFiles.ts --detail --no-dashboard-data
```

Observed coverage:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
```

Source context:

```ts
// @filename: a.ts
import { b } from './b';
export const a = {
    f: () => {
        class A { }
        class B extends A { }
        b.f();
    }
};
// @filename: b.ts
import { a } from './a';
```

Compiler evidence:

```text
tokens: ok; includes import { b }, export const a, arrow function, nested class A, nested class B extends A, and b.f()
ast: ok; ImportNamed("./b"), ExportDecl const a with object literal property f as ArrowFn, nested class declarations, and b.f() call
module_graph/resolved dump: issue-232 missing local module `./b`; tried on-disk ./b.ts, ./b.js, ./b.d.ts, ./b.tsx, ./b.mjs, ./b.cjs
coverage: UnsupportedModule/import-export
```

Smart triage header also reports `UnresolvedName` for `b` in the raw
single-file pass, but the build/coverage blocker is module graph resolution for
the virtual sibling file.

TypeScript oracle evidence:

```text
TS2307 Cannot find module './b' or its corresponding type declarations.
TS2307 Cannot find module './a' or its corresponding type declarations.
TS2395 merged declaration export/local errors for a and b in the raw single-file oracle.
```

Resolution:

```text
Issue 5229 owns registering TypeScript reference `@filename` sections as virtual module paths and resolving local imports between those sections. The current failure is that exact module materialization boundary, not a standalone class inheritance or arrow-function implementation slice.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsAcrossFiles.ts
result: pass; current blocker is issue-232 missing virtual local module `./b`, superseded by issue 5229
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsAcrossFiles.ts --detail --no-dashboard-data
result: pass; unsupported=1, UnsupportedModule/import-export for virtual @filename module resolution
date: 2026-05-07
```

Remaining risks:

- After issue 5229 resolves virtual `a.ts`/`b.ts` imports, this path may expose circular module evaluation or class-in-arrow runtime behavior.
