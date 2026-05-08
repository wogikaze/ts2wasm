---
id: 1103
title: "Implement Callbacksdontsharetypes"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: [5202]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Triage callbacksDontShareTypes across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `callbacksDontShareTypes` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: callbacksDontShareTypes has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callbacksDontShareTypes.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/callbacksDontShareTypes.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/callbacksDontShareTypes.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callbacksDontShareTypes.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/done/5202-parse-member-call-explicit-type-arguments.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/callbacksDontShareTypes.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: callbacksDontShareTypes

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/callbacksDontShareTypes.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callbacksDontShareTypes.ts
```

Failure:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Greater) at 556..557
```

Source context:

```ts
interface Combinators {
    map<T, U>(c: Collection<T>, f: (x: T) => U): Collection<U>;
    map<T>(c: Collection<T>, f: (x: T) => any): Collection<any>;
}
var r5a = _.map<number, string>(c2, (x) => { return x.toFixed() });
var r5b = _.map<number, string>(c2, rf1);
```

Evidence:

- Tokenization succeeds.
- AST construction fails at the closing `>` in the explicit type argument list.
- Earlier ordinary member calls `_.map(c2, ...)` are visible before the failure.
- TypeScript oracle accepts the syntax and reports definite-assignment
  diagnostics for `_` and `c2`.
- Broad parser-syntax candidates are no-match owners for this exact member-call
  type argument blocker.
- Child issue `issues/done/5202-parse-member-call-explicit-type-arguments.md`
  owns the implementation slice.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...` pending

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/callbacksDontShareTypes.ts
result: pass; reproduced parser failure at member-call explicit type argument list
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

