---
id: 1470
title: "Implement Constructorargserrors"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5001]
blocks: [5355]
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1470.

## Summary

Closed by splitting the current concrete blocker to
`issues/done/5355-report-invalid-constructor-parameter-modifiers.md`.

Fresh coverage shows the group now contains two stale build-pass files and three
shared parser/diagnostic failures for invalid constructor parameter modifiers.

## Problem

Reference test results originally showed 3 failing cases with
`arguments-object`. Fresh focused coverage on 2026-05-07 still shows 3
unsupported files, but triage proves the common blocker is invalid constructor
parameter modifiers (`static`, `public static`, and `export`), not arguments
object runtime semantics.

Problem: `constructorArgsErrors` needed a focused implementation-ready child
issue for invalid constructor parameter modifier diagnostics.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorArgsErrors2.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorArgsErrors --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
executed=5
build_pass=2
unsupported=3
unsupported_diagcodes=UnsupportedSyntax:3
unsupported_features=arguments-object:3
```

Failing variants:

```text
constructorArgsErrors1.ts: constructor (static a: number)
current: issue-247 expected binding identifier or pattern, got Some(Static)
oracle: TS1090 "'static' modifier cannot appear on a parameter."

constructorArgsErrors2.ts: constructor (public static a: number)
current: expected Comma, got Some(Static)
oracle: TS1090 "'static' modifier cannot appear on a parameter."

constructorArgsErrors5.ts: constructor (export a: number)
current: issue-247 expected binding identifier or pattern, got Some(Export)
oracle: TS1090 "'export' modifier cannot appear on a parameter."
```

## Desired final state

This generated bucket is closed. Implementation proceeds through issue 5355,
which owns the exact parser diagnostic family.

## Scope

In scope:

- [x] Inspect the smart triage reports
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split the current feature family into child issue 5355
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Arguments-object runtime behavior
- Constructor overload or arity checking

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
- [x] Child issue 5355 contains exact `reference-triage` commands
- [x] Child issue includes failing paths, diagnostic codes, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names exact reference paths and diagnostic/stdout changes

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorArgsErrors --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorArgsErrors1.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorArgsErrors2.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorArgsErrors5.ts
```

Not run:

- `cargo fmt --all --check` (issue metadata only)
- `cargo nextest run` (issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5355-report-invalid-constructor-parameter-modifiers.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constructorArgsErrors1.ts`
- `reference/typescript/tests/cases/compiler/constructorArgsErrors2.ts`
- `reference/typescript/tests/cases/compiler/constructorArgsErrors5.ts`
- `reference/typescript/tests/cases/compiler/constructorArgsErrors3.ts` (fresh build pass)
- `reference/typescript/tests/cases/compiler/constructorArgsErrors4.ts` (fresh build pass)

## Duplicate detection

- `issues/done/226-implement-parameter-properties.md` covers valid constructor
  parameter properties, not invalid modifier diagnostics.
- No existing open issue was found for TS1090-style `static` or `export`
  modifier diagnostics on parameters.

## Smart triage

Generated 2026-05-07 for files 1, 2, and 5.

```text
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Feature label: arguments-object
Parser evidence: tokens ok, AST fails before construction
TypeScript oracle: parses Constructor/Parameter and reports TS1090
```

## Completion evidence

Commits:

- filled by local commit that moves this issue to `done/`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorArgsErrors --detail --no-dashboard-data
result: pass; reproduced executed=5 build_pass=2 unsupported=3 UnsupportedSyntax=3
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorArgsErrors1.ts
result: pass; reproduced invalid static parameter modifier parser failure
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorArgsErrors2.ts
result: pass; reproduced invalid public static parameter modifier parser failure
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorArgsErrors5.ts
result: pass; reproduced invalid export parameter modifier parser failure
date: 2026-05-07
```

Remaining risks:

- implementation remains tracked by issue 5355
