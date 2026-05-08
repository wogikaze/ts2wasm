---
id: 5274
title: "Parse general comma expressions"
type: feature
area: frontend
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Parse JavaScript/TypeScript comma expressions in ordinary expression positions,
including parenthesized initializer expressions and return expressions.
This also covers parenthesized comma expressions used as member/call receivers,
such as `(otherValue(), value).inner`.

## Problem

Problem: general comma expressions currently fail with UnsupportedSyntax before AST construction in ordinary expression positions.

`commaOperator1.ts` tokenizes successfully, but AST construction rejects the
first parenthesized comma expression in `var v1 = ((1, 2, 3), 4, 5, (6, 7));`.
TypeScript parses the expression and reports TS2695 diagnostics for unused
left-hand comma operands.

Current diagnostic:

```text
UnsupportedSyntax: comma expressions are not supported in this parser slice at 63..72
```

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commaOperator1.ts
```

Representative source:

```ts
var v1 = ((1, 2, 3), 4, 5, (6, 7));
function f1() {
    var a = 1;
    return a, v1, a;
}
```

Compiler evidence:

```text
tokens: ok; includes nested comma tokens in initializer and return expression
AST: fails at the first parenthesized comma expression
TypeScript oracle: parses comma expressions as BinaryExpression chains and reports TS2695 diagnostics
```

## Desired final state

The parser represents comma expressions in ordinary expression positions rather
than rejecting them before AST construction. The representative reference
should advance past the current `comma expressions are not supported` parser
failure to TS2695-compatible diagnostics or the next semantic blocker.

## Scope

In scope:

- [ ] Parse comma expressions in variable initializers.
- [ ] Parse comma expressions in return expressions.
- [ ] Parse comma expressions in `case` label expressions, such as `case 0, 1:`.
- [ ] Parse parenthesized comma expressions before member access and call
  arguments, such as `(otherValue(), value).inner`.
- [ ] Preserve left-to-right expression order in the AST/dump or equivalent
  representation.
- [ ] Add focused parser tests for `((1, 2, 3), 4, 5, (6, 7))` and
  `return a, v1, a;`.
- [ ] Re-run the representative reference triage and record the next diagnostic
  or pass state.

Out of scope:

- Full TS2695 unused-left-side diagnostic compatibility.
- Runtime lowering for arbitrary comma expressions beyond proving the parser
  shape is represented.
- Comma-separated `for` update expressions, tracked by
  `issues/open/5182-parse-comma-separated-for-update-expressions.md`.

## Affected paths

Expected:

- `crates/frontend/src/ast.rs`
- `crates/frontend/src/parser/expressions_main.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/compiler/src/dump.rs`
- focused CLI/reference tests

Do not touch:

- `crates/backend-wasm/` unless lowering already supports the resulting shape
- unrelated statement parsing

## Acceptance criteria

- [ ] `commaOperator1.ts` no longer reports `comma expressions are not
  supported in this parser slice`.
- [ ] A focused parser test covers nested comma expressions in a variable
  initializer.
- [ ] A focused parser test covers comma expressions in a return statement.
- [ ] `commaOperatorLeftSideUnused.ts` no longer reports `expected Colon, got
  Some(Comma)` for `case 0, 1:`.
- [ ] `narrowCommaOperatorNestedWithinLHS.ts` no longer reports `comma
  expressions are not supported in this parser slice` for
  `(otherValue(), value).inner`.
- [ ] Existing expression precedence tests still pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(comma) or test(expression)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commaOperator1.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commaOperator1.ts --detail --no-dashboard-data
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

Split from generated bucket `issues/done/1336-implement-commaOperator.md`.

Related but distinct:

- `issues/open/5182-parse-comma-separated-for-update-expressions.md` handles
  the narrow `for (...; ...; ++x, --y)` update slot.
- `issues/done/1338-implement-commaOperatorLeftSideUnused.md` is a generated
  blocked bucket for TS2695 diagnostics after parser support exists.

2026-05-07 additional evidence: `commaOperatorLeftSideUnused.ts` stops before
the later TS2695 diagnostic cases at `case 0, 1:`. Tokens are present for
`Case Number(0) Comma Number(1) Colon`, but AST construction fails with
`UnsupportedSyntax: expected Colon, got Some(Comma) at 179..180`. TypeScript
parses this as a comma expression in the case label and reports TS2695 on the
left operand. The later parenthesized comma expressions in assignments and
calls remain unproven until this case-label parser boundary advances.

2026-05-08 additional evidence: generated bucket
`issues/done/3445-implement-narrowCommaOperatorNestedWithinLHS.md` folds into
this general parser owner. `narrowCommaOperatorNestedWithinLHS.ts` fails before
control-flow narrowing at `typeof (otherValue(), value).inner === 'number'`:
tokens are ok, TypeScript oracle has no diagnostics, and AST construction
reports `UnsupportedSyntax: comma expressions are not supported in this parser
slice at 226..247` for the parenthesized comma expression used as the member
receiver.

## Completion evidence

Fill when implemented.
