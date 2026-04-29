---
id: 247
title: "Implement destructuring binding pattern parser support"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P2
depends_on: []
blocks: ["059"]
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Implement ECMA-262 binding pattern parsing for variable declarations and function parameters.

Problem: Array and object binding patterns in declarations are rejected before AST construction, so destructuring syntax reports `parser-syntax` at the first `[` or `{`.

## Current failure

Representative reproduction:

```sh
tmp=/tmp/ts2wasm-247-destructuring.ts
printf 'let arr = [1, 2];\nlet [a, b] = arr;\nconsole.log(a);\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- dump --ast --unparse "$tmp"
```

Current result:

```text
error: [UnsupportedSyntax] expected identifier, got Some(SpannedToken { kind: LeftBracket, span: Span { start: 22, end: 23 } }) at 23..24
```

Spec refs:

- `reference/ecma262/spec.html`: BindingPattern, ObjectBindingPattern, ArrayBindingPattern
- `reference/test262/test/language/destructuring/binding/syntax/`

## Desired final state

Array and object binding patterns parse into explicit AST forms for declarations and parameters. Runtime binding semantics may be implemented in a follow-up issue if the parser slice only records AST and diagnostics.

## Scope

In scope:

- [ ] Parse array binding patterns in `let`, `const`, and `var` declarations.
- [ ] Parse object binding patterns in `let`, `const`, and `var` declarations.
- [ ] Parse array/object binding patterns in ordinary function and arrow parameters.
- [ ] Cover elisions, rest binding, default initializers, and nested binding patterns at parser level.
- [ ] Reject invalid rest placement and invalid binding targets with diagnostics.

Out of scope:

- Destructuring assignment expressions such as `({ x } = obj)`.
- `for-in` / `for-of` destructuring heads.
- Full iterator/property runtime semantics for destructuring.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/`
- `fixtures/`
- `docs/language-reference/javascript-features.md`

Do not touch:

- unrelated runtime builtins
- broad iterator protocol implementation

## Acceptance criteria

- [ ] `let [a, b] = arr;` and `let { x } = obj;` parse without the current `expected identifier` diagnostic.
- [ ] Function parameters `function f([a], { x }) {}` and arrow parameters `([a]) => a` parse at AST level.
- [ ] Invalid rest placement such as `let [...a, b] = arr;` reports an explicit diagnostic.
- [ ] A focused reference slice under `reference/test262/test/language/destructuring/binding/syntax/` no longer fails at the first binding-pattern token.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-cli --test dump_cli
mise run update-issue-index
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/destructuring/binding/syntax/ --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] update `docs/language-reference/javascript-features.md` when support status changes

Current state:

- [ ] update `current-state.md` if binding/runtime support changes

Follow-up issues:

- [ ] create runtime destructuring binding issue if this slice stops at AST support
- [ ] create assignment-pattern issue for `({ x } = obj)` separately

## Notes

This is a frontend/parser wave child issue split from issue 059 and `docs/language-reference/frontend-parser-wave.md`.

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
