---
id: 5302
title: "Parse fractional number literals in expressions"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Parse ordinary fractional numeric literals with an integer part in expression
position, starting with `0.5` in a comparison before a ternary:

```ts
Math.random() > 0.5 ? null : t
```

## Problem

Problem: `conditionalTypeAssignabilityWhenDeferred.ts` currently fails before
conditional type semantics because the parser treats `0.5` as a member-access
dot followed by `Number(5)`.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/conditionalTypeAssignabilityWhenDeferred.ts
```

Current compiler diagnostic:

```text
UnsupportedSyntax: expected member property name, got Number(5) at 946..947
```

Representative source:

```ts
function f<T>(t: T) {
  var x: T | null = Math.random() > 0.5 ? null : t;
}
```

Compiler evidence:

- Tokens reach `Math.random() > 0.5 ? null : t`.
- AST construction fails before resolved IR with `expected member property name, got Number(5)`.
- TypeScript parses the file and reports later type diagnostics, not a numeric literal parse error.

## Desired final state

The parser accepts `0.5` as a numeric literal in this expression and the
representative reference advances to the next narrower blocker.

## Scope

In scope:

- [ ] Parse `Number(0) Dot Number(5)` as one fractional numeric expression literal.
- [ ] Preserve normal member access parsing for non-numeric dot expressions.
- [ ] Add focused parser coverage for `Math.random() > 0.5`.

Out of scope:

- Leading-decimal `.5` literals, owned by issue 5191.
- Double-dot numeric member access, owned by issue 5296.
- Full floating-point runtime semantics beyond parsing the source construct.

## Affected paths

Expected:

- `crates/frontend/src/parser/expressions.rs`
- `crates/frontend/src/parser/tests.rs`
- `fixtures/` or focused frontend parser tests

Do not touch:

- backend number representation
- conditional type semantics

## Acceptance criteria

- [ ] `Math.random() > 0.5` parses as a comparison against a numeric literal instead of member access.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/conditionalTypeAssignabilityWhenDeferred.ts` no longer reports `expected member property name, got Number(5)` at `946..947`.
- [ ] Existing member access parsing such as `obj.x` remains unchanged.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/conditionalTypeAssignabilityWhenDeferred.ts
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
`issues/done/1422-implement-conditionalTypeAssignabilityWhenDeferred.md`.

Related but not duplicates:

- Issue 5191 handles leading-decimal `.5`.
- Issue 5296 handles double-dot numeric member access.
- Issue 680 shows the same `Math.random() > 0.5` parser blocker in another
  generated bucket.

## Completion Evidence

Fill only when moving to `done/`.
