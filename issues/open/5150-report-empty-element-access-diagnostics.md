---
id: 5150
title: "Report empty element access diagnostics"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Implement the narrow parser diagnostic slice for empty element access expressions such as `number[]` in value position.

## Problem

The representative TypeScript reference case parses `number[]` as an `ElementAccessExpression` with no argument and reports TS1011. The current parser instead falls through to a generic unsupported expression diagnostic on `RightBracket`, which hides the specific syntax error and keeps the reference bucket classified as unknown unsupported.

Problem: empty element access `expr[]` currently fails with generic `UnsupportedSyntax` instead of a targeted missing-index diagnostic.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badArrayIndex.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: RightBracket, span: Span { start: 40, end: 41 } }) at 41..42
```

Source context:

```text
// @target: es2015
var results = number[];
```

TypeScript oracle evidence:

```text
TS2693: 'number' only refers to a type, but is being used as a value here.
TS1011: An element access expression should take an argument.
TypeScript AST path: ElementAccessExpression, text `number[]`.
```

Current compiler evidence:

```text
tokens: Var results = Ident("number") LeftBracket RightBracket ;
AST/resolved: parser fails before AST with generic unsupported expression diagnostic at the closing bracket.
```

## Desired final state

The parser recognizes an empty element access expression enough to emit a source-spanned, issue-linked diagnostic for the missing index expression, without changing valid element access behavior.

## Scope

In scope:

- [ ] Detect `expr[]` in expression parsing after the element-access opening bracket.
- [ ] Emit a targeted diagnostic for the missing element access argument.
- [ ] Add a focused parser or diagnostic regression for `var results = number[];`.
- [ ] Re-run the representative triage and confirm it no longer reports `unsupported expression: Some(... RightBracket ...)`.

Out of scope:

- Valid element access semantics such as `arr[i]`.
- Type/value namespace checking for `number` beyond existing diagnostics.
- Type-level array syntax such as `let xs: number[]`.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/parser/tests.rs`
- `fixtures/`

Do not touch:

- runtime element access lowering
- TypeScript type checker behavior
- array type syntax handling in type positions

## Acceptance criteria

- [ ] A focused parser or diagnostic test covers `var results = number[];`.
- [ ] The diagnostic for the focused case names the missing element access argument or empty element access, rather than generic unsupported expression.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badArrayIndex.ts` no longer reports `unsupported expression: Some(... RightBracket ...)`.
- [ ] Existing valid element access and array type syntax tests continue to pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend empty_element_access
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badArrayIndex.ts
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/badArrayIndex.ts --detail
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

Split from generated bucket `issues/done/1026-implement-badArrayIndex.md`.

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
