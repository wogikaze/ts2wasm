---
id: 1209
title: "Implement Classfieldsupernotaccessible"
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
> Evidence: Empty completion evidence. No feat/fix commit for #1209.

## Summary

Triage classFieldSuperNotAccessible across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh smart triage shows this file parses and reaches lowering. The current
gap is a TypeScript TS2855 diagnostic for calling a parent class field through
`super`.

Problem: classFieldSuperNotAccessible was a stale generated bucket and needed
smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classFieldSuperNotAccessible.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classFieldSuperNotAccessible.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classFieldSuperNotAccessible.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classFieldSuperNotAccessible.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5321-report-super-class-field-access-diagnostic.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classFieldSuperNotAccessible.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated 2026-05-07:

```sh
TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm \
  python scripts/manager.py reference-triage tsc \
  reference/typescript/tests/cases/compiler/classFieldSuperNotAccessible.ts
```

Result:

```text
diagnostic: UnsupportedSyntax / class
message: super method `T.field` not found at 91..104
source: super.field()
TypeScript oracle:
TS2855 Class field 'field' defined by the parent class is not accessible in the child class via super.
```

Compiler evidence:

- Tokens and AST succeed.
- AST has `ClassDecl T2 extends T` and `super.field()` as a member call.
- Lowering treats the parent field as a missing super method.

Duplicate review:

- `issues/done/5255-resolve-super-property-accesses.md` is related but broader
  to `super.x` receiver resolution.
- No exact open/done owner was found for the TS2855 parent class field via
  `super` diagnostic.

## Completion evidence

This generated bucket was split and closed as superseded by issue 5321.

Commits:

- this commit

Validation result:

```text
command: TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classFieldSuperNotAccessible.ts
result: pass; current blocker split to TS2855 parent class-field super diagnostic
date: 2026-05-07

command: TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classFieldSuperNotAccessible.ts --detail --no-dashboard-data
result: pass; single-file window reports UnsupportedSyntax: class
date: 2026-05-07
```

Remaining risks:

- Implementation remains open in issue 5321.
