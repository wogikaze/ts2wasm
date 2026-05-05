---
id: 5154
title: "Parse angle-bracket type assertion statements"
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

Implement the narrow parser erasure slice for TypeScript angle-bracket type assertion expression statements such as `<string>Spec.prop;`.

## Problem

The representative reference case ends with `<string>Spec.prop;`. TypeScript parses this as a `TypeAssertionExpression` wrapped in an expression statement. The current parser does not recognize the top-level angle-bracket assertion shape and fails later while expecting a left brace before the semicolon.

Problem: top-level `<T>expr;` type assertion statements currently fail parsing instead of erasing the type assertion.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseExpressionTypeParameters.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Semicolon) at 254..255
```

Source context:

```text
<string>Spec.prop;
```

TypeScript oracle evidence:

```text
AST path: ExpressionStatement -> TypeAssertionExpression `<string>Spec.prop` -> PropertyAccessExpression `Spec.prop`.
```

## Desired final state

The parser consumes and erases the angle-bracket type assertion at statement expression start, leaving the asserted expression for normal property-access parsing.

## Scope

In scope:

- [ ] Parse `<Identifier>expr` as an erased TypeScript type assertion when it appears at expression statement start.
- [ ] Preserve normal JSX-less TypeScript parsing behavior for property access after the assertion.
- [ ] Add a focused parser regression for `<string>Spec.prop;`.
- [ ] Re-run the representative triage and confirm it no longer reports `expected LeftBrace, got Some(Semicolon)`.

Out of scope:

- Ambiguous generic arrow parsing such as `<T>(x: T) => x`.
- JSX parsing.
- Type checking for the earlier `base<T>()` class heritage diagnostic.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/parser/tests.rs`
- `fixtures/`

Do not touch:

- class heritage type checking
- backend lowering

## Acceptance criteria

- [ ] A focused parser test erases `<string>Spec.prop;` to an expression statement for `Spec.prop`.
- [ ] The representative triage no longer reports `expected LeftBrace, got Some(Semicolon)`.
- [ ] Existing `as` assertion and generic class heritage parser tests continue to pass.
- [ ] Any next blocker from `baseExpressionTypeParameters.ts` is recorded separately if outside this assertion syntax slice.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend angle_bracket_type_assertion_statement
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseExpressionTypeParameters.ts
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/baseExpressionTypeParameters.ts --detail
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

Split from generated bucket `issues/done/1036-implement-baseExpressionTypeParameters.md`.

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
