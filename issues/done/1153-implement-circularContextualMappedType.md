---
id: 1153
title: "Implement Circularcontextualmappedtype"
type: spike
area: frontend/resolver
class: done
priority: P1
depends_on: [5226]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
---

## Summary

Triage circularContextualMappedType across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results previously showed 1 case failing in directory `circularContextualMappedType` with diagnostics: name-resolution. Fresh coverage and triage show the current blocker is `DuplicateFunction` for two ambient `declare function reproduce` overload declarations.

Problem: `circularContextualMappedType.ts` fails with `DuplicateFunction` at the second ambient `declare function reproduce` declaration, which is already tracked by issue 5226.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/circularContextualMappedType.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/circularContextualMappedType.ts --detail
```

## Desired final state

This generated bucket is superseded by implementation-ready issue 5226, which owns ambient `declare function` overload declarations. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with existing implementation-ready issue 5226
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the owning issue

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

- [x] Duplicate candidates below are confirmed and this issue is superseded by issue 5226
- [x] This closed issue contains an exact `python scripts/manager.py reference-triage ...` command for this path
- [x] This closed issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Issue 5226 owns the exact ambient overload declaration behavior; this closed issue preserves the path-specific diagnostic/stdout evidence

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/circularContextualMappedType.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/circularContextualMappedType.ts
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

- [x] updated: `issues/done/5226-w0-ast-node-span-requirement.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/circularContextualMappedType.ts`

## Duplicate detection

Fresh duplicate scan found exact owner `issues/done/5226-w0-ast-node-span-requirement.md`.

Other duplicate-function candidates are not the owner:

- `issues/done/5199-report-function-overload-list-class-merge-diagnostics.md` covers overload lists followed by a class of the same name.
- `issues/done/5200-validate-top-level-function-overload-implementations.md` covers non-ambient overload signatures plus an implementation body.
- generated duplicate-function buckets 2043, 2600, and 4258 still need their own smart triage.

## Smart triage

### Smart triage: duplicate function

- Issue class: `triage-needed`
- Feature label: `duplicate-function`
- Diagnostic: `DuplicateFunction` / `compiler-diagnostic`
- Path: `reference/typescript/tests/cases/compiler/circularContextualMappedType.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularContextualMappedType.ts
```

Coverage reproduction:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularContextualMappedType.ts --detail --no-dashboard-data
```

Source context:

```ts
type Func<T> = () => T;

type Mapped<T> = { [K in keyof T]: Func<T[K]> };

declare function reproduce(options: number): void;
declare function reproduce<T>(options: Mapped<T>): T

reproduce({
  name:   () => { return 123 }
});
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=DuplicateFunction:1
unsupported_features=duplicate-function:1
```

Compiler evidence:

```text
tokens: ok
ast: ok; two ambient Function reproduce declarations followed by reproduce calls
visible symbols before failure: function reproduce from the first declaration
validate_ast: DuplicateFunction on the second ambient declaration at 188..197
```

TypeScript oracle evidence:

```text
TypeScript reports ok with no diagnostics.
The first reproduce overload has return type void; the second generic overload has return type T.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularContextualMappedType.ts --detail --no-dashboard-data
result: fail on the main checkout; unsupported=1, unsupported_diagcodes=DuplicateFunction:1, unsupported_features=duplicate-function:1
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularContextualMappedType.ts
result: fail; current blocker is DuplicateFunction for ambient declare overloads, folded into issue 5226
date: 2026-05-06
```

Remaining risks:

- Implementing issue 5226 may expose later mapped-type/contextual typing behavior in this same reference case.
