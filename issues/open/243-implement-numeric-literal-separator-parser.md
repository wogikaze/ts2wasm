---
id: 243
title: "Implement numeric literal separator parser support"
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

Implement ECMA-262 numeric literal separator syntax for ordinary number literals.

Problem: Numeric literals such as `1_000` are tokenized as a number followed by an identifier suffix, so the parser reports `parser-syntax` instead of accepting the literal or diagnosing invalid separator placement.

## Current failure

Representative reproduction:

```sh
tmp=/tmp/ts2wasm-243-numeric-separator.ts
printf 'let x = 1_000;\nconsole.log(x);\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- dump --ast --unparse "$tmp"
```

Current result:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("_000")) at 9..13
```

Spec refs:

- `reference/ecma262/spec.html`: NumericLiteralSeparator, NumericLiteral
- `reference/test262/test/language/literals/numeric/numeric-separators/`

## Desired final state

Valid numeric separators parse as the same numeric value as the equivalent literal without `_`, and invalid separator placement is rejected with an explicit diagnostic instead of token stream drift.

## Scope

In scope:

- [ ] Parse decimal, binary, octal, and hexadecimal number literals with valid `_` separators.
- [ ] Reject leading, trailing, doubled, and otherwise invalid separator placement.
- [ ] Preserve source spans for the original literal.
- [ ] Add parser/unit and CLI dump coverage for valid and invalid separator forms.

Out of scope:

- BigInt literal support; tracked separately by issue 244.
- Changing the runtime number representation.
- Full fractional double parity beyond the existing number model.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/`
- unrelated runtime builtins

## Acceptance criteria

- [ ] `let x = 1_000;` parses and `dump --ast --unparse` emits a numeric expression equivalent to `1000`.
- [ ] At least one decimal, binary, octal, and hexadecimal valid separator case is covered.
- [ ] At least one invalid doubled or trailing separator case reports an explicit diagnostic.
- [ ] A focused reference slice under `reference/test262/test/language/literals/numeric/numeric-separators/` no longer fails due to `_000` being tokenized as an identifier.

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
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/literals/numeric/numeric-separators/ --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] updated if language-reference coverage rows change

Current state:

- [ ] not affected unless implementation status changes

Follow-up issues:

- [ ] none

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
