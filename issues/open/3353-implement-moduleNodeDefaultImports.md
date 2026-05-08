---
id: 3353
title: "Implement Modulenodedefaultimports"
type: maintenance
area: frontend/syntax
class: superseded
priority: P1
depends_on: [432]
blocks: [5417]
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Split into implementation-ready child issue 5417. Fresh triage for
`moduleNodeDefaultImports.ts` shows the current blocker is parser support for
the contextual `default` keyword in named import specifier lists.

## Problem

Reference test results show 1 case failing in directory
`moduleNodeDefaultImports` with diagnostics: import-export. Fresh coverage still
reports `UnsupportedSyntax`, and compiler dumps show the parser rejects:

```ts
import { default as b } from "./mod.cjs";
```

Problem: this generated bucket is too broad for direct implementation. The
actionable work is tracked by issue 5417.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleNodeDefaultImports.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleNodeDefaultImports.ts --detail
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
- [x] Child issue 5417 contains an exact `reference-triage` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleNodeDefaultImports.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleNodeDefaultImports.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5417a-parse-default-keyword-import-export-specifiers.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleNodeDefaultImports.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh run on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleNodeDefaultImports.ts
```

Coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleNodeDefaultImports.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=import-export:1
reference/typescript/tests/cases/compiler/moduleNodeDefaultImports.ts: UnsupportedSyntax: import-export
```

Current diagnostic:

```text
UnsupportedSyntax: expected identifier, got Some(SpannedToken { kind: Default, span: Span { start: 196, end: 203 } }) at 204..206
```

Source context:

```text
// @target: es2022
// @module: node16,node18,node20,nodenext
// @filename: mod.cts
declare function fun(): void;
export default fun;

// @filename: b.mts
import a from "./mod.cjs";
import { default as b } from "./mod.cjs";
import c, { default as d } from "./mod.cjs";
```

Compiler evidence:

```text
tokens: ok; Default tokens are preserved in `export default fun`, `import { default as b }`, combined default+named import, and later re-export specifiers
ast/resolved: fail while parsing named import specifier `default as b`
visible symbols: []
```

TypeScript oracle:

```text
AST topLevel includes `import { default as b }`, `import c, { default as d }`, `export { default }`, and `export { default as def }`.
Diagnostics include TS2528 multiple default exports and TS2307 missing local `.cjs`/`.mjs` modules, but those are hidden until the parser accepts `default` specifiers.
```

Split to:

- `issues/open/5417a-parse-default-keyword-import-export-specifiers.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleNodeDefaultImports.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnsupportedSyntax/import-export
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleNodeDefaultImports.ts
result: pass; current blocker split to issue 5417
date: 2026-05-08
```

Remaining risks:

- After issue 5417 lands, this reference may advance to `.cjs`/`.mjs` virtual
  filename module resolution, named re-export `default` specifier parsing,
  TS2528 multiple default export diagnostics, or default import/export runtime
  semantics.
