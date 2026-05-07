---
id: 5301
title: "Report literal reference comparison diagnostics"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Report TypeScript-compatible diagnostics for equality comparisons between object
or array literals and freshly-created reference values, starting with the first
condition in `conditionalEqualityOnLiteralObjects.ts`:

```ts
if ({ a: 1 } === { a: 1 }) {
}
```

## Problem

Problem: `conditionalEqualityOnLiteralObjects.ts` builds successfully even
though TypeScript reports TS2839 because the compared object/array literals are
distinct references and the condition is statically always true or false.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/conditionalEqualityOnLiteralObjects.ts
```

Current compiler result:

```text
BuildPass: ts2wasm build succeeded
```

Representative source:

```ts
const a = { a: 1 }
const b = [1]

if ({ a: 1 } === { a: 1 }) {
}
```

Compiler evidence:

- Tokens, AST, and resolved IR succeed.
- Resolved IR preserves `Binary(Object, StrictEqual, Object)` and the related
  array/local-vs-literal equality conditions.
- No compiler diagnostic is emitted for reference comparison conditions.

TypeScript oracle evidence:

```text
TS2839: This condition will always return 'false' since JavaScript compares objects by reference, not value.
```

## Desired final state

The compiler reports a source-spanned diagnostic for the representative
object-literal strict equality condition instead of returning `BuildPass`.

## Scope

In scope:

- [ ] Detect the direct object-literal `===` object-literal condition shape.
- [ ] Report a source-spanned TS2839-compatible diagnostic at the condition.
- [ ] Add focused coverage for the representative strict-equality fixture.

Out of scope:

- Full control-flow unreachable-code analysis.
- Array literal comparisons and abstract equality variants after the first object-literal slice.
- Runtime equality semantics for non-literal object references.

## Affected paths

Expected:

- `crates/ir/src/semantic.rs`
- `crates/cli/tests/ir_lowering.rs`
- `fixtures/`

Do not touch:

- backend equality lowering
- unrelated object/array runtime representation

## Acceptance criteria

- [ ] The focused `if ({ a: 1 } === { a: 1 }) {}` case reports a source-spanned diagnostic.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/conditionalEqualityOnLiteralObjects.ts` no longer reports `BuildPass` for the first condition.
- [ ] Existing primitive equality fixtures continue to build without the new diagnostic.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli --test ir_lowering
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/conditionalEqualityOnLiteralObjects.ts
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from generated bucket
`issues/done/1417-implement-conditionalEqualityOnLiteralObjects.md`.

This issue intentionally starts with only the first object-literal strict
equality condition. The rest of the reference file includes array literals,
local-vs-literal comparisons, `!==`, `==`, and `!=` variants that can be
advanced after this diagnostic path exists.

## Completion Evidence

Fill only when moving to `done/`.
