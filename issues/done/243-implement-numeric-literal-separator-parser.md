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
completed: 2026-04-29
status: done
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

- [x] Parse decimal, binary, octal, and hexadecimal number literals with valid `_` separators.
- [x] Reject leading, trailing, doubled, and otherwise invalid separator placement.
- [x] Preserve source spans for the original literal.
- [x] Add parser/unit and CLI dump coverage for valid and invalid separator forms.

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

- [x] `let x = 1_000;` parses and `dump --ast --unparse` emits a numeric expression equivalent to `1000`.
- [x] At least one decimal, binary, octal, and hexadecimal valid separator case is covered.
- [x] At least one invalid doubled or trailing separator case reports an explicit diagnostic.
- [x] A focused reference slice under `reference/test262/test/language/literals/numeric/numeric-separators/` no longer fails due to `_000` being tokenized as an identifier.

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

- [x] updated: `docs/language-reference/javascript-features.md`

Current state:

- [x] updated: `current-state.md`

Follow-up issues:

- [x] none

## Notes

This is a frontend/parser wave child issue split from issue 059 and `docs/language-reference/frontend-parser-wave.md`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `8d5c409` (`issue-243: support numeric literal separators`)
- close commit records this issue completion

Validation result:

```text
command: tmp=/tmp/ts2wasm-243-numeric-separator.ts; printf 'let x = 1_000;\nconsole.log(x);\n' > "$tmp"; cargo run -q -p ts2wasm-cli -- dump --ast --unparse "$tmp"
result: pass; output normalized `1_000` to `1000`
date: 2026-04-29

command: cargo fmt --all --check
result: pass
date: 2026-04-29

command: cargo nextest run -p ts2wasm-frontend
result: pass; 61 tests run, 61 passed
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli --test dump_cli
result: pass; 25 tests run, 25 passed
date: 2026-04-29

command: TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --path-filter reference/test262/test/language/literals/numeric/numeric-separators/numeric-separator-literal-nzd-nsl-dd.js --detail
result: pass; executed=1, unsupported=1, unsupported_diagcodes=UnresolvedName:1, unsupported_features=name-resolution:1; no parser-syntax or `_000` identifier split remains
date: 2026-04-29
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/243-implement-numeric-literal-separator-parser.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
