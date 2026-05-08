---
id: 1472
title: "Implement Constructorinvocationwithtoofewtypeargs"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5001]
blocks: [5356]
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1472.

## Summary

Closed by splitting the current concrete false-pass blocker to
`issues/done/5356-report-uninitialized-generic-class-fields.md`.

Fresh coverage shows `constructorInvocationWithTooFewTypeArgs.ts` now builds in
ts2wasm. TypeScript still reports TS2564 for uninitialized fields `x` and `y`
before the later TS2558 too-few-type-arguments diagnostic.

## Problem

Reference test results originally showed one `arguments-object` failure. Fresh
focused triage on 2026-05-07 reports `BuildPass` instead.

Problem: `constructorInvocationWithTooFewTypeArgs.ts` needed a focused child for
the current false build pass before type-argument count checking can be triaged.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorInvocationWithTooFewTypeArgs.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorInvocationWithTooFewTypeArgs.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
coverage: executed=1, build_pass=1, unsupported=0
triage: BuildPass: ts2wasm build succeeded
```

TypeScript oracle diagnostics:

```text
TS2564: Property 'x' has no initializer and is not definitely assigned in the constructor.
TS2564: Property 'y' has no initializer and is not definitely assigned in the constructor.
TS2558: Expected 2 type arguments, but got 1.
```

Compiler evidence:

```text
tokens: ok through class D<T, U>, typed fields x/y, and new D<number>()
ast: ok; ClassDecl D body is empty and Let d = New D is retained
resolved: ok; New D resolves
```

## Desired final state

This generated bucket is closed. Implementation proceeds through issue 5356,
which owns the first false-pass diagnostic in this reference case.

## Scope

In scope:

- [x] Inspect the smart triage report
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split the current observable false-pass behavior into child issue 5356
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- TS2558 generic constructor type-argument count checking
- Full strict-property-initialization parity beyond the representative slice

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
- [x] Child issue 5356 contains exact `reference-triage` and `reference-coverage` commands
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorInvocationWithTooFewTypeArgs.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorInvocationWithTooFewTypeArgs.ts
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

- [x] created: `issues/done/5356-report-uninitialized-generic-class-fields.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constructorInvocationWithTooFewTypeArgs.ts`

## Duplicate detection

- No exact open implementation-ready owner was found for TS2564 strict property
  initialization diagnostics in this representative false-pass case.
- Several open and done issues mention TS2564 as later oracle diagnostics, but
  they do not own this minimal generic class field false-pass slice.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Build pass: constructorInvocationWithTooFewTypeArgs

- Issue class: none
- Feature label: build-pass
- Diagnostic: BuildPass / pass
- Path: reference/typescript/tests/cases/compiler/constructorInvocationWithTooFewTypeArgs.ts
```

## Completion evidence

Commits:

- filled by local commit that moves this issue to `done/`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorInvocationWithTooFewTypeArgs.ts --detail --no-dashboard-data
result: pass; reproduced executed=1 build_pass=1 unsupported=0
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorInvocationWithTooFewTypeArgs.ts
result: pass; reproduced BuildPass while TypeScript reports TS2564 and TS2558
date: 2026-05-07
```

Remaining risks:

- implementation remains tracked by issue 5356
