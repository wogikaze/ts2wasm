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

Move lexer/parser/AST/Span/Diagnostic code from cli/src/lib.rs to frontend/src/ and update imports in cli.

## Problem

Frontend code (lexer, parser, AST, span, diagnostics) is currently in cli/src/lib.rs, violating the target crate structure defined in AGENTS.md. This mixes frontend concerns with CLI orchestration and makes the 104KB lib.rs file unmanageable.

## Desired final state

- crates/frontend/src/ contains lexer.rs, ast.rs, diagnostic.rs
- cli/src/lib.rs only contains orchestration code (build_file, build_file_with_options)
- crates/cli depends on ts2wasm-frontend
- All imports in cli updated to use ts2wasm_frontend

## Scope

In scope:

- [x] Extract lexer code from cli lib.rs to frontend crate
- [x] Extract AST types from cli lib.rs to frontend crate
- [x] Extract Span/Diagnostic from cli lib.rs to frontend crate
- [x] Add ts2wasm-frontend dependency to cli
- [x] Update all imports in cli lib.rs to use ts2wasm_frontend
- [x] Ensure cli lib.rs only contains orchestration functions

Out of scope:

- Changing frontend contracts or semantics
- TypeScript parser integration (separate issue 019)

## Affected paths

Expected (after migration):

- crates/frontend/src/lexer.rs (DONE)
- crates/frontend/src/ast.rs (DONE)
- crates/frontend/src/diagnostic.rs (DONE)
- crates/cli/Cargo.toml (DONE)
- crates/cli/src/lib.rs (DONE)

Do not touch:

- crates/shared/
- crates/runtime-abi/
- crates/ir/
- crates/backend-wasm/
- docs/

## Acceptance criteria

- [x] crates/frontend/src/ contains full frontend implementation
- [x] cli/src/lib.rs is reduced to orchestration only (< 200 lines)
- [x] cargo check passes
- [x] cargo nextest run passes (all 186 tests)
- [x] No behavior changes in frontend semantics

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

- [x] not affected

Current state:

- [x] updated: current-state.md (repo root)

Follow-up issues:

- [x] created: issues/open/028-migrate-lexer-parser-ast-to-frontend-crate.md (not needed, frontend migration done in this issue)

## Notes

This is the most complex migration because frontend code is interleaved in lib.rs. Carefully extract types and functions while preserving all behavior.

Frontend depends on ts2wasm-shared for Diagnostic types if needed.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `Refactor: Split monolithic cli crate into smaller crates`

Validation result:

```text
command: cargo nextest run
result: 186 tests run: 186 passed, 1 skipped
date: 2026-04-26
```

Remaining risks:

- none
