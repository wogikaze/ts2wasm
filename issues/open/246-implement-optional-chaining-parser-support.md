---
id: 246
title: "Implement optional chaining parser support"
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

Implement ECMA-262 optional chaining grammar in the frontend parser and define the semantic handoff.

Problem: The lexer recognizes `?.`, but the parser does not accept optional member access, so `obj?.x` reports `parser-syntax`.

## Current failure

Representative reproduction:

```sh
tmp=/tmp/ts2wasm-246-optional.ts
printf 'let obj = { x: 1 };\nconsole.log(obj?.x);\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- dump --ast --unparse "$tmp"
```

Current result:

```text
error: [UnsupportedSyntax] expected Comma, got Some(OptionalChain) at 35..37
```

Spec refs:

- `reference/ecma262/spec.html`: OptionalExpression, OptionalChain
- `reference/test262/test/language/expressions/optional-chaining/`

## Desired final state

Optional chaining syntax is parsed as a distinct frontend AST shape or lowered supported subset. Unsupported semantic forms produce issue-linked diagnostics after parsing rather than parser token errors.

## Scope

In scope:

- [ ] Parse optional property access `obj?.x`.
- [ ] Parse optional element access `obj?.[key]`.
- [ ] Parse optional call `fn?.()`.
- [ ] Reject invalid optional chaining assignment/update targets.
- [ ] Add parser and CLI dump coverage for accepted and rejected forms.

Out of scope:

- Full `super`, private-name, `eval`, and tagged-template optional-chain semantics.
- Implementing broad object/prototype behavior not already supported by the runtime.
- Nullish coalescing; tracked by issue 245.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/`
- `fixtures/`
- `docs/language-reference/javascript-features.md`

Do not touch:

- unrelated runtime builtins
- capability manifest policy

## Acceptance criteria

- [ ] `obj?.x`, `obj?.[key]`, and `fn?.()` no longer fail with the current `expected Comma, got Some(OptionalChain)` parser error.
- [ ] Invalid assignment/update targets such as `obj?.x = 1` and `obj?.x++` report explicit diagnostics.
- [ ] CLI dump or diagnostics preserve optional-chain structure rather than erasing it accidentally.
- [ ] `reference/test262/test/language/expressions/optional-chaining/member-expression.js` no longer fails due to the current parser comma error.

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
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/expressions/optional-chaining/ --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] update `docs/language-reference/javascript-features.md` when support status changes

Current state:

- [ ] update `current-state.md` if semantic support boundary changes

Follow-up issues:

- [ ] create semantic-lowering child issues for optional-chain forms left unsupported after parsing

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
