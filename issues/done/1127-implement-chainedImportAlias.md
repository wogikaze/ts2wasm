---
id: 1127
title: "Implement Chainedimportalias"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [232]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage chainedImportAlias across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `chainedImportAlias` with diagnostics: import-export. Fresh triage shows the current runner view parses the import-equals source far enough to collect `require("./chainedImportAlias_file0")`, then stops on the existing issue-232 missing local module diagnostic for the virtual `// @Filename:` module layout.

Problem: `chainedImportAlias` is not a standalone implementation order in the current runner view; the first blocker is an oracle-matching missing local module diagnostic covered by issue 232 module graph behavior.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/chainedImportAlias.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/chainedImportAlias.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by `issues/done/232-resolve-local-relative-es-module-graph.md` for the current missing local module diagnostic. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 232's missing local module diagnostic behavior
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/chainedImportAlias.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/chainedImportAlias.ts
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

- `reference/typescript/tests/cases/compiler/chainedImportAlias.ts`

## Duplicate detection

- `issues/done/232-resolve-local-relative-es-module-graph.md` owns source-spanned missing local module diagnostics for local relative module graph construction.
- Import alias issues are not exact matches for the current first blocker because triage stops at missing module resolution before alias binding or namespace member lookup.

## Smart triage

### Smart triage: Triage name resolution: chainedImportAlias

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/chainedImportAlias.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/chainedImportAlias.ts
```

Coverage reproduction:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/chainedImportAlias.ts --detail --no-dashboard-data
```

Source context:

```text
// @target: es2015
// @module: commonjs
// @Filename: chainedImportAlias_file0.ts
export namespace m {
    export function foo() { }
}

// @Filename: chainedImportAlias_file1.ts
import x = require("./chainedImportAlias_file0");
import y = x;
y.m.foo();
```

Current compiler failure:

```text
error: [UnsupportedModule] issue-232: missing local module `./chainedImportAlias_file0` imported from .../chainedImportAlias.ts; tried .../chainedImportAlias_file0.ts, .js, .d.ts, .tsx, .mjs, .cjs at 206..234
```

Compiler evidence:

- Tokens succeed for `export namespace m`, `import x = require("./chainedImportAlias_file0")`, `import y = x`, and `y.m.foo()`.
- AST currently contains `ImportDefault { local: "", source: "./chainedImportAlias_file0" }`, an `Expr { expr: Undefined }` for `import y = x;`, and `Call(Member(Member(Ident y, "m"), "foo"), args=[])`.
- Resolved dump stops in module graph validation before alias binding with the issue-232 missing local module diagnostic.

TypeScript oracle evidence:

```text
TS2307: Cannot find module './chainedImportAlias_file0' or its corresponding type declarations.
```

The TypeScript AST for the raw source includes:

```text
ModuleDeclaration: export namespace m { ... }
ImportEqualsDeclaration: import x = require('./chainedImportAlias_file0');
ImportEqualsDeclaration: import y = x;
ExpressionStatement: y.m.foo();
```

Resolution:

```text
Issue 232 established source-spanned missing local module diagnostics for local relative module graph construction. The current reference-triage failure is the same missing-module boundary in this runner view rather than an actionable chained import alias binding slice.
```

## Completion evidence

Fill only when moving to `done/`.

chainedImportAlias triage is complete. The current failure is superseded by issue 232 missing local module diagnostics.

Commits:

- superseded by `issues/done/232-resolve-local-relative-es-module-graph.md`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/chainedImportAlias.ts --detail --no-dashboard-data
result: pass on the main checkout; 1 executed, current failure is UnsupportedModule import-export
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/chainedImportAlias.ts
result: pass; module_graph reports issue-232 missing local module `./chainedImportAlias_file0`
date: 2026-05-06
```

Remaining risks:

- none
