---
id: 3431
title: "Implement Namespacemergedwithimportaliasnocrash"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Closed as superseded by `issues/open/5229a-resolve-imports-between-filename-sections.md`.

Fresh focused coverage shows `namespaceMergedWithImportAliasNoCrash.ts` parses
the static namespace import, then reaches the existing issue-232 missing local
module boundary for `./file1` between virtual `// @filename:` sections. The
smart triage headline also exposes namespace/name-resolution evidence for
`Library.foo`, but the generated `import-export` blocker is the virtual section
module graph boundary owned by issue 5229.

## Problem

Reference test results show 1 cases fail in directory `namespaceMergedWithImportAliasNoCrash` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: namespaceMergedWithImportAliasNoCrash had 1 generated reference
failure and needed smart-triage evidence before implementation starts.

Disposition: no child issue created because the current import/export blocker
is covered by existing open issue 5229.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/namespaceMergedWithImportAliasNoCrash.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/namespaceMergedWithImportAliasNoCrash.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close as superseded by an existing implementation-ready owner issue
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
- [x] Fresh evidence contains an exact `reference-triage` command
- [x] Evidence includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Existing owner issue 5229 names the exact current diagnostic change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/namespaceMergedWithImportAliasNoCrash.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/namespaceMergedWithImportAliasNoCrash.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/namespaceMergedWithImportAliasNoCrash.ts`

## Duplicate detection

- `issues/open/5229a-resolve-imports-between-filename-sections.md` owns the
  current module graph boundary: resolve `import * as Lib from "./file1"` from
  the `file2.ts` virtual section to the sibling `file1.ts` section.
- Related later semantic diagnostics after 5229 advances include TS2708 for
  `Library.foo` and TS2694 for `var x: Lib.Bar;`.

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/namespaceMergedWithImportAliasNoCrash.ts --detail --no-dashboard-data

result:
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
semantic_enabled=0
reference/typescript/tests/cases/compiler/namespaceMergedWithImportAliasNoCrash.ts: UnsupportedModule: import-export
```

Fresh focused triage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namespaceMergedWithImportAliasNoCrash.ts

headline:
UnresolvedName: unresolved name: `Library` at 101..108

resolved/module_graph:
UnsupportedModule: issue-232: missing local module `./file1` imported from namespaceMergedWithImportAliasNoCrash.ts at 251..260
```

Source context:

```ts
// @filename: file1.ts
export namespace Library {
    export type Bar = { a: number };
}
var x: Library.Bar; // should work
Library.foo; // should be an error

// @filename: file2.ts
import * as Lib from './file1';
namespace Lib { // should fail to merge
    export const foo: string = "";
}
Lib.foo; // should work
var x: Lib.Bar; // should be an error
export { Lib }
```

Compiler evidence:

```text
tokens: ok through exported namespace Library, type-only Bar, Library.foo, import * as Lib from "./file1", namespace Lib, Lib.foo, var x: Lib.Bar, and export { Lib }
ast: ok; retains var x, Library.foo expression, ImportNamespace Lib from "./file1", Lib.foo expression, var x, and ExportNamed Lib
resolved/module_graph: stops at issue-232 missing local module `./file1` because virtual @filename section resolution has not registered file1.ts for the file2.ts import
```

TypeScript oracle evidence:

```text
TS2708: Cannot use namespace 'Library' as a value.
TS2307: Cannot find module './file1' or its corresponding type declarations.
TS2694: Namespace 'Lib' has no exported member 'Bar'.
```

## Completion evidence

Closed as superseded by issue 5229; no additional child issue created.

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/namespaceMergedWithImportAliasNoCrash.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_diagcodes=UnsupportedModule:1
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namespaceMergedWithImportAliasNoCrash.ts
result: pass; current import/export blocker is issue-232 missing local module `./file1`, owned by issue 5229
date: 2026-05-08
```

Remaining risks:

- After issue 5229 advances this path, narrower follow-up diagnostics may be
  exposed for namespace-as-value use, namespace/import alias merging, or missing
  namespace members in type annotations.
