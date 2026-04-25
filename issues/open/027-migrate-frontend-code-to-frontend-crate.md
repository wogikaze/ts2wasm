---
id: 027
title: "Migrate frontend code to frontend crate"
type: refactor
area: frontend
class: implementation-ready
priority: P1
depends_on: [024]
blocks: [025, 026]
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Move lexer/parser/AST/Span/Diagnostic code from `crates/cli/src/lib.rs` to `crates/frontend/src/` and update imports in cli.

## Problem

Frontend code (lexer, parser, AST, span, diagnostics) is currently in `crates/cli/src/lib.rs`, violating the target crate structure defined in AGENTS.md. This mixes frontend concerns with CLI orchestration and makes the 104KB lib.rs file unmanageable.

## Desired final state

- `crates/frontend/src/` contains lexer.rs, parser.rs, ast.rs, span.rs, diagnostic.rs
- `crates/cli/src/lib.rs` only contains orchestration code (build_file, build_file_with_options)
- `crates/cli` depends on `ts2wasm-frontend`
- All imports in cli updated to use `ts2wasm_frontend::`

## Scope

In scope:

- [ ] Extract lexer code from `crates/cli/src/lib.rs` → `crates/frontend/src/lexer.rs`
- [ ] Extract parser code from `crates/cli/src/lib.rs` → `crates/frontend/src/parser.rs`
- [ ] Extract AST types from `crates/cli/src/lib.rs` → `crates/frontend/src/ast.rs`
- [ ] Extract Span/Diagnostic from `crates/cli/src/lib.rs` → `crates/frontend/src/diagnostic.rs`
- [ ] Add `ts2wasm-frontend` dependency to `crates/cli/Cargo.toml`
- [ ] Update all imports in `crates/cli/src/lib.rs` to use `ts2wasm_frontend::`
- [ ] Ensure `crates/cli/src/lib.rs` only contains orchestration functions

Out of scope:

- Changing frontend contracts or semantics
- TypeScript parser integration (separate issue 019)

## Affected paths

Expected (after migration):

- crates/frontend/src/lexer.rs
- crates/frontend/src/parser.rs
- crates/frontend/src/ast.rs
- crates/frontend/src/diagnostic.rs
- crates/cli/Cargo.toml
- crates/cli/src/lib.rs

Do not touch:

- `crates/shared/`
- `crates/runtime-abi/`
- `crates/ir/`
- `crates/backend-wasm/`
- `docs/`
- `scripts/`
- `fixtures/`

## Acceptance criteria

- [ ] `crates/frontend/src/` contains full frontend implementation
- [ ] `crates/cli/src/lib.rs` is reduced to orchestration only (< 200 lines)
- [ ] `cargo check` passes
- [ ] `cargo nextest run` passes (all 205 tests)
- [ ] No behavior changes in frontend semantics

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo check
cargo test
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] created: `issues/open/028-migrate-lexer-parser-ast-to-frontend-crate.md`

## Notes

This is the most complex migration because frontend code is interleaved in lib.rs. Carefully extract types and functions while preserving all behavior.

Frontend depends on `ts2wasm-shared` for Diagnostic types if needed.

Import replacement pattern:
- `Span` → `ts2wasm_frontend::Span`
- `Diagnostic` → `ts2wasm_frontend::Diagnostic`
- `DiagCode` → `ts2wasm_frontend::DiagCode`
- `Lexer` → `ts2wasm_frontend::Lexer`
- `Parser` → `ts2wasm_frontend::Parser`
- AST types → `ts2wasm_frontend::*`

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
