---
id: 1185
title: "Implement Classexpressiontest"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: [5202]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1185.

## Summary

Triage classExpressionTest across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `classExpressionTest` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: classExpressionTest has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExpressionTest2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionTest2.ts --detail
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
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionTest2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExpressionTest2.ts
```

Not run:

- cargo gates; issue close only, no implementation changes

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5202-parse-member-call-explicit-type-arguments.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classExpressionTest2.ts`
- `reference/typescript/tests/cases/compiler/classExpressionTest1.ts`

## Duplicate detection

- `issues/open/5202-parse-member-call-explicit-type-arguments.md` owns the current parser blocker: explicit TypeScript type arguments on member calls such as `v.f<string>()`.

## Smart triage

Fresh triage on 2026-05-06:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionTest1.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionTest2.ts
```

Result:

```text
classExpressionTest1.ts: UnsupportedSyntax: unsupported expression near the closing `)` of `return v.f<string>();`
classExpressionTest2.ts: UnsupportedSyntax: unsupported expression near the closing `)` of `return v.f<string>();`
```

Compiler dump evidence:

```text
tokens: ok; includes `new C<number>()` or `new m<number>()` and `v.f<string>()`
ast: fails at the explicit member-call type argument expression
resolved: same parser failure
```

TypeScript oracle:

```text
TS2454: Variable 't' is used before being assigned.
TS2454: Variable 'x' is used before being assigned.
```

The generated bucket is closed as superseded by issue 5202 because both
representative files reach the same member-call explicit type argument parser
boundary. No new child issue is needed.

## Completion evidence

Commits:

- local superseded-close commit for issue 1185

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionTest --detail --no-dashboard-data
result: pass; executed=2, unsupported=2, unsupported_features=unknown-unsupported:2
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionTest1.ts
result: pass; current blocker is member-call explicit type arguments, owned by issue 5202
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionTest2.ts
result: pass; current blocker is member-call explicit type arguments, owned by issue 5202
date: 2026-05-06
```

Remaining risks:

- After issue 5202 advances parsing, these references may expose TS2454 definite-assignment diagnostic parity or class generic erasure/lowering gaps.
