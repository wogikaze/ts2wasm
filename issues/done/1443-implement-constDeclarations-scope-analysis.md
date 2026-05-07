---
id: 1443
title: "Implement Constdeclarations Scope Analysis"
type: spike
area: frontend/resolver
class: superseded
priority: P2
depends_on: [5310]
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---

## Summary

Closed this generated const-declaration scope-analysis bucket because fresh
triage stops at the nested-block parser boundary already owned by
`issues/open/5310-parse-nested-block-statements-with-variable-declarations.md`.

## Problem

Reference test results show 1 cases fail in directory `constDeclarations-scope-analysis` with diagnostics: scope-analysis. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: constDeclarations-scope-analysis has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constDeclarations-scopes.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constDeclarations-scopes.ts --detail
```

## Desired final state

This generated bucket is closed. Implement from
`issues/open/5310-parse-nested-block-statements-with-variable-declarations.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with the existing implementation-ready parser issue
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constDeclarations-scopes.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constDeclarations-scopes.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by: `issues/open/5310-parse-nested-block-statements-with-variable-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constDeclarations-scopes.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated on 2026-05-07.

- Path: `reference/typescript/tests/cases/compiler/constDeclarations-scopes.ts`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Current compiler message: `expected Comma, got Some(Ident("c")) at 1018..1019`
- Current failing construct: nested block containing `const c = false;`
- Visible symbols before failure include top-level `c`, `n`, and many prior block-local `c` bindings.
- TypeScript oracle parses the file and reports earlier strict-mode `with` and invalid-label diagnostics.
- Superseding issue: `issues/open/5310-parse-nested-block-statements-with-variable-declarations.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constDeclarations-scopes.ts
result: pass; current blocker identified as nested-block variable declaration parser dispatch, superseded by issue 5310
date: 2026-05-07
```

Remaining risks:

- Later triage may expose the intended scope-analysis diagnostics after issue 5310 parses nested blocks.
