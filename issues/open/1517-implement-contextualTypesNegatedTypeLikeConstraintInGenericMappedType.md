---
id: 1517
title: "Implement Contextualtypesnegatedtypelikeconstraintingenericmappedtype"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1517.

## Summary

Triage contextualTypesNegatedTypeLikeConstraintInGenericMappedType across 3 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 3 cases fail in directory `contextualTypesNegatedTypeLikeConstraintInGenericMappedType` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualTypesNegatedTypeLikeConstraintInGenericMappedType has 3 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypesNegatedTypeLikeConstraintInGenericMappedType1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypesNegatedTypeLikeConstraintInGenericMappedType1.ts --detail
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
mise run reference-coverage -- tsc --limit 6
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypesNegatedTypeLikeConstraintInGenericMappedType1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypesNegatedTypeLikeConstraintInGenericMappedType1.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5375-support-callable-ambient-interface-local-calls.md`
- [x] created: `issues/open/5376-support-ambient-generic-factory-local-calls.md`
- [x] created: `issues/open/5377-support-callable-ambient-interface-local-calls-with-key-remap.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualTypesNegatedTypeLikeConstraintInGenericMappedType1.ts`
- `reference/typescript/tests/cases/compiler/contextualTypesNegatedTypeLikeConstraintInGenericMappedType3.ts`
- `reference/typescript/tests/cases/compiler/contextualTypesNegatedTypeLikeConstraintInGenericMappedType2.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Date: 2026-05-07

Commands:

```sh
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypesNegatedTypeLikeConstraintInGenericMappedType1.ts
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypesNegatedTypeLikeConstraintInGenericMappedType2.ts
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypesNegatedTypeLikeConstraintInGenericMappedType3.ts
```

Result: split to issues 5375, 5376, and 5377.

Current diagnostics:

```text
contextualTypesNegatedTypeLikeConstraintInGenericMappedType1.ts:
UnsupportedSyntax: issue-211: function-valued local calls such as extracted method `TabGroup(...)` are not supported

contextualTypesNegatedTypeLikeConstraintInGenericMappedType2.ts:
UnsupportedSyntax: issue-211: function-valued local calls such as extracted method `typeTags(...)` are not supported

contextualTypesNegatedTypeLikeConstraintInGenericMappedType3.ts:
UnsupportedSyntax: issue-211: function-valued local calls such as extracted method `TabGroup(...)` are not supported
```

Representative source context:

```ts
declare let TabGroup: _internal_ComponentTabGroup;

TabGroup({
  defaultIndex: 0,
  onChange: (index) => {
    const i: number = index;
  },
});

declare const typeTags: <I>() => <P extends ...>(fields: P) => unknown;
const matcher = typeTags<Value>();
```

Compiler evidence:

- tokens: ok for all three files, including mapped type/key remapping syntax
- ast: ok for all three files
- visible symbols include `DEFAULT_TABS_TAG`/`TabGroup` in files 1 and 3, and
  `typeTags`/`matcher` in file 2
- resolved/lowered: fails at the generic issue-211 function-valued local call
  boundary before contextual/mapped-type behavior is reached
- TypeScript oracle accepts files 1 and 3 with diagnostics `[]`
- TypeScript oracle reaches the intended later TS2322 diagnostic in file 2
  after typing `matcher = typeTags<Value>()`

Duplicate review:

- `issues/open/5375-support-callable-ambient-interface-local-calls.md` owns
  `contextualTypesNegatedTypeLikeConstraintInGenericMappedType1.ts`.
- `issues/open/5376-support-ambient-generic-factory-local-calls.md` owns the
  `declare const typeTags: <I>() => ...; typeTags<Value>()` shape.
- `issues/open/5377-support-callable-ambient-interface-local-calls-with-key-remap.md`
  owns the same `TabGroup(...)` call after mapped-type key remapping in file 3.
- `issues/open/5374-support-callable-ambient-const-local-calls.md` is related
  but remains focused on a single generic ambient const function call used
  directly with runtime arguments.
- `issues/open/5195-support-callable-interface-typed-local-calls.md` is related
  but covers non-ambient callable interface-typed locals and definite
  assignment/type diagnostics.
- Broad type-system buckets such as `issues/open/2497-...` and `issues/open/345-...`
  are less specific than the ambient callable-local boundary.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- split to issues 5375, 5376, and 5377

Validation result:

```text
command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypesNegatedTypeLikeConstraintInGenericMappedType --detail --no-dashboard-data
result: pass; executed=3, unsupported=3, current failures are UnsupportedSyntax type-system
date: 2026-05-07

command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypesNegatedTypeLikeConstraintInGenericMappedType1.ts
result: pass; reproduced issue-211 at ambient callable local `TabGroup(...)`
date: 2026-05-07

command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypesNegatedTypeLikeConstraintInGenericMappedType2.ts
result: pass; reproduced issue-211 at ambient callable local `typeTags<Value>()`
date: 2026-05-07

command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypesNegatedTypeLikeConstraintInGenericMappedType3.ts
result: pass; reproduced issue-211 at ambient callable local `TabGroup(...)`
date: 2026-05-07
```

Remaining risks:

- The reference paths remain unsupported until issues 5375, 5376, and 5377 implement
  or precisely diagnose their callable ambient local call shapes.
