---
id: 1444
title: "Implement Constdeclarations Unknown Unsupported"
type: spike
area: frontend/syntax
class: superseded
priority: P2
depends_on: [5350]
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1444.

## Summary

Closed this generated unknown-unsupported bucket after splitting the current
missing const initializer diagnostic boundary to
`issues/open/5350-report-missing-const-initializer-diagnostics.md`.

## Problem

Reference test results show 1 cases fail in directory `constDeclarations-unknown-unsupported` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: constDeclarations-unknown-unsupported has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constDeclarations-errors.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constDeclarations-errors.ts --detail
```

## Desired final state

This generated bucket is closed. Implement from
`issues/open/5350-report-missing-const-initializer-diagnostics.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one observable diagnostic boundary into an implementation-ready child issue
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
- [x] Child issue contains exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact reference path and diagnostic change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constDeclarations-errors.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constDeclarations-errors.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5350-report-missing-const-initializer-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constDeclarations-errors.ts`

## Duplicate detection

- `issues/open/694-implement-arrayToLocaleStringES-unknown-unsupported.md` - Implement Arraytolocalestringes Unknown Unsupported (same feature label, title overlap)

## Smart triage

Generated on 2026-05-07.

- Path: `reference/typescript/tests/cases/compiler/constDeclarations-errors.ts`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Current compiler message: `const declarations require an initializer at 74..76`
- First failing source: `const c1;`
- Visible symbol before failure: binding `c1`
- TypeScript oracle parses the file and reports TS1155 for `c1`, `c2`, `c3`, `c4`, `c5`, `c6`, `c9`, and `c11`, plus TS2588 for assigning to `c8`.
- Superseding child: `issues/open/5350-report-missing-const-initializer-diagnostics.md`

Nearby non-owners:

- `issues/done/5264-parse-typed-const-declarations-before-initializers.md` covers typed const declarations with valid initializers, not missing-initializer diagnostics.
- `issues/done/5184-parse-const-enum-declarations.md` covers `const enum`, not ordinary const variable declarations.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constDeclarations-errors.ts
result: pass; current blocker identified as missing const initializer diagnostics, split to issue 5350
date: 2026-05-07
```

Remaining risks:

- Later triage may expose const assignment/update diagnostics or for-loop const semantics after issue 5350 advances past missing initializers.
