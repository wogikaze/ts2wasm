---
id: 1048
title: "Implement Bigint"
type: spike
area: frontend/resolver
class: superseded
priority: P1
depends_on: [5165]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage bigint across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `bigint` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: bigint has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/bigint64ArraySubarray.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/bigint64ArraySubarray.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/bigint64ArraySubarray.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/bigint64ArraySubarray.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5165-support-typed-array-subarray-builtins.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/bigint64ArraySubarray.ts`

## Duplicate detection

- `issues/done/369-implement-full-multilimb-bigint-arithmetic.md` - Implement full multi-limb BigInt arithmetic (same group key, title overlap)
- `issues/done/370-implement-bigint-arithmetic-exception-parity.md` - Implement BigInt arithmetic RangeError and TypeError parity (same group key, title overlap)
- `issues/done/244-implement-bigint-literal-parser-classification.md` - Implement BigInt literal parser classification (same group key, title overlap)
- `issues/done/259-implement-bigint-literal-runtime-values.md` - Implement BigInt literal runtime values (same group key, title overlap)
- `issues/done/260-implement-bigint-arithmetic-operators.md` - Implement BigInt arithmetic operators (same group key, title overlap)
- `issues/done/261-implement-bigint-equality-comparison-coercion.md` - Implement BigInt equality comparison and coercion boundaries (same group key, title overlap)
- `issues/done/262-implement-bigint-builtins-and-string-conversion.md` - Implement BigInt builtins and string conversion (same group key, title overlap)
- `issues/done/263-implement-bigint-dynamic-mul-div-rem-signed-i64-slice.md` - Implement BigInt dynamic mul/div/rem signed-i64 runtime slice (same group key, title overlap)
- `issues/done/280-implement-dynamic-bigint-builtin-inputs.md` - Implement dynamic BigInt builtin inputs (same group key, title overlap)
- `issues/done/281-implement-bigint-number-edge-equality-comparison.md` - Implement BigInt/Number edge equality and comparison (same group key, title overlap)

## Smart triage

Generated on 2026-05-06.

- Path: `reference/typescript/tests/cases/compiler/bigint64ArraySubarray.ts`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Failure: `method BigInt64Array.subarray not found at 96..110`
- Source context: `arr.subarray();`, `arr.subarray(0);`, `arr.subarray(0, 10);`
- Visible symbol: `arr: BigInt64Array<ArrayBuffer>`
- Compiler stage: tokens and AST succeed; `resolve_builtins` / `lower_program` fails during method-call lowering.
- TypeScript AST path: `FunctionDeclaration -> Block -> ExpressionStatement -> CallExpression -> PropertyAccessExpression`
- Split child: `issues/open/5165-support-typed-array-subarray-builtins.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigint64ArraySubarray.ts
result: pass; current blocker identified as missing `BigInt64Array.subarray` built-in method binding, split to issue 5165
date: 2026-05-06
```

Remaining risks:

- BigInt64Array runtime behavior needs follow-up triage after issue 5165 advances beyond the current method lookup blocker.
