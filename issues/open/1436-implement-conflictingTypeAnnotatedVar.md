---
id: 1436
title: "Implement Conflictingtypeannotatedvar"
type: spike
area: reference/triage
class: done
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1436.

## Summary

Triage conflictingTypeAnnotatedVar across 1 failing reference test case and
split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case fails in directory
`conflictingTypeAnnotatedVar` with diagnostics: duplicate-local. Fresh triage on
2026-05-07 shows this is a resolver diagnostic alignment issue for a typed
`var foo` followed by duplicate `function foo` declarations, split into issue
5307.

Problem: conflictingTypeAnnotatedVar has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conflictingTypeAnnotatedVar.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conflictingTypeAnnotatedVar.ts --detail
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

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Child issue 5307 contains an exact `mise run reference-triage -- ...` command
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conflictingTypeAnnotatedVar.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conflictingTypeAnnotatedVar.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5307-report-var-function-duplicate-identifier-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/conflictingTypeAnnotatedVar.ts`

## Duplicate detection

- `issues/open/5162-allow-compatible-var-redeclarations.md` is related but
  covers compatible `var` redeclarations, not this var/function conflict.
- `issues/open/5205a-report-incompatible-var-redeclaration-type-diagnostics.md`
  is related but starts after duplicate-local blockers are gone and covers
  repeated `var` type compatibility.
- Split into `issues/open/5307-report-var-function-duplicate-identifier-diagnostics.md`.

## Smart triage

### Smart triage: Triage duplicate local: conflictingTypeAnnotatedVar

- Issue class: `triage-needed`
- Feature label: `duplicate-local`
- Diagnostic: `DuplicateLocal` / `compiler-diagnostic`
- Path: `reference/typescript/tests/cases/compiler/conflictingTypeAnnotatedVar.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conflictingTypeAnnotatedVar.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 89,
  "lines": 4,
  "extension": ".ts",
  "first_code_line": "var foo: string;"
}
```

Failure location:

```json
{
  "code": "DuplicateLocal",
  "message": "top-level function `foo` conflicts with existing lexical binding at 38..46",
  "span_start": 38,
  "span_end": 46,
  "line": 3,
  "column": 3,
  "feature_label": "duplicate-local",
  "error_type": "compiler-diagnostic"
}
```

Source context:

```text
1 | // @target: es2015
2 | var foo: string;
3 | function foo(): number { }
4 | function foo(): number { }
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "foo",
    "line": 2,
    "column": 1
  }
]
```

Compiler dumps:

- tokens: ok
- ast: ok; contains `Let` for `foo` and two `Function` entries for `foo`
- resolved: fails in `validate_ast`

TypeScript oracle:

```text
TS2300: Duplicate identifier 'foo'.
TS2393: Duplicate function implementation.
TS2355: A function whose declared type is neither 'undefined', 'void', nor 'any' must return a value.
```

Coverage result:

```text
command: TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/conflictingTypeAnnotatedVar.ts --detail --no-dashboard-data
result: pass; executed=1 unsupported=1 unsupported_features=duplicate-local:1
date: 2026-05-07
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- split into `issues/open/5307-report-var-function-duplicate-identifier-diagnostics.md`

Validation result:

```text
command: python scripts/manager.py update-issue-index; python scripts/manager.py update-issue-index --check; python scripts/manager.py check-issue-health; python scripts/manager.py check-issue-readiness -- --fail-ready-below 80; git diff --check
result: pass
date: 2026-05-07
```

Remaining risks:

- none
