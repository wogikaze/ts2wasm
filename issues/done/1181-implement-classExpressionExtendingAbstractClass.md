---
id: 1181
title: "Implement Classexpressionextendingabstractclass"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Triage classExpressionExtendingAbstractClass across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `classExpressionExtendingAbstractClass` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: classExpressionExtendingAbstractClass has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExpressionExtendingAbstractClass.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionExtendingAbstractClass.ts --detail
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
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command or the bucket is closed as stale build-pass evidence
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence or no current compiler blocker remains
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change or no child issue is needed

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionExtendingAbstractClass.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExpressionExtendingAbstractClass.ts
```

Not run:

- cargo gates; issue close only, no implementation changes

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classExpressionExtendingAbstractClass.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage on 2026-05-06:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionExtendingAbstractClass.ts
```

Result:

```text
Smart triage class: none
Feature label: build-pass
Diagnostic: BuildPass / pass
Current result: ts2wasm build succeeded
```

Compiler dump evidence:

```text
tokens: ok
ast: ok; abstract class A plus ClassDecl C extends A
resolved: ok; ClassDecl C extends Some("A")
```

TypeScript oracle:

```text
TS2653: Non-abstract class expression does not implement inherited abstract member 'foo' from class 'A'.
```

The original generated `parser-syntax` blocker is stale. The reference case now
builds under the current compiler, so this bucket is closed without creating a
child issue. The remaining TS2653 diagnostic is semantic parity/type-checking
work, not a current build blocker in this coverage path.

## Completion evidence

Commits:

- local stale-close commit for issue 1181

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionExtendingAbstractClass.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionExtendingAbstractClass.ts
result: pass; BuildPass, tokens/ast/resolved ok
date: 2026-05-06
```

Remaining risks:

- TypeScript oracle reports TS2653 for abstract-member implementation, but semantic diagnostics are not enabled in this coverage path.
