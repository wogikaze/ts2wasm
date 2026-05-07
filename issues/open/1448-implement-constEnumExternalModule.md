---
id: 1448
title: "Implement Constenumexternalmodule"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5184]
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1448.

## Summary

Closed this generated external-module bucket because fresh triage stops at the
`const enum` parser boundary already owned by
`issues/done/5184-parse-const-enum-declarations.md`.

## Problem

Reference test results show 1 cases fail in directory `constEnumExternalModule` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: constEnumExternalModule has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constEnumExternalModule.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constEnumExternalModule.ts --detail
```

## Desired final state

This generated bucket is closed. Implement from
`issues/done/5184-parse-const-enum-declarations.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with the existing implementation-ready const-enum parser issue
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
- [x] Superseding issue contains exact `reference-triage` commands
- [x] Superseding issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Superseding issue acceptance names the exact reference path and diagnostic change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constEnumExternalModule.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constEnumExternalModule.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by: `issues/done/5184-parse-const-enum-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constEnumExternalModule.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated on 2026-05-07.

- Path: `reference/typescript/tests/cases/compiler/constEnumExternalModule.ts`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Current compiler message: `const declarations require an initializer at 62..66`
- First failing source: `const enum E {`
- Token evidence includes `Const`, `Ident("enum")`, `Ident("E")`, enum body tokens, `Export`, `Equal`, `Import`, `require`, and `A.V`.
- Visible symbol extraction reports a bogus binding named `enu`.
- TypeScript oracle parses `EnumDeclaration`, `ExportAssignment`, `ImportEqualsDeclaration`, and `var v = A.V`, then reports TS2307 for `m1`.
- Superseding issue: `issues/done/5184-parse-const-enum-declarations.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumExternalModule.ts
result: pass; current blocker is the same const enum parser support tracked by issue 5184
date: 2026-05-07
```

Remaining risks:

- Later triage may expose export assignment, import-equals, or module resolution behavior after issue 5184 parses const enums.
