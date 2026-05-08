---
id: 3343
title: "Implement Moduleintypeposition"
type: maintenance
area: frontend/syntax
class: superseded
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed as superseded. Fresh triage for `moduleInTypePosition1.ts` reaches the
same dependency-module `export class` issue-5005 boundary already owned by
`issues/open/5324-support-dependency-export-class-declarations.md`.

## Problem

Reference test results show 1 case failing in directory `moduleInTypePosition`
with diagnostics: import-export. Fresh triage confirms this generated bucket is
not an implementation unit: the current blocker is the shared static module
export slice for dependency virtual files that start with `export class`.

Problem: `moduleInTypePosition1.ts` begins with virtual section
`moduleInTypePosition1_0.ts` containing `export class Promise { ... }`; module
build reports:

```text
UnsupportedModule: issue-5005: dependency module declaration export uses a form outside the current static export slice
```

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleInTypePosition1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleInTypePosition1.ts --detail
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
- [x] Existing owner `issues/open/5324-support-dependency-export-class-declarations.md` contains the implementation-ready acceptance criteria
- [x] This closed bucket preserves the exact reference path, diagnostic, source context, AST, and oracle evidence
- [x] No new child issue was needed because the blocker matches an existing open owner

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleInTypePosition1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleInTypePosition1.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5324-support-dependency-export-class-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleInTypePosition1.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh run on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleInTypePosition1.ts
```

Coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleInTypePosition1.ts --detail --no-dashboard-data
```

Result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
reference/typescript/tests/cases/compiler/moduleInTypePosition1.ts: UnsupportedModule: import-export
```

Current diagnostic:

```text
UnsupportedModule: issue-5005: dependency module declaration export uses a form outside the current static export slice at 7..41
```

Source context:

```text
1 | // @target: es2015
2 | // @module: commonjs
3 | // @Filename: moduleInTypePosition1_0.ts
4 | export class Promise {
```

Compiler evidence:

```text
tokens: ok through `export class Promise`, `import WinJS = require("./moduleInTypePosition1_0")`, and arrow parameter type `WinJS`
ast: ok; top-level nodes include ExportDecl(ClassDecl Promise), ImportDefault source "./moduleInTypePosition1_0", and `var x = (w1: WinJS) => {}`
resolved dump: also exposes issue-232 missing virtual local module `./moduleInTypePosition1_0` if the dependency export-class boundary is bypassed
```

TypeScript oracle:

```text
TS2564: Property 'foo' has no initializer and is not definitely assigned in the constructor.
TS2307: Cannot find module './moduleInTypePosition1_0' or its corresponding type declarations.
binding x: (w1: WinJS) => void
parameter w1: WinJS
```

Superseded by:

- `issues/open/5324-support-dependency-export-class-declarations.md`

Related later boundary:

- `issues/open/5229a-resolve-imports-between-filename-sections.md` owns virtual `@Filename` sibling import resolution when the current dependency export-class blocker is removed.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleInTypePosition1.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnsupportedModule/import-export
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleInTypePosition1.ts
result: pass; reproduced issue-5005 dependency-module export class boundary owned by issue 5324
date: 2026-05-08
```

Remaining risks:

- After dependency export-class support lands, this reference may advance to
  virtual `@Filename` local module resolution tracked by issue 5229.
