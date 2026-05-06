---
id: 1423
title: "Implement Conditionaltypebasedcontextualtypereturntypewidening"
type: spike
area: frontend/semantics
class: done
priority: P1
depends_on: [5002]
blocks: [5273]
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---

## Summary

Triage conditionalTypeBasedContextualTypeReturnTypeWidening across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case in directory
`conditionalTypeBasedContextualTypeReturnTypeWidening`. Fresh triage on
2026-05-07 shows the current first blocker is not conditional type semantics;
AST construction stops on the nested zero-argument arrow expression
`() => () => 0`, which is already owned by issue 5273.

Problem: conditionalTypeBasedContextualTypeReturnTypeWidening has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conditionalTypeBasedContextualTypeReturnTypeWidening.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conditionalTypeBasedContextualTypeReturnTypeWidening.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with existing issue 5273
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

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

- [x] Duplicate candidates below are confirmed; issue 5273 covers this bucket
- [x] Issue 5273 already contains exact `reference-triage` commands for the same nested zero-argument arrow parser boundary
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Superseding issue 5273 acceptance names the exact `RightParen` parser diagnostic change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conditionalTypeBasedContextualTypeReturnTypeWidening.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conditionalTypeBasedContextualTypeReturnTypeWidening.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by issue 5273

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/conditionalTypeBasedContextualTypeReturnTypeWidening.ts`

## Duplicate detection

- Issue 5273 `Parse nested zero-argument arrow returns` is an exact
  implementation owner for the current `unsupported expression:
  ... RightParen` boundary. It already covers `() => ...` used as the
  expression body of another arrow.
- Other type-system duplicate candidates from smart triage are broad generated
  buckets and do not match this parser boundary.

## Smart triage

Generated on 2026-05-07:

```text
Feature label: type-system
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Path: reference/typescript/tests/cases/compiler/conditionalTypeBasedContextualTypeReturnTypeWidening.ts
Failure: unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 286, end: 287 } }) at 288..290
line 5, column 38
Source context:
2 | declare function useState1<S>(initialState: (S extends (() => any) ? never : S) | (() => S)): S; // No args
3 | declare function useState2<S>(initialState: (S extends ((...args: any[]) => any) ? never : S) | (() => S)): S; // Any args
4 |
5 | const func1 = useState1(() => () => 0);
Visible symbols:
- binding func1
```

Compiler evidence:

```text
tokens: ok; useState1 LeftParen LeftParen RightParen Arrow LeftParen RightParen Arrow Number(0)
ast: fails before resolved IR with unsupported expression at the nested zero-argument arrow
TypeScript oracle: ok, diagnostics=[]; binding func1 type is () => 0
```

Coverage result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/conditionalTypeBasedContextualTypeReturnTypeWidening.ts --detail --no-dashboard-data
result: executed=1, build_pass=0, unsupported=1, blocked=0, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=type-system:1
date: 2026-05-07
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Superseded by `issues/open/5273-parse-nested-zero-argument-arrow-returns.md`.

Validation result:

```text
command: python scripts/manager.py update-issue-index && python scripts/manager.py update-issue-index --check && python scripts/manager.py check-issue-health && python scripts/manager.py check-issue-readiness -- --fail-ready-below 80 && git diff --check
result: pass
date: 2026-05-07
```

Remaining risks:

- Issue 5273 still needs implementation. After 5273 lands, this reference may expose conditional type contextual return widening work.
