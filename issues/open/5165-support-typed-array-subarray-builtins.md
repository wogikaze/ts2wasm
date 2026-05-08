---
id: 5165
title: "Support typed array subarray builtins"
type: feature
area: ir/builtin-resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Typed array `subarray` calls parse successfully, but `resolve_builtins`/IR lowering treats typed array instances as user classes and reports a `<TypedArray>.subarray` method-not-found diagnostic.

## Problem

The current failures are method binding/runtime lowering gaps, not parser gaps:

- `reference/typescript/tests/cases/compiler/typedArraysSubarray.ts`: first failure is `method Int8Array.subarray not found`.
- `reference/typescript/tests/cases/compiler/bigint64ArraySubarray.ts`: first failure is `method BigInt64Array.subarray not found`.

Both references construct a typed array and call:

```ts
arr.subarray();
arr.subarray(0);
arr.subarray(0, 10);
```

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/typedArraysSubarray.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigint64ArraySubarray.ts
```

Current diagnostics:

```text
UnsupportedSyntax: method `Int8Array.subarray` not found at 105..119
UnsupportedSyntax: method `BigInt64Array.subarray` not found at 96..110
```

Compiler evidence:

- Tokens and AST are successful for both representative cases.
- The pipeline reaches `resolve_builtins` and `lower_program`.
- The failure is emitted from method-call resolution after `arr` is associated with a typed-array constructor class name.

TypeScript oracle evidence:

```text
typedArraysSubarray.ts: no diagnostics
arr: Int8Array<ArrayBuffer>
bigint64ArraySubarray.ts: no diagnostics
arr: BigInt64Array<ArrayBuffer>
```

TypeScript AST path at the failing call:

```text
FunctionDeclaration -> Block -> ExpressionStatement -> CallExpression -> PropertyAccessExpression
```

## Desired final state

Typed array `subarray` calls for the supported typed-array constructors are recognized by the built-in/method-call resolver and lower to a runtime path or a later source-spanned typed-array runtime diagnostic. The representative cases should no longer fail with a `<TypedArray>.subarray` method-not-found diagnostic.

## Scope

In scope:

- [x] Recognize `subarray` on `Int8Array`, `Uint8Array`, `Uint8ClampedArray`, `Int16Array`, `Uint16Array`, `Int32Array`, `Uint32Array`, `Float32Array`, `Float64Array`, and `BigInt64Array`.
- [x] Preserve ordinary user-class method lookup for non-typed-array classes.
- [x] Cover the zero-, one-, and two-argument forms used by the TypeScript reference tests.
- [x] Add focused IR/builtin resolver coverage for typed-array `subarray` method binding.
- [x] Re-run representative triage and confirm the current method-not-found blocker is gone.

Out of scope:

- Full TypedArray memory model beyond what `subarray` needs to advance these references.
- BigUint64Array coverage unless a reference test or follow-up triage exposes it.
- General library method-call support unrelated to typed arrays.

## Affected paths

Expected:

- `crates/ir/src/lowered/program_builtins.rs`
- `crates/ir/src/lowered/resolver_expr.rs`
- `crates/ir/src/builtin_resolver.rs`
- `crates/backend-wasm/src/runtime_collections.rs`
- `crates/backend-wasm/src/lib.rs`
- `crates/cli/tests/`

Do not touch:

- parser syntax code unless triage regresses before AST construction.
- unrelated `Array.prototype` or `String.prototype` method-call lowering.

## Acceptance criteria

- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/typedArraysSubarray.ts` no longer reports `method Int8Array.subarray not found`.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigint64ArraySubarray.ts` no longer reports `method BigInt64Array.subarray not found`.
- [x] Focused IR/builtin resolver tests cover `arr.subarray()`, `arr.subarray(0)`, and `arr.subarray(0, 10)`.
- [x] Any downstream unsupported typed-array runtime behavior is source-spanned and tracked separately instead of hidden behind method lookup.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir
cargo nextest run -p ts2wasm-backend-wasm
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/typedArraysSubarray.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigint64ArraySubarray.ts
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from generated buckets `1048` and `4593` on 2026-05-06 after both smart-triage runs reached `resolve_builtins`/`lower_program` and failed on missing typed-array `subarray` method binding.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none


## False-done audit

Date: 2026-05-07

Classification: truly-done.

Audit result: retained in issues/done/. Implementation commits confirmed.
