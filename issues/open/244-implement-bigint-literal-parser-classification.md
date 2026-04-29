---
id: 244
title: "Implement BigInt literal parser classification"
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

Implement lexer/parser recognition for ECMA-262 BigInt literal syntax and classify the runtime boundary explicitly.

Problem: BigInt literals such as `1n` are tokenized as a numeric literal followed by identifier `n`, so the parser reports `parser-syntax` instead of recognizing BigInt syntax and producing a stable unsupported or supported frontend result.

## Current failure

Representative reproduction:

```sh
tmp=/tmp/ts2wasm-244-bigint.ts
printf 'let x = 1n;\nconsole.log(x);\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- dump --ast --unparse "$tmp"
```

Current result:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("n")) at 9..10
```

Spec refs:

- `reference/ecma262/spec.html`: BigIntLiteralSuffix, BigIntLiteral
- `reference/test262/test/language/literals/bigint/`

## Desired final state

BigInt literals are recognized as BigInt syntax. Until BigInt runtime values are implemented, supported parser paths either carry an explicit BigInt AST/literal classification or emit an issue-linked unsupported diagnostic that is not caused by tokenization drift.

## Scope

In scope:

- [ ] Recognize decimal, binary, octal, and hexadecimal BigInt literal forms with `n` suffix.
- [ ] Reject invalid BigInt numeric forms, including fractional or exponent forms.
- [ ] Add parser/unit and CLI diagnostic coverage for BigInt syntax.
- [ ] Keep BigInt runtime semantics out of semantic-pass claims unless separately implemented.

Out of scope:

- Implementing BigInt value representation, arithmetic, equality, or builtin behavior.
- Numeric separator support for ordinary numbers; tracked by issue 243.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/`
- `fixtures/`
- `docs/language-reference/javascript-features.md`

Do not touch:

- `crates/runtime-abi/` unless this issue is explicitly expanded into runtime BigInt support
- unrelated builtins

## Acceptance criteria

- [ ] `1n` is no longer parsed as `Number(1)` followed by `Ident("n")`.
- [ ] `dump --ast --unparse` or CLI diagnostics show an explicit BigInt literal classification.
- [ ] Invalid BigInt forms such as `1.0n` and `1e2n` report stable diagnostics.
- [ ] A focused reference slice under `reference/test262/test/language/literals/bigint/` no longer reports the current semicolon/identifier parser failure.

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
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/literals/bigint/ --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] update `docs/language-reference/javascript-features.md` if implementation status changes

Current state:

- [ ] update `current-state.md` if BigInt support boundary changes

Follow-up issues:

- [ ] create runtime BigInt issue if parser classification exposes a new semantic gap

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
