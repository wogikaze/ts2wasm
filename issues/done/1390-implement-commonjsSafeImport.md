---
id: 1390
title: "Implement Commonjssafeimport"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: [5229]
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1390.

## Summary

Closed as superseded by
`issues/done/5229-w0-user-runtime-string-origin.md`.

Fresh focused triage shows `commonjsSafeImport.ts` currently stops in module
graph construction because `./10_lib` is a TypeScript reference
`// @filename:` virtual section, not a real on-disk sibling file.

## Problem

Reference test results originally showed 1 case failing in directory
`commonjsSafeImport` with diagnostics: import-export. Fresh focused coverage on
2026-05-07 still reports `UnsupportedModule` / `import-export`, and the resolved
dump shows the existing issue-232 missing local module diagnostic for
`./10_lib`.

Problem: `commonjsSafeImport.ts` contains:

```ts
// @filename: 10_lib.ts
export function Foo() {}

// @filename: main.ts
import { Foo } from './10_lib';
Foo();
```

The current reference harness parses both sections but module graph resolution
looks for `10_lib.ts` on disk beside the combined source file instead of
registering the virtual section as a local module.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonjsSafeImport.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonjsSafeImport.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
coverage: executed=1, build_pass=0, unsupported=1
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
```

Resolved/module graph evidence:

```text
error: [UnsupportedModule] issue-232: missing local module `./10_lib`
imported from .../reference/typescript/tests/cases/compiler/commonjsSafeImport.ts;
tried ..././10_lib.ts, ..././10_lib.js, ..././10_lib.d.ts, ..././10_lib.tsx,
..././10_lib.mjs, ..././10_lib.cjs at 160..170
```

TypeScript oracle evidence:

```text
TS2307: Cannot find module './10_lib' or its corresponding type declarations.
```

## Desired final state

This generated bucket is closed. Implementation proceeds through issue 5229,
which owns registering TypeScript reference `@Filename` / `@filename` sections
as virtual module paths and resolving imports between those sections.

After issue 5229 resolves the virtual `./10_lib` module, this reference path may
need fresh triage for CommonJS emit/declaration output or static named import
execution details.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5229's virtual-section import work
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- CommonJS emit semantics after virtual module resolution succeeds
- Declaration emit output for this reference test

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
- [x] Existing issue 5229 covers local imports between `@filename` sections
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonjsSafeImport.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonjsSafeImport.ts
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

- [x] superseded by: `issues/done/5229-w0-user-runtime-string-origin.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commonjsSafeImport.ts`

## Duplicate detection

- `issues/done/5229-w0-user-runtime-string-origin.md` owns
  registering TypeScript reference `@Filename` / `@filename` sections as
  virtual module paths and resolving local imports between them.
- `issues/done/232-resolve-local-relative-es-module-graph.md` owns real
  on-disk local relative module graph diagnostics, but not virtual section
  registration.
- `issues/done/1384-implement-commonJsImportClassExpression.md`,
  `issues/done/1127-implement-chainedImportAlias.md`, and
  `issues/done/1162-implement-circularReferenceInImport.md` are prior generated
  buckets closed on the same missing virtual module boundary.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage name resolution: commonjsSafeImport

- Issue class: triage-needed
- Feature label: name-resolution
- Diagnostic: UnresolvedName / resolver-symbol
- Path: reference/typescript/tests/cases/compiler/commonjsSafeImport.ts
```

Primary build/coverage diagnostic:

```text
UnsupportedModule / import-export
issue-232: missing local module `./10_lib`
```

AST evidence:

```text
ExportDecl(Function name: "Foo")
ImportNamed { imported: "Foo", local: "Foo", source: "./10_lib" }
Expr(Call(Ident("Foo"), []))
```

Resolver/module graph evidence:

```text
[pipeline] module_graph
error: [UnsupportedModule] issue-232: missing local module `./10_lib`
```

TypeScript oracle evidence:

```text
typescriptVersion: 6.0.3
TS2307 Cannot find module './10_lib' or its corresponding type declarations.
```

## Completion evidence

Commits:

- superseded by `issues/done/5229-w0-user-runtime-string-origin.md`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonjsSafeImport.ts
result: pass; reproduced issue-232 missing virtual local module `./10_lib`
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonjsSafeImport.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_diagcodes=UnsupportedModule:1
date: 2026-05-07
```

Remaining risks:

- After issue 5229 lands, this reference path may expose CommonJS module emit,
  declaration emit, or named import execution behavior.
