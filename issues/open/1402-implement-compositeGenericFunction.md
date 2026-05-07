---
id: 1402
title: "Implement Compositegenericfunction"
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
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1402.

## Summary

Closed as a stale generated bucket after fresh triage showed the representative
now builds successfully.

`compositeGenericFunction.ts` no longer reports the generated
`duplicate-local` blocker. Current focused coverage reports `build_pass=1`,
`unsupported=0`, and `blocked=0`.

## Problem

Reference test results originally showed 1 case failing in directory
`compositeGenericFunction` with diagnostics: duplicate-local. Fresh focused
triage on 2026-05-07 reports `BuildPass`.

Problem: the generated duplicate-local/compiler blocker is stale; there is no
current compiler blocker to split into an implementation-ready child issue.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/compositeGenericFunction.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/compositeGenericFunction.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
Smart triage: Build pass: compositeGenericFunction
coverage: executed=1, build_pass=1, unsupported=0, blocked=0
semantic_enabled=0
```

TypeScript oracle evidence:

```text
typescriptVersion: 6.0.3
diagnostics: TS2322 at h<R> returning null as R
```

## Desired final state

This generated bucket is closed. No implementation issue is created because the
current compiler build accepts the representative source; TypeScript still
reports a semantic diagnostic, but semantic parity is not enabled for this
coverage window.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close this stale generated bucket
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Creating a child issue without a current compiler blocker
- TypeScript semantic parity for TS2322 generic return assignability

## Affected paths

Expected:

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Focused triage reports `BuildPass`
- [x] Focused coverage reports `build_pass=1`
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/compositeGenericFunction.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/compositeGenericFunction.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/compositeGenericFunction.ts`

## Duplicate detection

Fresh smart triage found only this same issue as a duplicate candidate. No
matching implementation-ready issue is needed because the current build passes.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Build pass: compositeGenericFunction

- Issue class: none
- Feature label: build-pass
- Diagnostic: BuildPass / pass
- Path: reference/typescript/tests/cases/compiler/compositeGenericFunction.ts
```

Source context:

```ts
function f<T>(value: T) { return value; };

function h<R>(func: (x: number) => R): R { return null; }

var z: number = h<number>(f);
var z: number = h(f);
```

Compiler evidence:

```text
tokens: ok; includes generic functions f/h and duplicate var z declarations
ast: ok; Function f, Function h, two is_var Let z declarations
resolved: ok; duplicate var z declarations are accepted in the resolved dump
```

TypeScript oracle evidence:

```text
TS2322: Type 'null' is not assignable to type 'R'.
```

Coverage result:

```text
executed=1
build_pass=1
unsupported=0
blocked=0
semantic_enabled=0
```

## Completion evidence

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/compositeGenericFunction.ts
result: pass; BuildPass, no compiler blocker found
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/compositeGenericFunction.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0, blocked=0
date: 2026-05-07
```

Remaining risks:

- `semantic_enabled=0` for this focused tsc coverage run, so this closure only
  claims the generated duplicate-local/compiler blocker is gone, not TS2322
  semantic parity.
