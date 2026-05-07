---
id: 1416
title: "Implement Concatclassandstring"
type: spike
area: frontend/resolver
class: done
priority: P1
depends_on: [5005]
blocks: [5300]
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1416.

## Summary

Triage concatClassAndString across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `concatClassAndString`.
Fresh triage on 2026-05-07 shows the file parses and reaches name
resolution; the remaining blocker is a TypeScript-compatible diagnostic for
assignment to a class binding.

Problem: concatClassAndString has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/concatClassAndString.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/concatClassAndString.ts --detail
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
- [x] Child issue 5300 contains an exact `mise run reference-triage -- ...` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/concatClassAndString.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/concatClassAndString.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] split to issue 5300

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/concatClassAndString.ts`

## Duplicate detection

- Issue 5192 is related but not a duplicate: it owns accepted class
  constructor value flow such as passing a class to a factory call.
- Issue 5011 is done and intentionally emits the current generic
  class-value guard; this bucket needs a narrower assignment-to-class
  diagnostic instead of reopening silent class-value support.
- Issue 661 is related because it contains more TS2629 compound assignment
  cases, but its current first blocker is parser support for `*=`.

## Smart triage

Generated on 2026-05-07:

```text
Feature label: class
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Path: reference/typescript/tests/cases/compiler/concatClassAndString.ts
Failure: issue-5011: class `f` used as a value is not yet supported; class declarations are partially supported (methods work, constructor/prototype/class-value not yet implemented) at 94..102
line 5, column 1
Source context:
1 | // @target: es2015
2 | // Shouldn't compile (the long form f = f + ""; doesn't):
3 | class f { }
4 |
5 | f += '';
Visible symbols before failure:
- class f
```

Compiler evidence:

```text
tokens: ok; Class Ident("f") LeftBrace RightBrace Ident("f") PlusEqual String("") Semicolon
ast: ok; ClassDecl { name: "f", ... } followed by Assign { name: "f", expr: Binary { left: Ident("f"), op: Add, right: String("") } }
resolved: UnsupportedSyntax issue-5011 class `f` used as a value at 94..102
TypeScript oracle: TS2629 Cannot assign to 'f' because it is a class.
```

Coverage result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/concatClassAndString.ts --detail --no-dashboard-data
result: executed=1, build_pass=0, unsupported=1, blocked=0, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=unknown-unsupported:1
date: 2026-05-07
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split to `issues/open/5300-report-assignment-to-class-binding-diagnostics.md`.

Validation result:

```text
command: python scripts/manager.py update-issue-index && python scripts/manager.py update-issue-index --check && python scripts/manager.py check-issue-health && python scripts/manager.py check-issue-readiness -- --fail-ready-below 80 && git diff --check
result: pass
date: 2026-05-07
```

Remaining risks:

- Child issue 5300 still needs implementation.
