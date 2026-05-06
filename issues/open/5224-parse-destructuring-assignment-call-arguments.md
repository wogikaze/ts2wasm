---
id: 5224
title: "Parse destructuring assignment call arguments"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Parse parenthesized object destructuring assignment expressions used as call
arguments, such as `Test(({ b = "5" } = {}));`, without turning the whole
pattern into a synthetic identifier name.

## Problem

`checkDestructuringShorthandAssigment.ts` tokenizes and builds an AST, but the
AST represents `({ b = "5" } = {})` as an assignment whose target name is the
string `"{b = \"5\"}"`. Name resolution then treats that string as an
identifier and reports `UnresolvedName` before the intended TypeScript
shorthand-property diagnostic can surface.

Current diagnostic:

```text
UnresolvedName: unresolved name: `{b = "5"}` at 174..191
```

## Current failure

Use the validation command below to reproduce the representative failure.

Source shape:

```ts
function Test({ b = "" } = {}) {}
Test(({ b = "5" } = {}));
```

Compiler evidence:

```text
tokens: ok; includes LeftBrace, Ident b, Equal, String "5", RightBrace, Equal, LeftBrace, RightBrace
ast: ok but Call(Test, args=[Assign { name: "{b = \"5\"}", expr: Object {} }])
resolved: UnresolvedName for the synthetic pattern string
TypeScript oracle: TS18004 No value exists in scope for the shorthand property 'b'
```

## Desired final state

The parser represents destructuring assignment call arguments as destructuring
patterns or emits a source-spanned destructuring diagnostic before name
resolution treats the pattern text as an identifier.

## Scope

In scope:

- [ ] Detect parenthesized object destructuring assignments in expression/call-argument position.
- [ ] Avoid synthesizing names such as `{b = "5"}` for object patterns.
- [ ] Add a focused fixture for `Test(({ b = "5" } = {}));`.

Out of scope:

- Full TypeScript shorthand-property diagnostic parity.
- Destructuring runtime semantics for all object patterns.
- Array destructuring assignment forms not needed by the representative path.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/ast.rs`
- `crates/frontend/src/diagnostic.rs`
- `crates/cli/tests/`
- focused fixtures

Do not touch:

- backend destructuring lowering
- unrelated resolver lookup rules

## Acceptance criteria

- [ ] `checkDestructuringShorthandAssigment.ts` no longer reports `UnresolvedName` for `{b = "5"}`.
- [ ] A focused parser or CLI fixture covers `Test(({ b = "5" } = {}));`.
- [ ] Existing simple assignment expressions still resolve ordinary identifiers normally.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend destructuring
cargo nextest run -p ts2wasm-cli -E 'test(destructuring) or test(name)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkDestructuringShorthandAssigment.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkDestructuringShorthandAssigment.ts --detail
```

Not run:

- none

## Notes

Split from generated bucket `issues/done/1130-implement-checkDestructuringShorthandAssigment-name-resolution.md`.

## Completion evidence

Fill when implemented.
