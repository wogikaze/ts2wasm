---
id: 4228
title: "Implement Standalonebreak"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [035]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage standaloneBreak across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `standaloneBreak` with diagnostics: switch. Fresh smart triage shows the compiler already parses `break;` and reports `break must be inside a loop or switch` at the standalone statement, while TypeScript reports TS1105 at the same span.

Problem: `standaloneBreak` is not a standalone implementation order; the current failure is an oracle-matching invalid-break diagnostic covered by issue 035 break/continue behavior.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/standaloneBreak.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/standaloneBreak.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by `issues/done/035-implement-break-continue.md` for the current invalid-break diagnostic. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 035's break/continue diagnostic behavior
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue

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
- [x] This closed issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] This closed issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/standaloneBreak.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/standaloneBreak.ts
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

- `reference/typescript/tests/cases/compiler/standaloneBreak.ts`

## Duplicate detection

- `issues/done/1084-implement-breakNotInIterationOrSwitchStatement.md` has the same standalone `break;` evidence and was closed in the same triage slice.
- `issues/done/035-implement-break-continue.md` owns break/continue statement support and the current invalid-break diagnostic behavior.
- Switch implementation issues are not matches: no switch syntax is present in this reference case.

## Smart triage

### Smart triage: switch: standaloneBreak

- Issue class: `triage-needed`
- Feature label: `switch`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/standaloneBreak.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/standaloneBreak.ts
```

Source context:

```text
1 | // @target: es2015
2 | break;
```

Current compiler failure:

```text
error: [UnsupportedSyntax] break must be inside a loop or switch at 19..25
```

Compiler evidence:

- Tokens succeed and contain `Break` followed by `Semicolon`.
- AST succeeds as `Break { label: None, span: 19..25 }`.
- Validation rejects the standalone break before lowering.

TypeScript oracle evidence:

```text
TS1105: A 'break' statement can only be used within an enclosing iteration or switch statement.
AST path: SourceFile -> BreakStatement.
```

Resolution:

```text
The current compiler diagnostic is an expected invalid-break diagnostic at the same source span as TypeScript's TS1105. No new implementation child is created from this generated bucket.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- superseded by `issues/done/035-implement-break-continue.md`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/standaloneBreak.ts
result: pass; reproduced oracle-matching invalid standalone-break diagnostic
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

