---
id: 1438
title: "Implement Consistentaliasvsnonaliasrecordbehavior"
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
> Evidence: Empty completion evidence. No feat/fix commit for #1438.

## Summary

Triage consistentAliasVsNonAliasRecordBehavior across 1 failing reference test
case and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case fails in directory
`consistentAliasVsNonAliasRecordBehavior` with diagnostics: type-alias. Fresh
triage on 2026-05-07 shows the leading type alias now parses; the current
blocker is generic type arguments inside function parameter annotations, split
into issue 5309.

Problem: consistentAliasVsNonAliasRecordBehavior has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/consistentAliasVsNonAliasRecordBehavior.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/consistentAliasVsNonAliasRecordBehavior.ts --detail
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
- [x] Child issue 5309 contains an exact `mise run reference-triage -- ...` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/consistentAliasVsNonAliasRecordBehavior.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/consistentAliasVsNonAliasRecordBehavior.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5309-skip-generic-type-arguments-in-type-annotations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/consistentAliasVsNonAliasRecordBehavior.ts`

## Duplicate detection

- `issues/open/345-implement-tsc-type-alias-coverage.md` is the broad parent.
  This bucket is now narrowed to a parser helper gap rather than broad
  type-alias erasure.
- Split into `issues/done/5309-skip-generic-type-arguments-in-type-annotations.md`.

## Smart triage

### Smart triage: Triage type alias: consistentAliasVsNonAliasRecordBehavior

- Issue class: `triage-needed`
- Feature label: `type-alias`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/consistentAliasVsNonAliasRecordBehavior.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/consistentAliasVsNonAliasRecordBehavior.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 1141,
  "lines": 40,
  "extension": ".ts",
  "first_code_line": "type Record2<K extends keyof any, T> = {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Comma, got Some(Greater) at 355..356",
  "span_start": 355,
  "span_end": 356,
  "line": 10,
  "column": 54,
  "feature_label": "type-alias",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
 6 | type Record2<K extends keyof any, T> = {
 7 |     [P in K]: T;
 8 | };
 9 |
10 | function defaultRecord(x: Record<'a', string>, y: Record<string, string>) {
```

Compiler dumps:

- tokens: ok; `Record`, `<`, string literal `'a'`, comma, `string`, `>`
- ast/resolved: fail with `UnsupportedSyntax: expected Comma, got Some(Greater)`

TypeScript oracle:

```text
TS2741: Property 'a' is missing in type 'Record<string, string>' but required in type 'Record2<"a", string>'.
```

Coverage result:

```text
command: TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/consistentAliasVsNonAliasRecordBehavior.ts --detail --no-dashboard-data
result: pass; executed=1 unsupported=1 unsupported_features=type-alias:1
date: 2026-05-07
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- split into `issues/done/5309-skip-generic-type-arguments-in-type-annotations.md`

Validation result:

```text
command: python scripts/manager.py update-issue-index; python scripts/manager.py update-issue-index --check; python scripts/manager.py check-issue-health; python scripts/manager.py check-issue-readiness -- --fail-ready-below 80; git diff --check
result: pass
date: 2026-05-07
```

Remaining risks:

- none
