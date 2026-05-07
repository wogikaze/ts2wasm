---
id: 1155
title: "Implement Circularinferredtypeofvariable"
type: spike
area: frontend/semantics
class: done
priority: P1
depends_on: [5240]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
---

## Summary

Triage circularInferredTypeOfVariable across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `circularInferredTypeOfVariable` with diagnostics: type-system. Fresh triage shows the current blocker is earlier than type inference: the parser rejects the parenthesized async arrow head.

Problem: `circularInferredTypeOfVariable.ts` currently fails with `unsupported expression: ... Async` at `(async () => {`, before AST construction reaches the loop, `await`, or inferred-variable behavior.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/circularInferredTypeOfVariable.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/circularInferredTypeOfVariable.ts --detail
```

## Desired final state

This generated bucket is closed after splitting the current parser blocker into implementation-ready child issue 5240. Do not implement directly from this bucket.

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

- [x] Duplicate candidates below are confirmed as no-match for an implementation-ready owner
- [x] Child issue 5240 contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue 5240 includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue 5240 acceptance names the exact fixture/reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/circularInferredTypeOfVariable.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/circularInferredTypeOfVariable.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5240-w2-docs-audit-and-stale-entries.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/circularInferredTypeOfVariable.ts`

## Duplicate detection

Fresh triage surfaced several related async-arrow parser buckets:

- `issues/open/759-implement-asyncIIFE.md` has the same raw `Async` parser failure for `(async () => { ... })()`.
- `issues/open/751-implement-asyncFunctionNoReturnType.md` has the same raw `Async` parser failure for bare `async () => { ... }`.
- `issues/open/3758-implement-parenthesizedAsyncArrowFunction.md` is a generated bucket for parenthesized async arrows but still needs fresh triage.

No existing implementation-ready issue owned this exact parser boundary, so
issue 5240 was created.

## Smart triage

### Smart triage: type-system label, parser boundary

- Issue class: `triage-needed`
- Feature label: `type-system`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/circularInferredTypeOfVariable.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularInferredTypeOfVariable.ts
```

Coverage reproduction:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularInferredTypeOfVariable.ts --detail --no-dashboard-data
```

Source context:

```ts
(async () => {
    function foo(p: string[]): string[] {
        return [];
    }

    function bar(p: string[]): string[] {
        return [];
    }

    let a1: string[] | undefined = [];

    while (true) {
        let a2 = foo(a1!);
        a1 = await bar(a2);
    }
});
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=type-system:1
```

Compiler evidence:

```text
tokens: ok; LeftParen, Async, LeftParen, RightParen, Arrow, LeftBrace ...
ast: fails before construction with unsupported expression Async at 50..51
resolved: same parser failure
visible symbols before failure: []
```

TypeScript oracle evidence:

```text
TypeScript reports ok with no diagnostics.
Oracle sees the top-level expression as a ParenthesizedExpression containing an ArrowFunction.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularInferredTypeOfVariable.ts --detail --no-dashboard-data
result: fail on the main checkout; unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=type-system:1
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularInferredTypeOfVariable.ts
result: fail; current blocker is raw Async parser failure, split into issue 5240
date: 2026-05-06
```

Remaining risks:

- Fixing issue 5240 may expose later async/await runtime or circular inferred-variable semantics.
