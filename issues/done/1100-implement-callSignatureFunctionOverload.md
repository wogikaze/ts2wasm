---
id: 1100
title: "Implement Callsignaturefunctionoverload"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5201]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Triage callSignatureFunctionOverload across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `callSignatureFunctionOverload` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: callSignatureFunctionOverload has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callSignatureFunctionOverload.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/callSignatureFunctionOverload.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/callSignatureFunctionOverload.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callSignatureFunctionOverload.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5201-parse-object-type-literal-call-signatures.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/callSignatureFunctionOverload.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: callSignatureFunctionOverload

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedTypeScriptSyntax` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/callSignatureFunctionOverload.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callSignatureFunctionOverload.ts
```

Failure:

```text
error: [UnsupportedTypeScriptSyntax] unterminated TypeScript type annotation at 288..289
```

Source context:

```ts
var foo: {
    (name: string): string;
    (name: 'order'): string;
    (name: 'content'): string;
    (name: 'done'): string;
}
```

Evidence:

- Tokens succeed and include the object type literal braces plus
  call-signature member tokens.
- AST construction fails with no module AST.
- Visible symbols include bindings `foo` and `foo2`.
- TypeScript oracle reports no diagnostics and hints both variables as object
  types with multiple call signatures.
- Broad parser-syntax candidates are no-match owners because the exact blocker
  is object type literal call-signature syntax.
- Child issue `issues/open/5201-parse-object-type-literal-call-signatures.md`
  owns the implementation slice.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...` pending

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/callSignatureFunctionOverload.ts
result: pass; reproduced unterminated TypeScript type annotation for object type literal call signatures
date: 2026-05-06
```

Remaining risks:

- none
