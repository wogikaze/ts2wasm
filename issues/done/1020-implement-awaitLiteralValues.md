---
id: 1020
title: "Implement Awaitliteralvalues"
type: spike
area: reference/triage
class: superseded
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage awaitLiteralValues across 1 failing reference test case and split this generated bucket into a smaller implementation-ready child issue.

## Problem

Reference test results show 1 case fails in directory `awaitLiteralValues` with diagnostics: runtime-subset. Fresh triage shows the concrete blocker is reporting an `await` expression context error in ordinary non-async functions before the generic async runtime-subset diagnostic.

Problem: awaitLiteralValues has 1 reference failure that is now tracked by child issue 5147.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitLiteralValues.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/awaitLiteralValues.ts --detail
```

## Desired final state

This generated bucket is superseded by `issues/open/5147-report-await-expression-context-errors-before-runtime.md`.

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
- [x] Child issue 5147 contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/awaitLiteralValues.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitLiteralValues.ts
```

Not run:

- `cargo fmt --all --check`; issue split only, no Rust code changed
- `cargo nextest run`; issue split only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5147-report-await-expression-context-errors-before-runtime.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/awaitLiteralValues.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage runtime subset: awaitLiteralValues

- Issue class: `triage-needed`
- Feature label: `runtime-subset`
- Diagnostic: `UnsupportedRuntimeSubset` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/awaitLiteralValues.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitLiteralValues.ts
```

Failure location:

```json
{
  "code": "UnsupportedRuntimeSubset",
  "message": "issue-294: await is only supported for Bun.file(\"/dev/stdin\").text() stdin lowering in this slice at 50..65",
  "span_start": 50,
  "span_end": 65,
  "line": 3,
  "column": 7,
  "feature_label": "runtime-subset",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | function awaitString() {
3 |     await 'literal';
4 | }
5 |
6 | function awaitNumber() {
```

Compiler evidence:

```text
AST: Function awaitString body contains Expr::Await(String("literal")) at 50..65.
resolved: resolve_builtins reports issue-294 async runtime subset.
TypeScript oracle: TS1308 await expressions are only allowed within async functions and at top-level module contexts.
```

## Completion evidence

Commits:

- pending local commit

Validation result:

```text
command:
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/awaitLiteralValues.ts
result:
pass; emitted UnsupportedRuntimeSubset report for `await 'literal'` in a non-async function; split to issue 5147
date:
2026-05-06
```

Remaining risks:

- none
