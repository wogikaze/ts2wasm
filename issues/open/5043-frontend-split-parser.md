---
id: 5043
title: "[frontend] Split large lexer/parser files by grammar responsibility (audit reopened #5043)"
type: refactor
area: frontend
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-06
status: done
---

## Summary

`lexer.rs` や statement/expression parser が肥大化しているため、literal、binding、class、module、TS erasure などへ分割する。

## Problem

frontend の lexer/parser ファイルが肥大化しており、保守性とテスト容易性が低下している。

## Current failure

単一ファイルの巨大化により、コードレビューが難しく、並行開発が困難。

## Desired final state

lexer/parser が文法責任単位（literal、binding、class、module、TS erasure）に分割される。

## Scope

In scope:
- [x] lexer の分割（literal, identifier, operator など）
- [x] expression parser の分割
- [x] statement parser の分割
- [x] class/module/TS erasure の分離

Out of scope:
- [x] ロジックの変更
- [x] 新機能の追加

## Affected paths

Expected:
- `crates/frontend/src/`

## Acceptance criteria

- [x] 各機能単位のファイルが存在する
- [x] 既存テストがすべて通過する

## Validation

```sh
cargo fmt --all --check
cargo nextest run
```

## Docs / current-state / issue sync

Final-state docs:
- [x] not affected

Current state:
- [x] not affected

Follow-up issues:
- [x] none

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/5043-frontend-split-parser.md` before this closure
- `issues/open/5043-frontend-split-parser.md` after this closure

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.

## Completion evidence

### Lexer split (this commit)

- `crates/frontend/src/lexer_numbers.rs` (382 lines) — number literal parsing (`number`, `decimal_number_digits`, `radix_number_digits`, BigInt helpers)
- `crates/frontend/src/lexer_strings.rs` (193 lines) — string literal parsing (`string`, `legacy_octal_escape_value`, `hex_escape_value`)
- `crates/frontend/src/lexer_identifiers.rs` (247 lines) — identifier/keyword parsing (`ident_or_keyword`, `starts_identifier`, `unicode_identifier_escape`, `private_identifier`)
- `crates/frontend/src/lexer.rs` reduced from 1929 to 1114 lines — retains struct definition, cursor helpers, operator/delimiter tokenization loop, regexp/template literal parsing, and `include!()` of the three split files

Existing previously split:
- `lexer_tokens.rs` — `Token`/`SpannedToken`/`TokenKind` definitions
- `lexer_helpers.rs` — character classification and directive helper functions
- `lexer_tests.rs` — unit tests

### Parser split (pre-existing in the codebase at time of reopening)

- `parser/expressions.rs` → includes `expressions_main.rs` (1886 lines) + `expressions_destructure.rs` (315 lines)
- `parser/statements.rs` → includes `statements_core.rs`, `statements_ts.rs`, `statements_class.rs`, `statements_general.rs`
- `parser/binding_patterns.rs` (325 lines) — binding/destructuring patterns
- `parser/helpers.rs` — parser helper utilities
- `parser/tokens.rs` — parser token definitions

### Validation

```
cargo fmt --all --check   # clean
cargo nextest run -p ts2wasm-frontend   # 139/139 passed
cargo build               # full workspace build clean
```
