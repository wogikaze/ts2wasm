---
id: 1536
title: "Implement Contextuallytypeasyncfunctionreturntypefromunion"
type: spike
area: runtime/builtins
class: triage-needed
priority: P1
depends_on: [5240]
blocks: []
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Triage contextuallyTypeAsyncFunctionReturnTypeFromUnion across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextuallyTypeAsyncFunctionReturnTypeFromUnion` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextuallyTypeAsyncFunctionReturnTypeFromUnion has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextuallyTypeAsyncFunctionReturnTypeFromUnion.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypeAsyncFunctionReturnTypeFromUnion.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypeAsyncFunctionReturnTypeFromUnion.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextuallyTypeAsyncFunctionReturnTypeFromUnion.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/done/5240-w2-docs-audit-and-stale-entries.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextuallyTypeAsyncFunctionReturnTypeFromUnion.ts`

## Duplicate detection

- Superseded by `issues/done/5240-w2-docs-audit-and-stale-entries.md`.
  The current first blocker is the same raw async-arrow parser boundary, here
  in object property value `test: async () => Promise.reject(...)`.

## Smart triage

Generated 2026-05-07.

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypeAsyncFunctionReturnTypeFromUnion.ts

result:
Feature label: unknown-unsupported
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
error: unsupported expression: Some(SpannedToken { kind: Async, span: Span { start: 319, end: 324 } }) at 325..326

source context:
createMachine<{ count: number }>({
  services: {
    test: async () => Promise.reject("some err"),
    async test2() {
      return Promise.reject("some err");
    },

compiler evidence:
tokens: ok through `async () => Promise.reject("some err")`
ast: fails before async arrow construction
visible symbols: class `StateMachine`
TypeScript AST: PropertyAssignment `test: async () => Promise.reject("some err")` with ArrowFunction initializer
TypeScript oracle: ok, diagnostics []
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- superseded by `issues/done/5240-w2-docs-audit-and-stale-entries.md`

Validation result:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypeAsyncFunctionReturnTypeFromUnion.ts --detail --no-dashboard-data
result:
pass; executed=1, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=unknown-unsupported:1
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypeAsyncFunctionReturnTypeFromUnion.ts
result:
pass; reproduced raw async-arrow parser failure at object property value `async () => ...`
date:
2026-05-07
```

Remaining risks:

- The reference path remains unsupported until issue 5240 parses async arrow
  function expressions.
