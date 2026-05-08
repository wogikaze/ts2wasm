---
id: 3325
title: "Implement Moduleaugmentationwithnonexistentnamedimport"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed this generated `import-export` bucket as superseded by the open
implementation-ready issue 5346.

Fresh smart triage shows the first actionable blocker is `export = Foo;`, which
currently reports the generic issue-055 static export boundary before AST
construction.

## Problem

`moduleAugmentationWithNonExistentNamedImport.ts` combines several module
features:

- CommonJS export assignment: `export = Foo;`
- UMD namespace export: `export as namespace Foo;`
- ambient namespace/global declarations
- virtual `@filename` import from `./foo`
- second export assignment and namespace export in `bar.d.ts`

Problem: the stale generated bucket is too broad. The current first blocker is
already owned by issue 5346, while later syntax/module-graph blockers are
tracked by existing issues 5231 and 5229.

## Current failure

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationWithNonExistentNamedImport --detail --no-dashboard-data
```

Representative triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationWithNonExistentNamedImport.ts
```

## Desired final state

This generated bucket is closed. Implement the first blocker from
`issues/open/5346-parse-commonjs-export-assignment-statements.md`.

## Scope

In scope:

- [x] Inspect fresh coverage and smart triage.
- [x] Identify `export = Foo;` as the current first blocker.
- [x] Confirm open issue 5346 owns CommonJS `export = expr;` parsing.
- [x] Record later related owner issues 5231 and 5229.

Out of scope:

- Direct implementation from this generated bucket.
- Runtime lowering for CommonJS export assignment.
- `export as namespace` parsing.
- Virtual `@filename` import resolution.
- TypeScript semantic diagnostics after the parser advances.

## Affected paths

Expected:

- `issues/open/5346-parse-commonjs-export-assignment-statements.md`

Do not touch:

- Rust implementation files

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded.
- [x] Existing owner issue 5346 contains the exact `moduleAugmentationWithNonExistentNamedImport.ts` evidence.
- [x] Later known blockers are linked to issue 5231 and issue 5229.
- [x] Closure preserves exact reproduction commands and current diagnostic.

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationWithNonExistentNamedImport --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationWithNonExistentNamedImport.ts
```

Not run:

- `cargo fmt --all --check` (no Rust changes)
- `cargo nextest run` (no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleAugmentationWithNonExistentNamedImport.ts`

## Duplicate detection

- `issues/open/5346-parse-commonjs-export-assignment-statements.md` owns the
  current first blocker: `export = Foo;` reports generic issue-055 before AST
  construction.
- `issues/open/5231-parse-export-as-namespace-declarations.md` owns later
  `export as namespace Foo;` / `export as namespace Bar;` parsing.
- `issues/open/5229-resolve-imports-between-filename-sections.md` owns later
  virtual `@filename` section resolution for `import { Bar } from "./foo";`.
- `issues/open/5306-report-export-assignment-with-other-exports.md` is related
  but not the current first blocker because it covers a specific TS2309 mixed
  export diagnostic after `export =` is recognized.

## Smart triage

Fresh coverage on 2026-05-08:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=import-export:1
```

Fresh smart triage headline:

```text
Feature label: import-export
Diagnostic: UnsupportedModule / unsupported-feature-boundary
message: issue-055: unsupported static export; module resolution and loading are not implemented at 0..6
line: 1, column: 1
```

Source shape:

```ts
// @filename: foo.d.ts
export = Foo;
export as namespace Foo;

declare namespace Foo {
    function foo();
}

declare global {
    namespace Bar { }
}

// @filename: bar.d.ts
import { Bar } from './foo';
export = Bar;
export as namespace Bar;
```

Tokens include both export assignments, both namespace exports, the ambient
namespace/global declarations, and the named import from `./foo`. AST and
resolved output stop before AST construction:

```text
error: [UnsupportedModule] issue-055: unsupported static export; module resolution and loading are not implemented at 63..69
```

TypeScript oracle reports:

```text
TS2300: Duplicate identifier 'export='.
TS1315: Global module exports may only appear in declaration files.
TS2307: Cannot find module './foo' or its corresponding type declarations.
```

## Completion evidence

Commits:

- this local closure commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationWithNonExistentNamedImport --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnsupportedSyntax:1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationWithNonExistentNamedImport.ts
result: pass; current first blocker is issue-055 static export for `export = Foo;`, owned by issue 5346
date: 2026-05-08
```

Remaining risks:

- Advancing issue 5346 may expose issue 5231, issue 5229, or the TypeScript
  oracle diagnostics listed above.
