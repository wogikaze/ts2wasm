---
id: 1057
title: "Implement Bind"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5172]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage bind across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `bind` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: bind has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/bind1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/bind1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/bind1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/bind1.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] added: `issues/open/5172-report-unresolved-implements-in-erased-namespace.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/bind1.ts`

## Duplicate detection

- `issues/open/432-implement-import-export.md` is not a match for the current blocker: `bind1.ts` contains a TypeScript namespace declaration and now builds, while the remaining mismatch is a missing `TS2304` diagnostic.
- No open issue was found for unresolved `implements I` inside an erased namespace body.

## Smart triage

Generated on 2026-05-06.

- Path: `reference/typescript/tests/cases/compiler/bind1.ts`
- Diagnostic: `BuildPass` / `pass`
- Failure: ts2wasm builds successfully while TypeScript reports `TS2304: Cannot find name 'I'.`
- Source context: `namespace M { export class C implements I {} }`
- Visible symbols before mismatch: class `C`
- Compiler evidence: tokens include `namespace`, `export`, `class`, `implements`, and `I`; AST and resolved dumps are empty because the namespace body is erased.
- TypeScript oracle: reports `TS2304` at the `I` token.
- Superseding child: `issues/open/5172-report-unresolved-implements-in-erased-namespace.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bind1.ts
result: pass; current mismatch identified as a false build pass for unresolved `implements I`, split to issue 5172
date: 2026-05-06
```

Remaining risks:

- Namespace erasure may hide additional TypeScript diagnostics in larger files after issue 5172 handles this focused case.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

