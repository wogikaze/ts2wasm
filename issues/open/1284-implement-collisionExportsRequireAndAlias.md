---
id: 1284
title: "Implement Collisionexportsrequireandalias"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1284.

## Summary

Triage collisionExportsRequireAndAlias across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `collisionExportsRequireAndAlias` with diagnostics: import-export. Fresh triage shows the current blocker is virtual `@Filename` module resolution for local `import = require(...)` specifiers, already covered by issue 5229.

Problem: collisionExportsRequireAndAlias needs local imports between virtual `@Filename` sections to resolve before alias-specific behavior can be triaged.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionExportsRequireAndAlias.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionExportsRequireAndAlias.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by issue 5229. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with existing issue 5229
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed bucket

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
- [x] Superseded by `issues/open/5229a-resolve-imports-between-filename-sections.md`
- [x] Smart triage evidence below includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript evidence
- [x] Superseding issue 5229 acceptance names the same virtual `@Filename` module-resolution behavior

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionExportsRequireAndAlias.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionExportsRequireAndAlias.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by: `issues/open/5229a-resolve-imports-between-filename-sections.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionExportsRequireAndAlias.ts`

## Duplicate detection

- `issues/open/5229a-resolve-imports-between-filename-sections.md` owns registering TypeScript reference `// @Filename:` sections as virtual module paths and resolving local imports between them.

## Smart triage

Reproduction:
`python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionExportsRequireAndAlias.ts`.

Focused coverage:
`python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionExportsRequireAndAlias.ts --detail --no-dashboard-data`.

Current diagnostic:

```text
UnsupportedModule: issue-232: missing local module `./collisionExportsRequireAndAlias_file1` imported from collisionExportsRequireAndAlias.ts at 296..337
```

Focused coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
```

Representative source:

```ts
// @Filename: collisionExportsRequireAndAlias_file1.ts
export function bar() {
}

// @Filename: collisionExportsRequireAndAlias_file2.ts
import require = require('./collisionExportsRequireAndAlias_file1'); // Error
import exports = require('./collisionExportsRequireAndAlias_file3333'); // Error
export function foo() {
    require.bar();
}
```

Compiler evidence:

```text
tokens: ok through export functions and import-equals require declarations
ast: ok; require aliases are represented as import declarations with local module specifiers
module_graph: fails with issue-232 missing local module before alias binding
visible symbols: bar, bar2, foo, foo2
```

TypeScript oracle evidence:

```text
TS2307: Cannot find module './collisionExportsRequireAndAlias_file1' or its corresponding type declarations.
TS2307: Cannot find module './collisionExportsRequireAndAlias_file3333' or its corresponding type declarations.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- superseded by `issues/open/5229a-resolve-imports-between-filename-sections.md`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionExportsRequireAndAlias.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; current missing virtual local module blocker is covered by issue 5229
date: 2026-05-07
```

Remaining risks:

- Issue 5229 currently names ES import examples. After virtual section
  resolution advances, this reference may expose a narrower CommonJS
  `import = require` alias-binding blocker, especially because the aliases are
  named `require` and `exports`.
