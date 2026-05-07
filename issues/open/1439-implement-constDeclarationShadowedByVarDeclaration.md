---
id: 1439
title: "Implement Constdeclarationshadowedbyvardeclaration"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1439.

## Summary

Triage constDeclarationShadowedByVarDeclaration across failing reference test
cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results previously showed 2 cases fail in directory
`constDeclarationShadowedByVarDeclaration` with diagnostics: parser-syntax.
Fresh focused coverage on 2026-05-07 shows
`constDeclarationShadowedByVarDeclaration3.ts` and `...2.ts` now build, while
`constDeclarationShadowedByVarDeclaration.ts` still fails on nested block
statement parsing. That blocker is split into issue 5310.

Problem: constDeclarationShadowedByVarDeclaration has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constDeclarationShadowedByVarDeclaration3.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constDeclarationShadowedByVarDeclaration3.ts --detail
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
- [x] Child issue 5310 contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constDeclarationShadowedByVarDeclaration3.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constDeclarationShadowedByVarDeclaration3.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5310-parse-nested-block-statements-with-variable-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constDeclarationShadowedByVarDeclaration3.ts`
- `reference/typescript/tests/cases/compiler/constDeclarationShadowedByVarDeclaration.ts`

## Duplicate detection

- `issues/open/5387-parse-function-expression-statements-in-nested-blocks.md`
  is related but covers function expression statements.
- `issues/open/5250-parse-class-declarations-in-nested-block-statements.md`
  is related but covers class declarations.
- Split into `issues/open/5310-parse-nested-block-statements-with-variable-declarations.md`.

## Smart triage

### Smart triage: Build pass: constDeclarationShadowedByVarDeclaration3

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/constDeclarationShadowedByVarDeclaration3.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constDeclarationShadowedByVarDeclaration3.ts
```

Result:

```text
ts2wasm build succeeded
```

### Smart triage: Triage parser syntax: constDeclarationShadowedByVarDeclaration

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/constDeclarationShadowedByVarDeclaration.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constDeclarationShadowedByVarDeclaration.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Comma, got Some(Ident(\"y\")) at 200..201",
  "span_start": 200,
  "span_end": 201,
  "line": 19,
  "column": 1,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
12 | var y = 0;
13 | {
14 |     const y = 0;
15 |     {
16 |         var y = 0;
17 |     }
18 | }
```

Compiler dumps:

- tokens: ok
- ast/resolved: fail with `UnsupportedSyntax: expected Comma, got Some(Ident("y"))`

TypeScript oracle:

```text
TS2481: Cannot initialize outer scoped variable 'y' in the same scope as block scoped declaration 'y'.
```

Coverage result:

```text
command: TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constDeclarationShadowedByVarDeclaration --detail --no-dashboard-data
result: pass; executed=3 build_pass=2 unsupported=1 unsupported_features=unknown-unsupported:1
date: 2026-05-07
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- split into `issues/open/5310-parse-nested-block-statements-with-variable-declarations.md`

Validation result:

```text
command: python scripts/manager.py update-issue-index; python scripts/manager.py update-issue-index --check; python scripts/manager.py check-issue-health; python scripts/manager.py check-issue-readiness -- --fail-ready-below 80; git diff --check
result: pass
date: 2026-05-07
```

Remaining risks:

- none
