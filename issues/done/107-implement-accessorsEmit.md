---
id: 107
title: "Implement Accessorsemit"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5183]
blocks: []
created: 2026-04-29
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage accessorsEmit across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `accessorsEmit` with diagnostics: class-accessor. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: accessorsEmit has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorsEmit.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorsEmit.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorsEmit.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorsEmit.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] added: `issues/open/5183-report-typed-getter-null-return-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/accessorsEmit.ts`

## Duplicate detection

- Older accessor buckets share the `class-accessor` feature label, but none owns the current typed getter false build-pass.
- `issues/open/574-implement-accessors.md` is a broad generated accessor bucket, not an implementation-ready child for this current `TS2322` mismatch.

## Smart triage

Generated on 2026-05-06.

- Path: `reference/typescript/tests/cases/compiler/accessorsEmit.ts`
- Diagnostic: `BuildPass` / `pass`
- Current compiler result: `ts2wasm build succeeded`
- Source context: `class Test { get Property(): Result { var x = 1; return null; } }`
- Visible symbols before mismatch: classes `Result`, `Test`, `Test2`, and getter-local bindings `x`
- Compiler evidence: tokens include `get`, `Property`, `:`, `Result`, and `return null`; AST and resolved output represent the getter as a class method named `get Property`, but the return type annotation is not preserved for diagnostics.
- TypeScript oracle: `TS2322: Type 'null' is not assignable to type 'Result'.` at the first getter's `return null`.
- Superseding child: `issues/open/5183-report-typed-getter-null-return-diagnostics.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/accessorsEmit.ts
result: pass; current mismatch identified as typed getter return diagnostic hidden by BuildPass, split to issue 5183
date: 2026-05-06
```

Remaining risks:

- Later triage may expose actual accessor runtime emit or property descriptor semantics after issue 5183 handles the typed-return diagnostic.
---

## ⚠️ False-done audit (re-opened from issues/done/)

**Why this was false-done**: This generated triage spike issue was copy-closed to `issues/done/` as part of a batch close cycle without actual triage completion. The done/ copy only differs from open/ in checkbox state ([ ] → [x]) with no "Status" note, no child issues created, no implementation commits, and empty completion evidence. The checkboxes were batch-checked without evidence that the triage was actually performed.

**True-done checklist** (all must pass):

1. Perform actual triage review of the reference failure case
2. Either create child implementation issue(s) or confirm this issue is superseded by an existing issue (with "Status" note)
3. Fill in completion evidence section with triage results
4. Remove stale open/ copy if it exists

**Commands that must pass**:

```sh
cargo fmt --all --check
cargo nextest run
```

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

