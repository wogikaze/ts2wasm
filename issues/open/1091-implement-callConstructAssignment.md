---
id: 1091
title: "Implement Callconstructassignment"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: [5193]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Triage callConstructAssignment across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `callConstructAssignment` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: callConstructAssignment has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callConstructAssignment.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/callConstructAssignment.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/callConstructAssignment.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callConstructAssignment.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] updated: `issues/open/5193-parse-asi-after-ambient-variable-declarations.md`

## Notes

Folded into `issues/open/5193-parse-asi-after-ambient-variable-declarations.md`.
Fresh triage shows `callConstructAssignment.ts` fails because the parser does
not accept ASI after declaration-only ambient variable type literals. It treats
the later `foo = bar` assignment as an ambient variable initializer and reports
`issue-400`.

## Affected test files

- `reference/typescript/tests/cases/compiler/callConstructAssignment.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated on 2026-05-06:

- command: `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/callConstructAssignment.ts`
- diagnostic: `UnsupportedTypeScriptSyntax`, `issue-400: ambient variable declarations with initializers would affect runtime bindings at 97..98`
- visible symbols: `foo`, `bar`
- source shape: `declare var foo:{ ( ):void; }`, `declare var bar:{ new ( ):any; }`, then `foo = bar;` and `bar = foo;`
- TypeScript oracle: TS2322 assignment incompatibilities for call-vs-construct signatures
- follow-up: `issues/open/5193-parse-asi-after-ambient-variable-declarations.md`

## Completion evidence

Closed as a generated triage bucket. The actionable parser blocker is tracked by
`issues/open/5193-parse-asi-after-ambient-variable-declarations.md`.

Commits:

- this fold commit

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/callConstructAssignment.ts
result: fail with issue-400 ambient variable initializer diagnostic caused by missing ASI handling
date: 2026-05-06
```

Remaining risks:

- Follow-up issue 5193 still needs implementation.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

