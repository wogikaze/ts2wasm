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
completed: 2026-04-29
status: done
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

- [x] Recognize decimal, binary, octal, and hexadecimal BigInt literal forms with `n` suffix.
- [x] Reject invalid BigInt numeric forms, including fractional or exponent forms.
- [x] Add parser/unit and CLI diagnostic coverage for BigInt syntax.
- [x] Keep BigInt runtime semantics out of semantic-pass claims unless separately implemented.

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

- [x] `1n` is no longer parsed as `Number(1)` followed by `Ident("n")`.
- [x] `dump --ast --unparse` or CLI diagnostics show an explicit BigInt literal classification.
- [x] Invalid BigInt forms such as `1.0n` and `1e2n` report stable diagnostics.
- [x] A focused reference slice under `reference/test262/test/language/literals/bigint/` no longer reports the current semicolon/identifier parser failure for non-separator BigInt forms in this issue scope.

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

- [x] updated: `docs/language-reference/javascript-features.md`

Current state:

- [x] updated: `current-state.md`

Follow-up issues:

- [x] created: `issues/open/250-design-bigint-runtime-value-support.md`

## Notes

This is a frontend/parser wave child issue split from issue 059 and `docs/language-reference/frontend-parser-wave.md`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `819e864` issue-244: classify bigint literals

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-04-29

command: cargo nextest run -p ts2wasm-frontend
result: pass; 63 tests run, 63 passed
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli --test dump_cli
result: pass; 26 tests run, 26 passed
date: 2026-04-29

command: TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --path-filter reference/test262/test/language/literals/bigint/ --detail
result: pass; executed=59, unsupported=59, unsupported_diagcodes=UnsupportedSyntax:59. Non-separator BigInt invalid forms now classify as issue-linked unsupported diagnostics; numeric separator cases remain parser-syntax under issue 243.
date: 2026-04-29

command: mise run update-issue-index
result: pass
date: 2026-04-29

command: mise run update-issue-index -- --check
result: pass
date: 2026-04-29

command: mise run check issues
result: pass
date: 2026-04-29
```

Remaining risks:

- BigInt runtime representation and operations are out of scope and tracked by issue 250.
- BigInt numeric separator syntax is out of scope and tracked by issue 243.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/244-implement-bigint-literal-parser-classification.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
