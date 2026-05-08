---
id: 3359
title: "Implement Modulepreserveimporthelpers"
type: maintenance
area: frontend/syntax
class: superseded
priority: P1
depends_on: [432, 5276]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed as superseded by
`issues/open/5276-report-class-declaration-decorator-boundary.md`.
Fresh triage for `modulePreserveImportHelpers.ts` stops at class declaration
decorator syntax before import helpers or tslib declarations become actionable.

## Problem

Reference test results show 1 case failing in directory
`modulePreserveImportHelpers` with diagnostics: import-export. Fresh triage
shows the current first blocker is:

```text
UnsupportedSyntax: unsupported character: @ at 109..110
```

Problem: this generated bucket duplicates the class declaration decorator
boundary already tracked by issue 5276.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/modulePreserveImportHelpers.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/modulePreserveImportHelpers.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5276
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closure

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
- [x] Existing issue 5276 owns the current class declaration decorator boundary
- [x] This closure includes failing path, diagnostic code, source context, visible symbols, lexer/parser evidence, and TypeScript AST evidence
- [x] No child issue is needed from 3359 because the current blocker is already implementation-ready in issue 5276

## Validation

Required commands for this closure:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/modulePreserveImportHelpers.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePreserveImportHelpers.ts
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Not run:

- Cargo gates; no Rust source changed.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; superseded by issue 5276

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/modulePreserveImportHelpers.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh run on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePreserveImportHelpers.ts
```

Coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/modulePreserveImportHelpers.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=import-export:1
reference/typescript/tests/cases/compiler/modulePreserveImportHelpers.ts: UnsupportedSyntax: import-export
```

Current diagnostic:

```text
UnsupportedSyntax: unsupported character: @ at 109..110
```

Source context:

```ts
declare var dec: any

@dec()
export class A {}

// @Filename: /b.cts
declare var dec: any

@dec()
class B {}
export {};
```

Compiler evidence:

```text
tokens: fail before token stream; lexer reports unsupported character `@`
ast/resolved: same lexer failure
visible symbols: []
```

TypeScript oracle:

```text
AST topLevel includes ClassDeclaration `@dec()\nexport class A {}`,
ClassDeclaration `@dec()\nclass B {}`, package.json Blocks, and tslib
FunctionDeclarations `__esDecorate` / `__runInitializers`.
```

Superseding issue:

- `issues/open/5276-report-class-declaration-decorator-boundary.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/modulePreserveImportHelpers.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnsupportedSyntax/import-export
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePreserveImportHelpers.ts
result: pass; current blocker is class declaration decorator boundary, superseded by issue 5276
date: 2026-05-08
```

Remaining risks:

- After issue 5276 lands, this reference may expose package.json section
  handling, tslib module resolution, decorator helper import semantics, or
  module-preserve emit parity blockers.
