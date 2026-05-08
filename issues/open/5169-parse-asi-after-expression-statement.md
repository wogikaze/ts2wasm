---
id: 5169
title: "Parse ASI after expression statement"
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

`bigintWithLib.ts` and `bigintWithoutLib.ts` contain a completed expression statement without a semicolon, followed by a blank line and a `let` declaration. TypeScript accepts this through automatic semicolon insertion, but the parser currently expects an explicit `Semicolon` and fails at `let`.

## Problem

Problem: the BigInt lib reference cases report `UnsupportedSyntax: expected Semicolon, got Some(Let)` after:

```ts
stringVal = bigintVal.toLocaleString('de-DE', { style: 'currency', currency: 'EUR' })

let bigIntArray: BigInt64Array = new BigInt64Array();
```

`bigintWithoutLib.ts` has the same ASI boundary with a trailing comment after the expression statement.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigintWithLib.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigintWithoutLib.ts
```

Current compiler diagnostic:

```text
UnsupportedSyntax: expected Semicolon, got Some(Let) at 660..663
UnsupportedSyntax: expected Semicolon, got Some(Let) at 998..1001
```

Compiler evidence:

- Tokens are successful through the completed `toLocaleString(...)` call and the following `Let`.
- AST construction fails because the expression statement parser requires an explicit semicolon.
- Existing issue `5151` covers ASI after a multi-line `const` initializer; this is the expression-statement counterpart.

TypeScript oracle evidence:

```text
TS accepts the semicolonless expression statement and reports later BigInt library/type diagnostics.
```

## Desired final state

The parser accepts ASI after a completed expression statement when the next
token starts a new statement on a later line or closes the containing block.
The representative references should no longer fail with `expected Semicolon,
got Some(Let)` or `expected Semicolon, got Some(RightBrace)`.

## Scope

In scope:

- [ ] Accept ASI after a completed expression statement before `let`, `const`, `var`, `function`, `class`, `import`, `export`, or other established statement starters when separated by a line terminator.
- [ ] Accept ASI after a completed expression statement before a closing block,
  such as `b = () => { ... }\n}`.
- [ ] Preserve required semicolon behavior where ASI must not apply, especially postfix/continuation tokens.
- [ ] Add focused parser coverage for a method-call assignment expression followed by `let`.
- [ ] Re-run representative triage and confirm the current `Some(Let)` parser blocker is gone.

Out of scope:

- Broader ASI policy for every ECMAScript restricted production.
- BigInt64Array, BigUint64Array, DataView, or Intl BigInt library semantics after parsing advances.
- TypeScript declaration emit behavior later in the same reference.

## Affected paths

Expected:

- `crates/frontend/src/parser/tokens.rs`
- `crates/frontend/src/parser/statements_general.rs`
- `crates/frontend/src/parser/expressions_main.rs`
- `crates/frontend/src/parser/tests.rs`

Do not touch:

- runtime/builtin lowering unless ASI advances and proves a downstream blocker.
- declaration emit code for this parser-only slice.

## Acceptance criteria

- [ ] `parse_program("a = b.c()\\nlet x = 1;")` succeeds as two statements.
- [ ] Parser tests cover line-terminator ASI after a completed call/member expression statement.
- [ ] Existing invalid continuation cases remain rejected.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigintWithLib.ts` no longer reports `expected Semicolon, got Some(Let)`.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigintWithoutLib.ts` no longer reports `expected Semicolon, got Some(Let)`.
- [ ] `narrowRefinedConstLikeParameterBIndingElementNameInInnerScope.ts` no
  longer reports `expected Semicolon, got Some(RightBrace)` after
  `b = () => { ... }`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigintWithLib.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigintWithoutLib.ts
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

Split from generated bucket `1053` on 2026-05-06 and expanded with generated bucket `1054` after both representative files stopped at the same expression-statement ASI boundary. Any BigInt library semantics that appear after this parser blocker should be triaged separately.

Also supersedes generated bucket `issues/done/1183-implement-classExpressionNames.md`: fresh triage for `classExpressionNames.ts` stops after the completed expression statement `A = class {}` before the later-line `var a = new A()`, which is the same ASI-after-expression-statement parser boundary.

Also supersedes generated bucket
`issues/done/3446-implement-narrowRefinedConstLikeParameterBIndingElementNameInInnerScope.md`:
fresh triage stops after the completed assignment expression statement
`b = () => { const x: string = a; }` before the closing `}` of the `if` block.
Tokens and TypeScript oracle are ok, and AST construction reports
`UnsupportedSyntax: expected Semicolon, got Some(RightBrace) at 196..197`.

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
