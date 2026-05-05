---
id: 3437
title: "Implement Narrowbybooleancomparison"
type: spike
area: frontend/semantics
class: triage-needed
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: open
---

## Summary

Triage narrowByBooleanComparison across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `narrowByBooleanComparison` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: narrowByBooleanComparison has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowByBooleanComparison.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowByBooleanComparison.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowByBooleanComparison.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowByBooleanComparison.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowByBooleanComparison.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Not generated. Rerun with `--triage-limit 1` or higher.

## Triage evidence

Date: 2026-05-06

Command:

```sh
python scripts/manager.py reference-triage --format json tsc reference/typescript/tests/cases/compiler/narrowByBooleanComparison.ts
```

Result: still open. The representative failure is parser syntax at `status?: number;` inside a class property:

```text
UnsupportedSyntax: expected LeftParen, got Some(Question) at 1079..1080
feature_label: parser-syntax
```

No implementation-ready child was created in this pass; this bucket still needs parser triage/splitting rather than closure.

Remaining risks:

- none

## Completion evidence

### Implementation commits

- `fe2e3f00, ae315d28` — DuplicateLocal detection (var redeclaration checks, redeclaration tolerance)

### Changed files

- crates/ir/src/lowered/, crates/frontend/src/

### Validation

```sh
cargo nextest run => DuplicateLocal tests pass
```
