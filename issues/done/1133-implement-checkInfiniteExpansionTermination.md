---
id: 1133
title: "Implement Checkinfiniteexpansiontermination"
type: spike
area: frontend/resolver
class: done
priority: P1
depends_on: [5226]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage checkInfiniteExpansionTermination across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `checkInfiniteExpansionTermination`. Fresh triage shows tokens and AST succeed; `validate_ast` stops on duplicate-function validation for two ambient `declare function combineLatest` overload declarations.

Problem: `checkInfiniteExpansionTermination2.ts` is too broad for direct implementation. Its current observable blocker is now tracked by `issues/open/5226-allow-ambient-function-overload-declarations.md`.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkInfiniteExpansionTermination2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkInfiniteExpansionTermination2.ts --detail
```

## Desired final state

This generated bucket is superseded by `issues/open/5226-allow-ambient-function-overload-declarations.md`. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split the current observable blocker into issue 5226
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this issue and issue 5226

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
- [x] Issue 5226 contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Issue 5226 includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Issue 5226 acceptance names the exact fixture/reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkInfiniteExpansionTermination2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkInfiniteExpansionTermination2.ts
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

- [x] created: `issues/open/5226-allow-ambient-function-overload-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/checkInfiniteExpansionTermination2.ts`

## Duplicate detection

- `issues/open/5200-validate-top-level-function-overload-implementations.md` is related but not exact: it handles non-ambient overload signatures with implementation declarations, while this bucket's current blocker is multiple ambient `declare function` signatures with no implementation body.
- `issues/open/5199-report-function-overload-list-class-merge-diagnostics.md` is not exact: this bucket has no class/function merge.

## Smart triage

### Smart triage: Triage duplicate function: checkInfiniteExpansionTermination2

- Issue class: `triage-needed`
- Feature label: `duplicate-function`
- Diagnostic: `DuplicateFunction` / `compiler-diagnostic`
- Path: `reference/typescript/tests/cases/compiler/checkInfiniteExpansionTermination2.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkInfiniteExpansionTermination2.ts
```

Coverage reproduction:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkInfiniteExpansionTermination2.ts --detail --no-dashboard-data
```

Source context:

```text
interface IObservable<T> {
    n: IObservable<T[]>;
}
interface ISubject<T> extends IObservable<T> { }

declare function combineLatest<TOther>(x: IObservable<TOther>[]): void;
declare function combineLatest(): void;

function fn<T>() {
    var values: ISubject<any>[] = [];
    combineLatest<T>(values);
}
```

Current compiler failure:

```text
error: [DuplicateFunction] duplicate function definition: `combineLatest` at 321..334
```

Compiler evidence:

- Tokens succeed for interface declarations, ambient function declarations, and `combineLatest<T>(values)`.
- AST succeeds with two ambient `Function combineLatest` declarations followed by `Function fn`.
- `validate_ast` rejects the second ambient declaration as a duplicate concrete function.

TypeScript oracle evidence:

```text
TypeScript reports no diagnostics for the ambient overload declarations.
```

Resolution:

```text
The current blocker is now tracked by child issue 5226. It is narrower than the generated bucket: allow multiple ambient `declare function` overload signatures for the same name.
```

## Completion evidence

Fill only when moving to `done/`.

checkInfiniteExpansionTermination2 triage is complete. The actionable blocker
is tracked by child issue 5226.

Commits:

- child issue: `issues/open/5226-allow-ambient-function-overload-declarations.md`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkInfiniteExpansionTermination2.ts --detail --no-dashboard-data
result: pass on the main checkout; 1 executed, current failure is DuplicateFunction duplicate-function
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkInfiniteExpansionTermination2.ts
result: pass; reproduced DuplicateFunction for ambient combineLatest overload declarations and split to issue 5226
date: 2026-05-06
```

Remaining risks:

- none
