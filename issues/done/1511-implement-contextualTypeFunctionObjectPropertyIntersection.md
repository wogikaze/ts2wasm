---
id: 1511
title: "Implement Contextualtypefunctionobjectpropertyintersection"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Triage contextualTypeFunctionObjectPropertyIntersection across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualTypeFunctionObjectPropertyIntersection` with diagnostics: object-literal. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualTypeFunctionObjectPropertyIntersection has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypeFunctionObjectPropertyIntersection.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeFunctionObjectPropertyIntersection.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Fold into the existing ambient function overload owner
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

- [x] Duplicate candidates below are confirmed; this issue is superseded by issue 5226
- [x] Existing issue 5226 contains an exact `reference-triage` command for ambient overload declarations
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Issue 5226 acceptance names ambient `declare function` overload declarations

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeFunctionObjectPropertyIntersection.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypeFunctionObjectPropertyIntersection.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] folded into `issues/done/5226-w0-ast-node-span-requirement.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualTypeFunctionObjectPropertyIntersection.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage on 2026-05-07 shows this generated bucket currently stops at the
ambient `declare function` overload boundary already tracked by
`issues/done/5226-w0-ast-node-span-requirement.md`.

Current diagnostic:

```text
DuplicateFunction: duplicate function definition: `createSlice` at 2497..2508
```

Source context:

```ts
declare function createSlice<T>(
  reducers: { [K: string]: (state: string) => void } & {
    [K in keyof T]: object;
  }
): void;

declare function createSlice<
  State,
  CaseReducers extends SliceCaseReducers<State>
>(options: {
  initialState: State | (() => State);
  reducers: ValidateSliceCaseReducers<State, CaseReducers>;
}): void;
```

Focused coverage:

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=DuplicateFunction:1
unsupported_features=duplicate-function:1
```

TypeScript accepts the ambient overload declarations and later reports TS2353
for an object literal property. No child issue is required because issue 5226
already owns duplicate-function validation for multiple bodyless ambient
`declare function` overload declarations.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending

Validation result:

```text
command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeFunctionObjectPropertyIntersection.ts --detail --no-dashboard-data
result: pass; current blocker is DuplicateFunction/duplicate-function on the second ambient `createSlice` overload declaration
date: 2026-05-07

command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypeFunctionObjectPropertyIntersection.ts
result: pass; triage identifies ambient `declare function createSlice` overload declarations owned by issue 5226
date: 2026-05-07
```

Remaining risks:

- The reference path remains duplicate-function unsupported until issue 5226 implements ambient function overload declaration grouping.
