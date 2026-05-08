---
id: 1094
title: "Implement Callofconditionaltypewithconcretebranches"
type: spike
area: frontend/semantics
class: blocked
priority: P1
depends_on: [5196]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Triage callOfConditionalTypeWithConcreteBranches across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `callOfConditionalTypeWithConcreteBranches` with diagnostics: type-system. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: callOfConditionalTypeWithConcreteBranches has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOfConditionalTypeWithConcreteBranches.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/callOfConditionalTypeWithConcreteBranches.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/callOfConditionalTypeWithConcreteBranches.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOfConditionalTypeWithConcreteBranches.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5196-support-callable-conditional-typed-parameter-calls.md`

## Notes

Superseded by `issues/open/5196-support-callable-conditional-typed-parameter-calls.md`.
Fresh triage shows this bucket is no longer parser-owned; the source parses and
then lowering reports `issue-211` for the callable conditional-typed parameter
call `arg(10)`.

## Affected test files

- `reference/typescript/tests/cases/compiler/callOfConditionalTypeWithConcreteBranches.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated on 2026-05-06:

- command: `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/callOfConditionalTypeWithConcreteBranches.ts`
- diagnostic: `UnsupportedSyntax`, `issue-211: function-valued local calls such as extracted method arg(...) are not supported`
- AST: `Function fn` with parameter `arg`; body contains `Call(Ident arg, Number 10)`
- TypeScript oracle: ok, diagnostics: []
- follow-up: `issues/open/5196-support-callable-conditional-typed-parameter-calls.md`

## Completion evidence

Closed as a generated triage bucket. The actionable callable conditional-typed
parameter call gap is tracked by
`issues/open/5196-support-callable-conditional-typed-parameter-calls.md`.

Commits:

- this split commit

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/callOfConditionalTypeWithConcreteBranches.ts
result: fail with issue-211 function-valued local call diagnostic after parse/name-resolution
date: 2026-05-06
```

Remaining risks:

- Follow-up issue 5196 still needs implementation.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/open/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

