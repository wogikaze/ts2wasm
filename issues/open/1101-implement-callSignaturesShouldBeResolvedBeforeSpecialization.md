---
id: 1101
title: "Implement Callsignaturesshouldberesolvedbeforespecialization"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5195]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Triage callSignaturesShouldBeResolvedBeforeSpecialization across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `callSignaturesShouldBeResolvedBeforeSpecialization` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: callSignaturesShouldBeResolvedBeforeSpecialization has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callSignaturesShouldBeResolvedBeforeSpecialization.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/callSignaturesShouldBeResolvedBeforeSpecialization.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/callSignaturesShouldBeResolvedBeforeSpecialization.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callSignaturesShouldBeResolvedBeforeSpecialization.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/done/5195-support-callable-interface-typed-local-calls.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/callSignaturesShouldBeResolvedBeforeSpecialization.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage function resolution: callSignaturesShouldBeResolvedBeforeSpecialization

- Issue class: `triage-needed`
- Feature label: `function-resolution`
- Diagnostic: `UnsupportedSyntax` / `issue-211`
- Path: `reference/typescript/tests/cases/compiler/callSignaturesShouldBeResolvedBeforeSpecialization.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callSignaturesShouldBeResolvedBeforeSpecialization.ts
```

Pipeline failure:

```text
error: [UnsupportedSyntax] issue-211: function-valued local calls such as extracted method `test(...)` are not supported; call receiver.method(...) directly at 143..184
```

Source context:

```ts
interface I1<T> {
    (value: T): void;
    field1: I1<boolean>;
}

function foo() {
    var test!: I1<string>;
    test("expects boolean instead of string");
    test(true);
}
```

Evidence:

- Tokens and AST succeed.
- AST body contains `Let test = Undefined`, `Call(Ident test, String ...)`,
  and `Call(Ident test, Bool true)`.
- TypeScript oracle specializes `I1<string>` and reports TS2345 for the boolean
  argument path.
- Existing child issue
  `issues/done/5195-support-callable-interface-typed-local-calls.md` owns the
  same generic callable interface local call boundary and has been expanded
  with this reference path.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...` pending

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/callSignaturesShouldBeResolvedBeforeSpecialization.ts
result: pass; reproduced issue-211 for callable interface-typed local `test(...)`
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

