---
id: 5464
title: "Report nested unary expression syntax diagnostics"
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

Report a source-spanned TypeScript syntax/type diagnostic for a pathological
binary `%` expression followed by a long chain of prefix `!` operators with no
operand, instead of failing with `UnsupportedSyntax: unsupported expression:
None`.

Split from generated bucket
`issues/done/3492-implement-nestedUnaryExpressionHang.md`.

## Problem

Problem: `nestedUnaryExpressionHang.ts` no longer hangs, but the compiler still
fails before producing a useful source diagnostic:

```text
UnsupportedSyntax: unsupported expression: None
```

The lexer tokenizes the representative as `Number(3333)`, `Percent`, and 28
`Bang` tokens. TypeScript reports diagnostics at the malformed right-hand side,
including TS2363 and TS1109, rather than an internal unsupported-expression
boundary.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nestedUnaryExpressionHang.ts
```

Representative source:

```ts
3333%!!!!!!!!!!!!!!!!!!!!!!!!!!!!
```

Compiler evidence:

```text
tokens: ok; Number(3333), Percent, then 28 Bang tokens
ast: fails before AST construction
resolved: fails with the same UnsupportedSyntax
diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
message: unsupported expression: None
```

TypeScript oracle evidence:

```text
TS2363: The right-hand side of an arithmetic operation must be of type 'any',
'number', 'bigint' or an enum type.
TS1109: Expression expected.
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nestedUnaryExpressionHang.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

## Desired final state

The parser or frontend diagnostic layer reports a source-spanned syntax
diagnostic for the missing operand in `3333%!!!!!!!!!!!!!!!!!!!!!!!!!!!!`,
matching the TypeScript oracle shape closely enough for triage to advance past
the generic `unsupported expression: None` failure.

## Scope

In scope:

- [ ] Detect prefix-unary chains that reach end-of-file without an operand.
- [ ] Preserve the `%` binary-expression context so the diagnostic points at the
  malformed right-hand side rather than at the top-level statement.
- [ ] Add focused parser or CLI coverage for
  `3333%!!!!!!!!!!!!!!!!!!!!!!!!!!!!`.
- [ ] Re-run `nestedUnaryExpressionHang.ts` triage and record the resulting
  diagnostic.

Out of scope:

- Full TypeScript checker parity for TS2363.
- General unary operator lowering such as `+x`, `-x`, `~x`, `!x`; broader valid
  unary operator behavior is tracked by generated unary operator buckets.
- Recovery for unrelated malformed expressions.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/diagnostic.rs`
- focused frontend/parser or CLI diagnostic tests

Do not touch:

- backend/runtime lowering for valid unary operators unless the focused parser
  test proves the failure has moved past parsing

## Acceptance criteria

- [ ] `nestedUnaryExpressionHang.ts` no longer reports
  `UnsupportedSyntax: unsupported expression: None`.
- [ ] A focused test covers
  `3333%!!!!!!!!!!!!!!!!!!!!!!!!!!!!` and asserts a source-spanned diagnostic.
- [ ] Valid logical-not expressions such as `!!x` still parse or continue to
  hit their existing non-parser blocker.
- [ ] If the representative advances to a different blocker, this issue records
  that blocker before closure.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend parser
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nestedUnaryExpressionHang.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nestedUnaryExpressionHang.ts --detail --no-dashboard-data
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

This is a malformed TypeScript input case. The goal is not to compile it; the
goal is to replace the internal `unsupported expression: None` boundary with a
source-spanned diagnostic compatible with TypeScript's TS1109/TS2363 failure
shape.

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
