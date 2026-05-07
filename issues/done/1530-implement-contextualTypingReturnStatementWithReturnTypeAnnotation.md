---
id: 1530
title: "Implement Contextualtypingreturnstatementwithreturntypeannotation"
type: spike
area: frontend/resolver
class: blocked
priority: P1
depends_on: [5384]
blocks: []
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Triage contextualTypingReturnStatementWithReturnTypeAnnotation across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualTypingReturnStatementWithReturnTypeAnnotation` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualTypingReturnStatementWithReturnTypeAnnotation has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypingReturnStatementWithReturnTypeAnnotation.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingReturnStatementWithReturnTypeAnnotation.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingReturnStatementWithReturnTypeAnnotation.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypingReturnStatementWithReturnTypeAnnotation.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5384-resolve-ambient-function-value-references.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualTypingReturnStatementWithReturnTypeAnnotation.ts`

## Duplicate detection

- No exact existing owner found.
- `issues/open/064-implement-name-resolution.md` is a superseded test262
  metadata bucket, not this TypeScript ambient function value-reference gap.
- `issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`
  is related but owns ambient value declarations such as `declare const`, not
  ambient `declare function` identifiers passed as callback values.

## Smart triage

Generated 2026-05-07.

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingReturnStatementWithReturnTypeAnnotation.ts

result:
Feature label: name-resolution
Diagnostic: UnresolvedName / resolver-symbol
error: [UnresolvedName] unresolved name: `isString`

source context:
declare function isString(text: unknown): text is string;

declare function getPropFromRaw<T>(
  prop: "files" | "include" | "exclude" | "references",
  validateElement: (value: unknown) => boolean,
  elementTypeName: string
): PropOfRaw<T>;

function getSpecsFromRaw(
  prop: "files" | "include" | "exclude"
): PropOfRaw<string> {
  return getPropFromRaw(prop, isString, "string");
}

compiler evidence:
tokens: ok
ast: ok; ambient functions `isString` and `getPropFromRaw`, then return call with `Ident isString`
visible symbols: function `isString`, function `getSpecsFromRaw`
TypeScript oracle: ok, diagnostics []
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- split to `issues/open/5384-resolve-ambient-function-value-references.md`

Validation result:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingReturnStatementWithReturnTypeAnnotation.ts --detail --no-dashboard-data
result:
pass; executed=1, unsupported=1, unsupported_diagcodes=UnresolvedName:1, unsupported_features=name-resolution:1
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingReturnStatementWithReturnTypeAnnotation.ts
result:
pass; reproduced UnresolvedName for ambient declared function value `isString`
date:
2026-05-07
```

Remaining risks:

- The reference path remains unsupported until issue 5384 resolves ambient
  declared function value references.
