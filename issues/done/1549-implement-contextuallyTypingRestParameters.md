---
id: 1549
title: "Implement Contextuallytypingrestparameters"
type: spike
area: reference/triage
class: triage-needed
priority: P2
depends_on: [5236]
blocks: []
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Triage contextuallyTypingRestParameters across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextuallyTypingRestParameters` with diagnostics: runtime-subset. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextuallyTypingRestParameters has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextuallyTypingRestParameters.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypingRestParameters.ts --detail
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

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypingRestParameters.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextuallyTypingRestParameters.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] folded into: `issues/done/5236-w1-implement-wasi-args-and-environment-variable-lowering.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextuallyTypingRestParameters.ts`

## Duplicate detection

- Superseded by
  `issues/done/5236-w1-implement-wasi-args-and-environment-variable-lowering.md`, which owns
  the same issue-062e closure guard for function expressions with rest
  parameters.
- `issues/open/5389-support-nested-function-default-parameters-in-closure-lowering.md`
  is related but owns optional/default parameters, not rest parameters.
- Other runtime-subset duplicate candidates from smart triage were same-label
  generated buckets but not this issue-062e rest-parameter closure shape.

## Smart triage

Generated 2026-05-07.

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypingRestParameters.ts

result:
Triage class: contextuallyTypingRestParameters
Feature label: runtime-subset
Diagnostic: UnsupportedRuntimeSubset / unsupported-feature-boundary
Current diagnostic: issue-062e: nested function `` closure parameters with defaults or rest are not supported in this slice

source context:
var x: (...y: string[]) => void = function (.../*3*/y) {
    var t = y;
    var x2: string = t; // This should be error
    var x3: string[] = t; // No error
    var y2: string = y; // This should be error
    var y3: string[] = y; // No error
};

visible symbols:
binding x
binding t = y
binding x2
binding x3
binding y2
binding y3

compiler evidence:
tokens: ok; includes contextual function type `(...y: string[]) => void` and
     function expression rest parameter `...y`
ast: ok; FunctionExpr params include `y` with is_rest=true
resolved/lowered: issue-062e nested function closure parameters with defaults
     or rest
TypeScript oracle: TS2322 for assigning `string[]` to `string` at x2 and y2
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- folded into `issues/done/5236-w1-implement-wasi-args-and-environment-variable-lowering.md`

Validation result:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypingRestParameters.ts --detail --no-dashboard-data
result:
pass; executed=1, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=unknown-unsupported:1
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypingRestParameters.ts
result:
pass; reproduced issue-062e rest-parameter closure boundary
date:
2026-05-07
```

Remaining risks:

- The reference path remains unsupported until issue 5236 handles rest
  parameters in function-expression closure lowering or reports the earlier
  TypeScript-style TS2322 diagnostics.
