---
id: 1031
title: "Implement Badthisbinding"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage badThisBinding across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `badThisBinding` with diagnostics: arrow-function. Fresh smart triage shows parsing succeeds and the current blocker is the issue-289 class constructor lexical-capture diagnostic for calling outer `foo` from a constructor that nests arrow callbacks.

Problem: `badThisBinding` is not a standalone implementation order; the executable constructor capture slice is split to issue 5152.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badThisBinding.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/badThisBinding.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by an implementation-ready child issue. Do not implement directly from this bucket.

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
- [x] At least one child issue contains an exact `python scripts/manager.py reference-triage ...` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/badThisBinding.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/badThisBinding.ts
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

- [x] created: `issues/done/5152-support-class-constructor-outer-callback-captures.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/badThisBinding.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: class: badThisBinding

- Issue class: `triage-needed`
- Feature label: `class`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/badThisBinding.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badThisBinding.ts
```

Source context:

```text
 5 | class Greeter {
 6 |     constructor() {
 7 |         foo(() => {
 8 |             bar(() => {
 9 |                 var x = this;
10 |             });
```

Current compiler failure:

```text
[pipeline] resolve_builtins
error: [UnsupportedSyntax] issue-289: class constructor `constructor` references outer local `foo`; class constructor lexical captures require environment support at 132..135
```

TypeScript oracle evidence:

```text
TypeScript diagnostics: none.
AST path: ClassDeclaration -> Constructor -> CallExpression `foo(...)`.
```

Resolution:

```text
The current blocker is constructor lexical-capture environment support, not class parsing. Issue 5152 owns the narrow constructor callback capture slice.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- split to `issues/done/5152-support-class-constructor-outer-callback-captures.md`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badThisBinding.ts
result: pass; reproduced issue-289 constructor lexical-capture diagnostic
date: 2026-05-06
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

