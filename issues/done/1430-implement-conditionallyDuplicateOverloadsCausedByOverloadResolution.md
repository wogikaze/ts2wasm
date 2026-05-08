---
id: 1430
title: "Implement Conditionallyduplicateoverloadscausedbyoverloadresolution"
type: spike
area: frontend/syntax
class: done
priority: P2
depends_on: [5005]
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1430.

## Summary

Triage conditionallyDuplicateOverloadsCausedByOverloadResolution across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results previously showed 1 case failing in directory
`conditionallyDuplicateOverloadsCausedByOverloadResolution` with diagnostics:
module-resolution. Fresh triage on 2026-05-07 shows the current first blocker
is duplicate-function validation for ambient `declare function` overload
declarations, already tracked by issue 5226.

Problem: conditionallyDuplicateOverloadsCausedByOverloadResolution has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conditionallyDuplicateOverloadsCausedByOverloadResolution.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conditionallyDuplicateOverloadsCausedByOverloadResolution.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with existing issue 5226
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

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Superseding issue 5226 contains exact ambient `declare function` overload acceptance
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Coverage names the exact reference path and current stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conditionallyDuplicateOverloadsCausedByOverloadResolution.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conditionallyDuplicateOverloadsCausedByOverloadResolution.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; superseded by `issues/done/5226-w0-ast-node-span-requirement.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/conditionallyDuplicateOverloadsCausedByOverloadResolution.ts`

## Duplicate detection

- Superseded by `issues/done/5226-w0-ast-node-span-requirement.md`.
  The current first blocker is the same ambient bodyless
  `declare function` overload declaration shape that 5226 owns.
- `issues/open/5200-validate-top-level-function-overload-implementations.md`
  is related but not exact because it covers non-ambient overload signatures
  plus implementation declarations.
- `issues/done/5280-validate-commented-top-level-function-overloads.md` and
  `issues/open/5289-validate-comments-overloads-top-level-functions.md` are
  comment/trivia-specific non-ambient overload slices.

## Smart triage

Generated on 2026-05-07:

```text
Feature label: duplicate-function
Diagnostic: DuplicateFunction / compiler-diagnostic
Path: reference/typescript/tests/cases/compiler/conditionallyDuplicateOverloadsCausedByOverloadResolution.ts
Failure: duplicate function definition: `foo` at 104..107
line: 3, column: 18
Visible symbols before failure:
- function foo at line 2
```

Source context:

```text
2 | declare function foo(func: (x: string, y: string) => any): boolean;
3 | declare function foo(func: (x: string, y: number) => any): string;
4 |
5 | var out = foo((x, y) => {
6 |     function bar(a: typeof x): void;
```

Compiler evidence:

```text
tokens: ok
ast: ok; contains two bodyless Function foo declarations, then var out = foo(...)
validate_ast/resolved: DuplicateFunction on the second ambient declaration
```

TypeScript oracle evidence:

```text
TypeScript does not reject the ambient foo/foo2 overload declarations.
The only oracle diagnostic is TS2454: Variable `bar` is used before being assigned.
Oracle hints include foo:boolean and foo:string overload signatures with function-typed parameter func.
```

Coverage result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/conditionallyDuplicateOverloadsCausedByOverloadResolution.ts --detail --no-dashboard-data
result: executed=1, unsupported=1, unsupported_diagcodes=DuplicateFunction:1, unsupported_features=duplicate-function:1, build_pass=0, semantic_pass=0, blocked=0
date: 2026-05-07
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Closed as superseded by issue 5226; no child issue created.

Validation result:

```text
command: python scripts/manager.py update-issue-index && python scripts/manager.py update-issue-index --check && python scripts/manager.py check-issue-health && python scripts/manager.py check-issue-readiness -- --fail-ready-below 80 && git diff --check
result: pass
date: 2026-05-07
```

Remaining risks:

- none
