---
id: 3512
title: "Implement Nocirculardefinitiononexportofprivateinmergednamespace"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5346]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Triage noCircularDefinitionOnExportOfPrivateInMergedNamespace across 1 failing reference test cases and split this bucket into implementation-ready child issues.

Closed as superseded by
`issues/open/5346-parse-commonjs-export-assignment-statements.md`. Fresh
triage shows the current first blocker is the `export = Foo;` issue-055 static
export boundary before merged namespace behavior is actionable.

## Problem

Reference test results show 1 cases fail in directory `noCircularDefinitionOnExportOfPrivateInMergedNamespace` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: noCircularDefinitionOnExportOfPrivateInMergedNamespace currently
stops at the existing CommonJS `export = expr;` parser boundary before the
export-of-private-in-merged-namespace behavior is actionable.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/noCircularDefinitionOnExportOfPrivateInMergedNamespace.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/noCircularDefinitionOnExportOfPrivateInMergedNamespace.ts --detail
```

## Desired final state

This generated bucket is closed. Implement the current blocker through
`issues/open/5346-parse-commonjs-export-assignment-statements.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5346
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
- [x] Existing issue 5346 owns the current CommonJS `export = expr;` issue-055 boundary
- [x] This closure includes failing path, diagnostic code, source context,
  visible symbols, parser evidence, and TypeScript AST evidence
- [x] No child issue is needed from 3512 because the current blocker is already implementation-ready in issue 5346

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/noCircularDefinitionOnExportOfPrivateInMergedNamespace.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/noCircularDefinitionOnExportOfPrivateInMergedNamespace.ts
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

- [x] none; superseded by issue 5346

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noCircularDefinitionOnExportOfPrivateInMergedNamespace.ts`

## Duplicate detection

- `issues/open/5346-parse-commonjs-export-assignment-statements.md` owns the
  current CommonJS `export = expr;` issue-055 static export boundary.
- Related export-assignment generated buckets such as
  `issues/done/3411-implement-multipleExportAssignments.md`,
  `issues/done/3428-implement-namedImportNonExistentName.md`, and
  `issues/done/3451-implement-narrowedImports.md` already fold the same first
  blocker into issue 5346.

## Smart triage

Fresh run on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCircularDefinitionOnExportOfPrivateInMergedNamespace.ts
```

Coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCircularDefinitionOnExportOfPrivateInMergedNamespace.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=import-export:1
reference/typescript/tests/cases/compiler/noCircularDefinitionOnExportOfPrivateInMergedNamespace.ts: UnsupportedSyntax: import-export
```

Current diagnostic:

```text
UnsupportedModule: issue-055: unsupported static export; module resolution and loading are not implemented at 73..79
```

Source context:

```ts
const cat = 12;
class Foo {}
export = Foo;
declare namespace Foo {
    export { cat };
}
```

Compiler evidence:

```text
tokens: ok; const cat, class Foo, export = Foo, declare namespace Foo, export { cat }
ast: fails at export = Foo with issue-055 unsupported static export
resolved: fails at the same issue-055 boundary
visible symbols before failure: binding cat, class Foo
```

TypeScript oracle:

```text
ok=true
diagnostics=[]
AST topLevel includes const cat, class Foo, ExportAssignment `export = Foo`,
and ModuleDeclaration `declare namespace Foo { export { cat }; }`.
binding hint: cat has literal type 12
```

## Completion evidence

Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCircularDefinitionOnExportOfPrivateInMergedNamespace.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnsupportedSyntax/import-export
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCircularDefinitionOnExportOfPrivateInMergedNamespace.ts
result: pass; current issue-055 CommonJS export-assignment blocker is superseded by issue 5346
date: 2026-05-08
```

Remaining risks:

- After issue 5346 lands, this reference may expose merged namespace export,
  export-of-private, or CommonJS module lowering diagnostics.
