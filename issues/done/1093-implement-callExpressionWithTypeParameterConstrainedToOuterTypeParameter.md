---
id: 1093
title: "Implement Callexpressionwithtypeparameterconstrainedtooutertypeparameter"
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

Triage callExpressionWithTypeParameterConstrainedToOuterTypeParameter across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `callExpressionWithTypeParameterConstrainedToOuterTypeParameter` with diagnostics: method-call. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: callExpressionWithTypeParameterConstrainedToOuterTypeParameter has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callExpressionWithTypeParameterConstrainedToOuterTypeParameter.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/callExpressionWithTypeParameterConstrainedToOuterTypeParameter.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/callExpressionWithTypeParameterConstrainedToOuterTypeParameter.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callExpressionWithTypeParameterConstrainedToOuterTypeParameter.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5195-support-callable-interface-typed-local-calls.md`

## Notes

Superseded by `issues/open/5195-support-callable-interface-typed-local-calls.md`.
Fresh triage shows the source parses and resolves; the current blocker is the
`issue-211` function-valued local call diagnostic for `i("")`.

## Affected test files

- `reference/typescript/tests/cases/compiler/callExpressionWithTypeParameterConstrainedToOuterTypeParameter.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated on 2026-05-06:

- command: `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/callExpressionWithTypeParameterConstrainedToOuterTypeParameter.ts`
- diagnostic: `UnsupportedSyntax`, `issue-211: function-valued local calls such as extracted method i(...) are not supported`
- AST: `Let i = Undefined`; `Let y = Call(Ident i, String "")`
- visible symbols: local bindings `i` and `y`
- TypeScript oracle: TS2454 `Variable 'i' is used before being assigned`
- follow-up: `issues/open/5195-support-callable-interface-typed-local-calls.md`

## Completion evidence

Closed as a generated triage bucket. The actionable callable-local semantic
gap is tracked by `issues/open/5195-support-callable-interface-typed-local-calls.md`.

Commits:

- this split commit

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/callExpressionWithTypeParameterConstrainedToOuterTypeParameter.ts
result: fail with issue-211 function-valued local call diagnostic after parse/name-resolution
date: 2026-05-06
```

Remaining risks:

- Follow-up issue 5195 still needs implementation.
