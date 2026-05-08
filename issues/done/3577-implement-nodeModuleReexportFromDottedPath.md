---
id: 3577
title: "Implement Nodemodulereexportfromdottedpath"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5324]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage nodeModuleReexportFromDottedPath across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh triage shows the representative path currently stops at the existing
dependency-module `export class` issue-5005 boundary owned by issue `5324`.

Problem: the generated bucket is not an independent implementation slice; the
first actionable blocker is already tracked.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nodeModuleReexportFromDottedPath.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nodeModuleReexportFromDottedPath.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by issue `5324`. Do not
implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue `5324`
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
- [x] Existing issue `5324` owns the current dependency-module `export class` issue-5005 boundary
- [x] Closed bucket includes failing path, diagnostic code, source context, visible symbols, parser/resolved evidence, and TypeScript AST evidence
- [x] No child issue is needed from `3577` because the current blocker is already implementation-ready in issue `5324`

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nodeModuleReexportFromDottedPath.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeModuleReexportFromDottedPath.ts
```

Not run:

- `cargo fmt --all --check`; metadata-only issue triage close.
- `cargo nextest run`; metadata-only issue triage close.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; superseded by issue `5324`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nodeModuleReexportFromDottedPath.ts`

## Duplicate detection

- `issues/open/5324-support-dependency-export-class-declarations.md` owns the
  current dependency virtual-file `export class` issue-5005 boundary.
- Completed issue `232` documents the later non-local module specifier
  boundary visible in the resolved dump for `.prisma/client`, but that is not
  the headline first blocker for this path.

## Smart triage

Fresh run on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeModuleReexportFromDottedPath.ts
```

Coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nodeModuleReexportFromDottedPath.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
reference/typescript/tests/cases/compiler/nodeModuleReexportFromDottedPath.ts: UnsupportedModule: import-export
```

Current headline/stack diagnostic:

```text
UnsupportedModule: issue-5005: dependency module declaration export uses a form
outside the current static export slice at 75..169
```

Source context:

```ts
// @Filename: /node_modules/.prisma/client/index.d.ts
export interface PrismaClientOptions {
  rejectOnNotFound?: any;
}

export class PrismaClient<T extends PrismaClientOptions = PrismaClientOptions> {
  private fetcher;
}
```

Compiler evidence:

```text
tokens: ok; export interface, export class, export * from ".prisma/client", import { PrismaClient } from "@prisma/client"
ast: ok; dependency ExportDecl(ClassDecl PrismaClient), ExportAllFrom ".prisma/client", ImportNamed "@prisma/client", ExportDefault
resolved dump: later issue-232 unsupported non-local module specifier `.prisma/client`
module build/stack: issue-5005 dependency-module export class boundary
visible symbols: []
```

TypeScript oracle:

```text
TS2395: Individual declarations in merged declaration 'PrismaClient' must be all exported or all local.
TS2307: Cannot find module '.prisma/client' or its corresponding type declarations.
TS2307: Cannot find module '@prisma/client' or its corresponding type declarations.
AST topLevel includes InterfaceDeclaration, ClassDeclaration, ExportDeclaration, ImportDeclaration, declare const, const, and ExportAssignment.
```

## Completion evidence

Status: done

Commits:

- this local issue-cleanup commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nodeModuleReexportFromDottedPath.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnsupportedModule/import-export
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeModuleReexportFromDottedPath.ts
result: pass; current dependency export-class blocker is superseded by issue 5324
date: 2026-05-08
```

Remaining risks:

- After issue `5324` advances dependency `export class`, this reference may
  expose the later `.prisma/client` dotted-path re-export or `@prisma/client`
  virtual node_modules package-resolution blocker.
