---
id: 1515
title: "Implement Contextualtypeselfreferencing"
type: spike
area: frontend/resolver
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Triage contextualTypeSelfReferencing across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualTypeSelfReferencing` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualTypeSelfReferencing has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypeSelfReferencing.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeSelfReferencing.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeSelfReferencing.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypeSelfReferencing.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5374-support-callable-ambient-const-local-calls.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualTypeSelfReferencing.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Date: 2026-05-07

Command:

```sh
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypeSelfReferencing.ts
```

Result: split to `issues/open/5374-support-callable-ambient-const-local-calls.md`.

Current diagnostic:

```text
UnsupportedSyntax: issue-211: function-valued local calls such as extracted method `parse(...)` are not supported; call receiver.method(...) directly at 339..360
feature_label: method-call
line 17, column 16
```

Source context:

```ts
declare const parse: <def>(def: narrow<def>) => def;

const result = parse([{ a: "foo" }]);
```

Compiler evidence:

- tokens: ok
- ast: ok; representative AST includes `AmbientValueDecl parse` and `Let result = Call(Ident parse, Array[Object { a: "foo" }])`
- visible symbols include the ambient binding `parse` and local binding `result`
- resolved/lowered: fails at the generic issue-211 function-valued local call boundary
- TypeScript oracle: ok, diagnostics `[]`

Duplicate review:

- `issues/open/5195-support-callable-interface-typed-local-calls.md` is related
  but covers callable interface-typed locals such as `var i: I<string>; i("")`.
- `issues/open/5196-support-callable-conditional-typed-parameter-calls.md` is
  related but covers callable conditional-typed parameters such as `arg(10)`.
- `issues/open/5279-report-function-typed-local-call-definite-assignment.md`
  covers uninitialized `var f: () => any; f()` definite-assignment
  diagnostics, not ambient declared callable const calls.
- No exact existing owner was found for ambient `declare const` callable local
  calls with generic call signatures, so issue 5374 owns this fixed reference
  window.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- split to `issues/open/5374-support-callable-ambient-const-local-calls.md`

Validation result:

```text
command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeSelfReferencing.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, current failure is UnsupportedSyntax unknown-unsupported
date: 2026-05-07

command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypeSelfReferencing.ts
result: pass; reproduced issue-211 at ambient callable local `parse(...)` and split to issue 5374
date: 2026-05-07
```

Remaining risks:

- The reference path remains unsupported until issue 5374 implements or
  precisely diagnoses ambient callable local calls.
