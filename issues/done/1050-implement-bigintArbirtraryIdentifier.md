---
id: 1050
title: "Implement Bigintarbirtraryidentifier"
type: spike
area: runtime/builtins
class: superseded
priority: P1
depends_on: [5166]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage bigintArbirtraryIdentifier across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `bigintArbirtraryIdentifier` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: bigintArbirtraryIdentifier has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/bigintArbirtraryIdentifier.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/bigintArbirtraryIdentifier.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/bigintArbirtraryIdentifier.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/bigintArbirtraryIdentifier.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5166-parse-string-literal-module-specifier-aliases.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/bigintArbirtraryIdentifier.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated on 2026-05-06.

- Path: `reference/typescript/tests/cases/compiler/bigintArbirtraryIdentifier.ts`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Failure: `expected identifier, got Some(SpannedToken { kind: String("0n"), span: Span { start: 97, end: 101 } })`
- Source context: `export { foo as "0n" };`
- Visible symbols before failure: none
- Compiler evidence: lexer emits `String("0n")`; AST/resolved fail before representing the valid arbitrary string-literal export name.
- TypeScript oracle: accepts `export { foo as "0n" }` / `import { "0n" as foo }`, but reports diagnostics for later BigInt literal specifier forms.
- Split child: `issues/done/5166-parse-string-literal-module-specifier-aliases.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigintArbirtraryIdentifier.ts
result: pass; current blocker identified as string-literal import/export specifier parsing, split to issue 5166
date: 2026-05-06
```

Remaining risks:

- The reference file also includes intentionally invalid BigInt literal specifier cases that should remain rejected after issue 5166 advances past the valid string-literal aliases.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

