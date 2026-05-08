---
id: 1034
title: "Implement Baseclassimprovedmismatcherrors"
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

Triage baseClassImprovedMismatchErrors across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `baseClassImprovedMismatchErrors` with diagnostics: parser-syntax. Fresh smart triage shows the current blocker is `as number | string` being parsed as a runtime bitwise-or expression after an `as` assertion.

Problem: `baseClassImprovedMismatchErrors` is not a standalone implementation order; the executable parser erasure slice is split to issue 5153.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseClassImprovedMismatchErrors.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/baseClassImprovedMismatchErrors.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/baseClassImprovedMismatchErrors.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/baseClassImprovedMismatchErrors.ts
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

- [x] created: `issues/done/5153-erase-union-types-in-as-assertions.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/baseClassImprovedMismatchErrors.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: operator: baseClassImprovedMismatchErrors

- Issue class: `triage-needed`
- Feature label: `operator`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/baseClassImprovedMismatchErrors.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseClassImprovedMismatchErrors.ts
```

Source context:

```text
class Derived extends Base {
    n: Derived | string;
    fn() {
        return 10 as number | string;
    }
}
```

Current compiler failure:

```text
error: [UnsupportedSyntax] binary operator BitwiseOr not yet supported
```

Compiler evidence:

```text
The AST for `return 10 as number | string;` contains a runtime `Binary` expression with `op: BitwiseOr`, left `10`, and right identifier `string`.
```

TypeScript oracle evidence:

```text
TypeScript parses the expression as an `as` assertion with union type text `number | string`.
The later TypeScript diagnostics are class member mismatch errors, not a runtime bitwise operator.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- split to `issues/done/5153-erase-union-types-in-as-assertions.md`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseClassImprovedMismatchErrors.ts
result: pass; reproduced runtime BitwiseOr lowering failure from `as number | string`
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

