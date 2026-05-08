---
id: 3451
title: "Implement Narrowedimports"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-01
status: done
---

## Summary

Closed as superseded by two existing import/export owners. Fresh triage shows
the two representatives stop at different established boundaries:
declaration-file exported const parsing and CommonJS `export =` parsing.

## Problem

Reference test results show 2 cases failing in directory `narrowedImports` with
diagnostics: import-export. Fresh evidence maps them to existing focused owners:
`narrowedImports.ts` to issue 5423 and
`narrowedImports_assumeInitialized.ts` to issue 5346.

Problem: narrowedImports has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowedImports_assumeInitialized.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowedImports_assumeInitialized.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede both representatives with existing focused owners
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the owner issues

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
- [x] Existing owner issues contain exact `reference-triage` commands
- [x] Evidence includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Owner issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowedImports_assumeInitialized.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowedImports_assumeInitialized.ts
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

- [x] `issues/open/5423a-parse-declaration-file-exported-const-declarations.md`
- [x] `issues/open/5346-parse-commonjs-export-assignment-statements.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowedImports_assumeInitialized.ts`
- `reference/typescript/tests/cases/compiler/narrowedImports.ts`

## Duplicate detection

- `reference/typescript/tests/cases/compiler/narrowedImports.ts` is
  superseded by `issues/open/5423a-parse-declaration-file-exported-const-declarations.md`.
- `reference/typescript/tests/cases/compiler/narrowedImports_assumeInitialized.ts`
  is superseded by `issues/open/5346-parse-commonjs-export-assignment-statements.md`.

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowedImports.ts --detail --no-dashboard-data
result: executed=1, build_pass=0, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=import-export:1

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowedImports_assumeInitialized.ts --detail --no-dashboard-data
result: executed=1, build_pass=0, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=import-export:1
```

Fresh triage for `narrowedImports.ts`:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowedImports.ts

result:
Feature label: import-export
Diagnostic code: UnsupportedSyntax
Message: const declarations require an initializer at 160..162
Failure line 8, column 14:
export const a1: number | undefined;
```

Fresh triage for `narrowedImports_assumeInitialized.ts`:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowedImports_assumeInitialized.ts

result:
Feature label: import-export
Diagnostic code: UnsupportedModule
Message: issue-055: unsupported static export; module resolution and loading are not implemented at 142..148
Current failing source:
export = a;
```

Compiler evidence:

```text
narrowedImports.ts: tokens ok through declare/export/import sections; AST fails at declaration-file `export const a1: number | undefined;`; TypeScript parses it and later reports TS1155/TS2395/module-resolution diagnostics.
narrowedImports_assumeInitialized.ts: tokens ok through declare namespace, export assignment, import-equals require, and `a.x`; AST fails at `export = a;`; TypeScript parses ExportAssignment and later reports missing virtual module `./a`.
```

## Completion evidence

Closed as superseded by issues 5423 and 5346; no new child issue created.

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowedImports.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, declaration-file exported const blocker
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowedImports.ts
result: pass; reproduced missing initializer at `export const a1`
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowedImports_assumeInitialized.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, issue-055 static export blocker
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowedImports_assumeInitialized.ts
result: pass; reproduced issue-055 at `export = a;`
date: 2026-05-08
```

Remaining risks:

- After issues 5423 and 5346 advance, these paths may expose virtual filename
  module resolution and narrowed import semantics.
