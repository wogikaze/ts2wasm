---
id: 3428
title: "Implement Namedimportnonexistentname"
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

Closed as superseded by `issues/open/5346-parse-commonjs-export-assignment-statements.md`.

Fresh focused coverage and triage show `namedImportNonExistentName.ts` still
stops at the first `export = Foo;` CommonJS export assignment with the generic
issue-055 static export boundary. That exact first blocker is already owned by
issue 5346. Later diagnostics in the same reference also touch
`export as namespace` parsing and virtual module resolution, which are tracked
separately.

## Problem

Reference test results show 1 cases fail in directory `namedImportNonExistentName` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: namedImportNonExistentName had 1 generated reference failure and
needed smart-triage evidence before implementation starts.

Disposition: no child issue created because the current first blocker is
covered by existing open issue 5346.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/namedImportNonExistentName.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/namedImportNonExistentName.ts --detail
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
- [x] Existing owner issue 5346 names the exact current diagnostic change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/namedImportNonExistentName.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/namedImportNonExistentName.ts
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

- `reference/typescript/tests/cases/compiler/namedImportNonExistentName.ts`

## Duplicate detection

- `issues/open/5346-parse-commonjs-export-assignment-statements.md` covers the
  current first blocker, `export = Foo;`, and the expected frontend advance:
  parse CommonJS export assignment instead of reporting issue-055 static export.
- Related later blockers after 5346 advances:
  `issues/open/5231a-parse-export-as-namespace-declarations.md` for
  `export as namespace Foo;`, `issues/open/5306-report-export-assignment-with-other-exports.md`
  for invalid mixed/duplicate export-assignment diagnostics, and
  `issues/open/5229a-resolve-imports-between-filename-sections.md` for virtual
  `./foo` and `./foo2` imports.

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/namedImportNonExistentName.ts --detail --no-dashboard-data

result:
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=import-export:1
semantic_enabled=0
reference/typescript/tests/cases/compiler/namedImportNonExistentName.ts: UnsupportedSyntax: import-export
```

Fresh focused triage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namedImportNonExistentName.ts

result:
UnsupportedModule: issue-055: unsupported static export; module resolution and loading are not implemented at 85..91
The failing source span is the first CommonJS export assignment, `export = Foo;`.
```

Source context:

```ts
export = Foo;
export as namespace Foo;

declare namespace Foo {
    function foo();
}

// @filename: foo2.ts
declare let x: { a: string; c: string; } | { b: number; c: number; };
export = x

import { Bar, toString, foo } from './foo';
foo();
import { a, b, c, d, toString as foo2String } from './foo2';
c;
```

Compiler evidence:

```text
tokens: ok through export assignment, export-as-namespace declaration, ambient namespace, second export assignment, and named imports
ast/resolved: fail immediately at the first `export = Foo;` with issue-055 unsupported static export
current owner: issue 5346 parses `export = expr;` as the next required frontend step
```

TypeScript oracle evidence:

```text
TS2300: Duplicate identifier 'export=' at `Foo`
TS1315: Global module exports may only appear in declaration files at `export as namespace Foo;`
TS2300: Duplicate identifier 'export=' at `x`
TS2307: Cannot find module './foo'
TS2307: Cannot find module './foo2'
```

## Completion evidence

Closed as superseded by issue 5346; no additional child issue created.

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/namedImportNonExistentName.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namedImportNonExistentName.ts
result: pass; current first blocker is issue-055 static export at `export = Foo;`
date: 2026-05-08
```

Remaining risks:

- After issue 5346 advances this path, narrower follow-up diagnostics may be
  exposed for duplicate export assignments, `export as namespace`, and virtual
  multi-file module resolution.
