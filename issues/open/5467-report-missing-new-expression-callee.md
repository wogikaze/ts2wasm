---
id: 5467
title: "Report missing new expression callee"
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

Report a source-spanned syntax diagnostic for `new ()` instead of failing with
a generic `unsupported expression` boundary at the closing parenthesis.

Split from generated bucket
`issues/done/3502-implement-newMissingIdentifier.md`.

## Problem

Problem: `newMissingIdentifier.ts` contains malformed `new ()` syntax. The
current parser tokenizes the input, then stops before AST construction with a
generic unsupported-expression diagnostic:

```text
UnsupportedSyntax: unsupported expression: Some(SpannedToken { kind: RightParen, ... }) at 34..35
```

TypeScript parses a `NewExpression` containing an empty parenthesized
expression and reports TS1109 `Expression expected.` at the missing callee.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/newMissingIdentifier.ts
```

Representative source:

```ts
var x = new ();
```

Compiler evidence:

```text
tokens: ok; Var, Ident("x"), Equal, New, LeftParen, RightParen, Semicolon
ast: fails before AST construction
resolved: fails with the same parser diagnostic
diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
message: unsupported expression at RightParen
```

TypeScript oracle evidence:

```text
TS1109: Expression expected.
AST path: VariableDeclaration -> NewExpression -> ParenthesizedExpression "()"
binding hint: x has type any
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/newMissingIdentifier.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

## Desired final state

The parser or frontend diagnostic layer reports a source-spanned
missing-expression diagnostic for `new ()`, matching the TypeScript TS1109
shape closely enough for triage to advance past the generic unsupported
expression failure.

## Scope

In scope:

- [ ] Detect `new ()` with no callee expression and emit a source-spanned
  missing-expression diagnostic.
- [ ] Add focused parser or CLI diagnostic coverage for `var x = new ();`.
- [ ] Preserve valid `new ClassName()` and `new (expr)()` behavior.
- [ ] Re-run `newMissingIdentifier.ts` triage and record the resulting
  diagnostic.

Out of scope:

- Dynamic constructor callee support.
- Indexed new callee diagnostics such as `new any[1]`, tracked by
  `issues/open/5203-report-indexed-new-type-only-callee-diagnostics.md`.
- Malformed angle-bracket new expressions, tracked by
  `issues/open/5466-report-malformed-new-angle-bracket-casts.md`.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/diagnostic.rs`
- focused frontend/parser or CLI diagnostic tests

Do not touch:

- backend/runtime lowering for valid new expressions unless the focused parser
  test proves the failure has moved past parsing

## Acceptance criteria

- [ ] `newMissingIdentifier.ts` no longer reports generic
  `unsupported expression` at the `)` token in `new ()`.
- [ ] A focused test covers `var x = new ();` and asserts a source-spanned
  missing-expression diagnostic.
- [ ] Existing valid `new ClassName()` and `new (expr)()` coverage remains
  green or continues to hit its existing owner.
- [ ] If the representative advances to a different blocker, this issue records
  that blocker before closure.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend parser
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/newMissingIdentifier.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/newMissingIdentifier.ts --detail --no-dashboard-data
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

This is an invalid TypeScript input case. The goal is not to compile it; the
goal is to replace the internal unsupported-expression boundary with a
source-spanned diagnostic compatible with TypeScript's TS1109 failure shape.

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
