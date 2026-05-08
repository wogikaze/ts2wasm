---
id: 3355
title: "Implement Modulenonedynamicimport"
type: maintenance
area: frontend/syntax
class: superseded
priority: P1
depends_on: [432]
blocks: [5418]
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Split into implementation-ready child issue 5418. Fresh triage for
`moduleNoneDynamicImport.ts` shows the current blocker is parser support for
dynamic `import("./b")` call expressions in expression position.

## Problem

Reference test results show 1 case failing in directory
`moduleNoneDynamicImport` with diagnostics: import-export. Fresh coverage still
reports import-export, and compiler dumps show the parser rejects:

```ts
const foo = import("./b");
```

Problem: this generated bucket is too broad for direct implementation. The
actionable work is tracked by issue 5418.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleNoneDynamicImport.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleNoneDynamicImport.ts --detail
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
- [x] Child issue 5418 contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, parser token evidence, and TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleNoneDynamicImport.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleNoneDynamicImport.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5418a-parse-dynamic-import-call-expressions.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleNoneDynamicImport.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh run on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleNoneDynamicImport.ts
```

Coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleNoneDynamicImport.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=import-export:1
reference/typescript/tests/cases/compiler/moduleNoneDynamicImport.ts: UnsupportedSyntax: import-export
```

Current diagnostic:

```text
UnsupportedModule: unsupported expression: Some(SpannedToken { kind: Import, span: Span { start: 112, end: 118 } }) at 118..119
```

Source context:

```text
// @allowJs: true
// @target: es2015,es2020
// @module: none
// @outFile: /a.js
// @filename: /a.ts
const foo = import("./b");

// @filename: /b.js
export default 1;
```

Compiler evidence:

```text
tokens: ok; Import, LeftParen, String("./b"), RightParen, Semicolon are present
visible symbols: []
ast/resolved: fail before representing the initializer expression
```

TypeScript oracle:

```text
AST topLevel includes `const foo = import("./b");` and `export default 1;`.
The binding hint for `foo` is `Promise<any>`. Diagnostics include TS2307 for
missing `./b`, but that is hidden until the parser accepts dynamic import calls.
```

Split to:

- `issues/open/5418a-parse-dynamic-import-call-expressions.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleNoneDynamicImport.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnsupportedSyntax/import-export
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleNoneDynamicImport.ts
result: pass; current blocker split to issue 5418
date: 2026-05-08
```

Remaining risks:

- After issue 5418 lands, this reference may advance to virtual-section import
  resolution for `./b`, export default parsing/lowering in the `.js` section,
  or dynamic import runtime/module loading semantics.
