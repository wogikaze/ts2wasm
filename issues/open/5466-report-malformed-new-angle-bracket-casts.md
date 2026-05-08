---
id: 5466
title: "Report malformed new angle-bracket casts"
type: bug
area: frontend/parser
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Report a source-spanned syntax/type diagnostic for malformed
`new <any>Test2()` input instead of failing with an internal
`unsupported expression` boundary at the `<` token.

Split from generated bucket
`issues/done/3497-implement-newExpressionWithCast.md`.

## Problem

Problem: `newExpressionWithCast.ts` contains both valid and invalid new/cast
forms. The current compiler handles the preceding `new Test()` but stops at the
invalid `new <any>Test2()` expression:

```text
UnsupportedSyntax: unsupported expression: Some(SpannedToken { kind: Less, ... }) at 188..191
```

TypeScript does not treat `new <any>Test2()` as a valid type assertion. It
parses the expression as binary syntax and reports diagnostics, including
TS1109 at `<` and TS2365 for the resulting operator expression.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/newExpressionWithCast.ts
```

Representative source:

```ts
function Test2() { }
var test2 = new <any>Test2();

function Test3() { }
var test3 = new (<any>Test3)();
```

Compiler evidence:

```text
tokens: ok; New, Less, Ident("any"), Greater, Ident("Test2"), call parens
ast: fails before AST construction
resolved: fails with the same parser diagnostic
diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
message: unsupported expression at Less
```

TypeScript oracle evidence:

```text
TS2365: Operator '>' cannot be applied to types 'boolean' and 'void'.
TS1109: Expression expected.
TS2693: 'any' only refers to a type, but is being used as a value here.
AST path: VariableDeclaration -> BinaryExpression "new <any>Test2()" ->
BinaryExpression "new <any" -> FirstBinaryOperator "<" -> Identifier "any"
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/newExpressionWithCast.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

## Desired final state

The parser or frontend diagnostic layer reports a source-spanned diagnostic for
`new <any>Test2()` that matches the TypeScript oracle shape closely enough for
triage to advance past the generic `unsupported expression` failure.

## Scope

In scope:

- [ ] Detect `new <...>` malformed expression syntax and report a source-spanned
  diagnostic at the `<` or malformed right-hand side.
- [ ] Preserve valid `new (<any>Test3)()` parsing behavior or its existing next
  blocker.
- [ ] Add focused parser or CLI diagnostic coverage for `new <any>Test2()`.
- [ ] Re-run `newExpressionWithCast.ts` triage and record the resulting next
  blocker.

Out of scope:

- General angle-bracket assertion statements, tracked by
  `issues/open/5154-parse-angle-bracket-type-assertion-statements.md`.
- Object/array angle-bracket type assertions such as
  `<{ id: number }[]>[...]`, tracked by generated array-cast buckets.
- Full TypeScript checker parity for TS2365 or TS2693.
- Broad new-expression runtime support.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/diagnostic.rs`
- focused frontend/parser or CLI diagnostic tests

Do not touch:

- backend/runtime lowering for valid new expressions unless the focused parser
  test proves the failure has moved past parsing

## Acceptance criteria

- [ ] `newExpressionWithCast.ts` no longer reports `unsupported expression` at
  the `<` token in `new <any>Test2()`.
- [ ] A focused test covers `new <any>Test2()` and asserts a source-spanned
  diagnostic.
- [ ] Existing valid `new Test()` coverage remains green.
- [ ] If the representative advances to a different blocker, this issue records
  that blocker before closure.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend parser
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/newExpressionWithCast.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/newExpressionWithCast.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
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

The goal is not to make `new <any>Test2()` compile. TypeScript intentionally
reports diagnostics for this malformed form. This issue only removes the
internal unsupported-expression boundary.

## Completion evidence

Fill only when implemented.

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
