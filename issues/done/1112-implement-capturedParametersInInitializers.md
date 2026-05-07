---
id: 1112
title: "Implement Capturedparametersininitializers"
type: spike
area: frontend/syntax
class: done
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
---

## Summary

Triage capturedParametersInInitializers across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `capturedParametersInInitializers` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: capturedParametersInInitializers has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedParametersInInitializers2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/capturedParametersInInitializers2.ts --detail
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
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/capturedParametersInInitializers2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedParametersInInitializers2.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5213-parse-generator-function-expressions-in-parameter-initializers.md`
- [x] `issues/open/5214-parse-computed-class-members-in-class-expression-initializers.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/capturedParametersInInitializers2.ts`
- `reference/typescript/tests/cases/compiler/capturedParametersInInitializers1.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh coverage for `capturedParametersInInitializers` executed 2 files and both
are parser-syntax failures:

```text
capturedParametersInInitializers1.ts: UnsupportedSyntax: expected LeftParen, got Some(Star)
capturedParametersInInitializers2.ts: UnsupportedSyntax: expected LeftParen, got Some(LeftBracket)
```

### Smart triage: capturedParametersInInitializers1

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/capturedParametersInInitializers1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedParametersInInitializers1.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(Star) at 654..655",
  "line": 35,
  "column": 2
}
```

Source context:

```text
31 | // ok - used inside immediately invoked generator function
32 | function foo7(y = (function*() {yield z})(), z = 1) {
33 | }
```

TypeScript parses the generator function expression and reports later TS2373
parameter capture diagnostics. Child issue
`issues/open/5213-parse-generator-function-expressions-in-parameter-initializers.md`
owns this parser slice.

### Smart triage: capturedParametersInInitializers2

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/capturedParametersInInitializers2.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedParametersInInitializers2.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(LeftBracket) at 92..93",
  "line": 5,
  "column": 17
}
```

Source context:

```text
3 |     y = class {
4 |         static c = x;
5 |         get [x]() {return x;}
6 |         constructor() { x; }
7 |         [z]() { return z; }
```

TypeScript AST sees `Parameter -> ClassExpression -> GetAccessor ->
ComputedPropertyName` for `[x]`. Child issue
`issues/open/5214-parse-computed-class-members-in-class-expression-initializers.md`
owns this parser slice.

## Completion evidence

capturedParametersInInitializers triage is complete. The two current parser
failures are represented by focused implementation issues 5213 and 5214.

Commits:

- `6a121a10` issues: split captured parameter initializer parser blockers

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/capturedParametersInInitializers --detail --no-dashboard-data
result: pass on the main checkout; 2 executed, 2 UnsupportedSyntax parser-syntax failures recorded
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/capturedParametersInInitializers1.ts
result: pass; recorded expected LeftParen, got Some(Star)
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/capturedParametersInInitializers2.ts
result: pass; recorded expected LeftParen, got Some(LeftBracket)
date: 2026-05-06

command: python scripts/manager.py update-issue-index
result: pass
date: 2026-05-06

command: python scripts/manager.py update-issue-index --check
result: pass
date: 2026-05-06

command: python scripts/manager.py check-issue-health
result: pass
date: 2026-05-06

command: python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
result: pass; child issues 5213 and 5214 are M-sized and ready
date: 2026-05-06

command: git diff --check
result: pass
date: 2026-05-06
```

Remaining risks:

- none
