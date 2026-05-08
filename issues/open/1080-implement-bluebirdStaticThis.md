---
id: 1080
title: "Implement Bluebirdstaticthis"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5190]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage bluebirdStaticThis across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `bluebirdStaticThis` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: bluebirdStaticThis has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/bluebirdStaticThis.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/bluebirdStaticThis.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/bluebirdStaticThis.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/bluebirdStaticThis.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] added: `issues/open/5190-skip-implements-in-ambient-class-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/bluebirdStaticThis.ts`

## Duplicate detection

- Broad parser syntax buckets are not matches; this is a focused ambient `declare class ... implements` parser gap.
- Existing class heritage generic issue `5156` covers generic type arguments in `extends`, not ambient `implements` skipping.

## Smart triage

Generated on 2026-05-06.

- Path: `reference/typescript/tests/cases/compiler/bluebirdStaticThis.ts`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Current compiler message: `expected LeftBrace, got Some(Ident("implements")) at 287..297`
- Source context: `export declare class Promise<R> implements Promise.Thenable<R> { ... }`
- Compiler evidence: tokens include `implements Promise.Thenable<R>` and the following `{`; AST/resolved construction fails before the ambient class body is skipped.
- TypeScript oracle: `TS2420: Class 'Promise<R>' incorrectly implements interface 'Thenable<R>'.`
- Superseding child: `issues/open/5190-skip-implements-in-ambient-class-declarations.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bluebirdStaticThis.ts
result: pass; current blocker is ambient class implements parsing, split to issue 5190
date: 2026-05-06
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/open/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

