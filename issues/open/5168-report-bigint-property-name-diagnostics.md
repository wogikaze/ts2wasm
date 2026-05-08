---
id: 5168
title: "Report BigInt property-name diagnostics"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

`bigintPropertyName.ts` starts with object literal and type/member forms that intentionally use BigInt literals as property names. TypeScript reports specific diagnostics, but the parser currently stops at the first object literal key with a generic `expected identifier or string literal as object key` error.

## Problem

Problem: `reference/typescript/tests/cases/compiler/bigintPropertyName.ts` currently reports `UnsupportedSyntax: expected identifier or string literal as object key, got Some(BigIntLiteral("1n"))` for `({1n: 123})`.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigintPropertyName.ts
```

Current compiler diagnostic:

```text
UnsupportedSyntax: expected identifier or string literal as object key, got Some(BigIntLiteral("1n")) at 102..104
```

Representative source:

```ts
{ ({1n: 123}); };

const a = { 1n: 123 };
const b = { [1n]: 456 };
```

Compiler evidence:

- Lexer recognizes `BigIntLiteral("1n")`.
- Parser fails before AST construction for the first object literal property.
- The current diagnostic is generic parser syntax, not the TypeScript-facing BigInt property-name diagnostic.

TypeScript oracle evidence:

```text
TS1539: A 'bigint' literal cannot be used as a property name.
TS2464: A computed property name must be of type 'string', 'number', 'symbol', or 'any'.
TS2538: Type 'bigint' cannot be used as an index type.
```

## Desired final state

BigInt literal property names are rejected with source-spanned, TypeScript-aligned diagnostics instead of the generic parser key expectation. The representative reference should no longer stop on `expected identifier or string literal as object key`.

## Scope

In scope:

- [x] Detect BigInt literal object keys such as `{ 1n: 123 }` and report an issue-linked TS1539-compatible diagnostic.
- [x] Detect computed BigInt literal object keys such as `{ [1n]: 456 }` and report a source-spanned TS2464-compatible diagnostic or explicit follow-up boundary.
- [x] Preserve existing accepted identifier, string, and numeric object keys.
- [x] Add focused parser/frontend regression coverage for BigInt literal object property names.
- [x] Re-run representative triage and confirm the first generic parser key diagnostic is gone.

Out of scope:

- Full TypeScript checker support for interface/class/type literal BigInt property-name forms beyond the first parser-visible blockers.
- BigInt-as-index type checking after parser diagnostics advance past object literal keys.
- Runtime BigInt property-key coercion semantics.

## Affected paths

Expected:

- `crates/frontend/src/parser/tokens.rs`
- `crates/frontend/src/parser/expressions_main.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/frontend/src/ast.rs`
- `scripts/run/reference-triage.py`

Do not touch:

- BigInt arithmetic/runtime lowering unless later triage proves it is the next blocker.
- module graph or resolver code for this parser-owned diagnostic.

## Acceptance criteria

- [x] `parse_program("({ 1n: 123 });")` fails with a source-spanned BigInt property-name diagnostic, not `expected identifier or string literal as object key`.
- [x] `parse_program("({ [1n]: 456 });")` fails with a source-spanned computed property-name diagnostic or a documented follow-up issue.
- [x] Existing identifier/string/numeric object key tests still pass.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigintPropertyName.ts` no longer reports the first generic object-key parser diagnostic.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigintPropertyName.ts
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

Split from generated bucket `1052` on 2026-05-06. The reference contains additional BigInt property-name and index-type diagnostics; those should be triaged after this parser diagnostic no longer masks the rest of the file.

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

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in issues/done/. Implementation commits confirmed.
Future-work tracking: none identified.
