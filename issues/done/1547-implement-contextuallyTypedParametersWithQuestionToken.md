---
id: 1547
title: "Implement Contextuallytypedparameterswithquestiontoken"
type: spike
area: reference/triage
class: triage-needed
priority: P2
depends_on: [5389]
blocks: []
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Triage contextuallyTypedParametersWithQuestionToken across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextuallyTypedParametersWithQuestionToken` with diagnostics: runtime-subset. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextuallyTypedParametersWithQuestionToken has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextuallyTypedParametersWithQuestionToken.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypedParametersWithQuestionToken.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypedParametersWithQuestionToken.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextuallyTypedParametersWithQuestionToken.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5389-support-nested-function-default-parameters-in-closure-lowering.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextuallyTypedParametersWithQuestionToken.ts`

## Duplicate detection

- No exact existing owner found.
- `issues/done/5236-w1-implement-wasi-args-and-environment-variable-lowering.md` is related but
  explicitly excludes nested function default parameters and asks for a
  follow-up after a representative default-parameter closure case is triaged.
- `issues/done/062e-function-closures.md` is the broader historical closure
  work, not an open implementation owner for this default-parameter slice.
- Other runtime-subset duplicate candidates from smart triage were same-label
  generated buckets but not this issue-062e default-parameter closure shape.

## Smart triage

Generated 2026-05-07.

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypedParametersWithQuestionToken.ts

result:
Triage class: contextuallyTypedParametersWithQuestionToken
Feature label: runtime-subset
Diagnostic: UnsupportedRuntimeSubset / unsupported-feature-boundary
Current diagnostic: issue-062e: nested function `self` closure parameters with defaults or rest are not supported in this slice

source context:
function acceptNum(num: number) {}

const f1: (a: string, b: number) => void = function self(a, b?) {
  acceptNum(b);
  self("");
  self("", undefined);
};

const f2: (a: string, b: number) => void = function self(a, b?: number) {
  acceptNum(b);
  self("");
  self("", undefined);
};

visible symbols:
function acceptNum(num: number)
binding f1
function self(a, b?)
binding f2
function self(a, b?: number)

compiler evidence:
tokens: ok; includes optional parameter question token on nested function
ast: ok; optional `b?` is represented as parameter default `Undefined`
resolved/lowered: fails at issue-062e nested function closure parameters with
     defaults or rest
TypeScript oracle: TS2345 for `acceptNum(b)` because `b` is
     `number | undefined`
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- split to `issues/open/5389-support-nested-function-default-parameters-in-closure-lowering.md`

Validation result:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypedParametersWithQuestionToken.ts --detail --no-dashboard-data
result:
pass; executed=1, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=unknown-unsupported:1
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypedParametersWithQuestionToken.ts
result:
pass; reproduced issue-062e nested function default-parameter closure boundary
date:
2026-05-07
```

Remaining risks:

- The reference path remains unsupported until issue 5389 handles nested
  function default/optional parameters in closure lowering or reports the
  earlier TypeScript-style TS2345 diagnostic.
