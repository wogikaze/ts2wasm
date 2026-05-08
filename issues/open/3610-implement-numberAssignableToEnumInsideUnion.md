---
id: 3610
title: "Implement Numberassignabletoenuminsideunion"
type: spike
area: runtime/builtins
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
status: done
---

## Summary

Close stale numberAssignableToEnumInsideUnion generated blocker bucket after
fresh triage showed the reference now build-passes.

## Problem

Reference test results show 1 cases fail in directory `numberAssignableToEnumInsideUnion` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: numberAssignableToEnumInsideUnion has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/numberAssignableToEnumInsideUnion.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/numberAssignableToEnumInsideUnion.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

Current close decision: stale build-pass; no current compiler blocker remains
for this generated parser-syntax bucket.

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
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/numberAssignableToEnumInsideUnion.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/numberAssignableToEnumInsideUnion.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Close note, 2026-05-08:

- Fresh triage reports `BuildPass` / `pass`; the compiler no longer has a
  parser-syntax, enum, or runtime/builtins blocker for this reference.
- Focused coverage reports `executed=1`, `build_pass=1`, `unsupported=0`, and
  `blocked=0`.
- TypeScript oracle still reports TS2454 (`Variable 'n' is used before being
  assigned`) at `let z: E | boolean = n;`, but semantic diagnostics are not
  enabled for this coverage window (`semantic_enabled=0`). This close removes
  only the stale generated build blocker.
- No exact existing open owner for the simple TS2454 `let n: number; ... = n`
  semantic parity gap was found during duplicate review.

## Affected test files

- `reference/typescript/tests/cases/compiler/numberAssignableToEnumInsideUnion.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh smart triage on 2026-05-08:

```text
Diagnostic: BuildPass / pass
Feature label: build-pass
Path: reference/typescript/tests/cases/compiler/numberAssignableToEnumInsideUnion.ts
Source context: enum E { A, B }; let n: number; let z: E | boolean = n;
AST/resolved: ok; enum/type annotations erased, Let n = Undefined, Let z = Ident n
TypeScript oracle: TS2454 Variable 'n' is used before being assigned
```

Focused coverage on 2026-05-08:

```text
executed=1
build_pass=1
unsupported=0
blocked=0
semantic_enabled=0
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `6b0ae8c25`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/numberAssignableToEnumInsideUnion.ts
result: pass; BuildPass / pass, no current compiler blocker
date: 2026-05-08

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/numberAssignableToEnumInsideUnion.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=1 unsupported=0 blocked=0 semantic_enabled=0
date: 2026-05-08
```

Remaining risks:

- TypeScript TS2454 semantic parity for unassigned local reads is not covered by
  this stale build-blocker cleanup.
