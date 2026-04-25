---
id: 025
title: "Migrate ir module to ir crate"
type: refactor
area: ir
class: implementation-ready
priority: P1
depends_on: [024, 028]
blocks: [026]
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Move `crates/cli/src/ir/` (builtin/builtin_resolved/builtin_resolver/lowered) to `crates/ir/src/` and update imports in cli and backend-wasm.

## Problem

IR code is currently in `crates/cli/src/ir/`, violating the target crate structure defined in AGENTS.md. This mixes IR concerns with CLI orchestration.

## Desired final state

- `crates/ir/src/` contains builtin.rs, builtin_resolved.rs, builtin_resolver.rs, lowered.rs with full implementation
- `crates/cli/src/ir/` is removed
- `crates/cli` and `crates/backend-wasm` depend on `ts2wasm-ir`
- All imports updated to use `ts2wasm_ir::`

## Scope

In scope:

- [ ] Move `crates/cli/src/ir/builtin.rs` → `crates/ir/src/builtin.rs`
- [ ] Move `crates/cli/src/ir/builtin_resolved.rs` → `crates/ir/src/builtin_resolved.rs`
- [ ] Move `crates/cli/src/ir/builtin_resolver.rs` → `crates/ir/src/builtin_resolver.rs`
- [ ] Move `crates/cli/src/ir/lowered.rs` → `crates/ir/src/lowered.rs`
- [ ] Remove `crates/cli/src/ir/mod.rs`
- [ ] Add `ts2wasm-ir` dependency to `crates/cli/Cargo.toml`
- [ ] Add `ts2wasm-ir` dependency to `crates/backend-wasm/Cargo.toml`
- [ ] Update all imports in `crates/cli/src/` from `crate::ir::` to `ts2wasm_ir::`
- [ ] Update all imports in `crates/backend-wasm/src/` from `crate::ir::` to `ts2wasm_ir::`
- [ ] Update `crates/cli/src/lib.rs` to remove `mod ir;`

Out of scope:

- Other crate migrations (backend, frontend)
- Changing IR contracts or semantics

## Affected paths

Expected (after migration):

- crates/ir/src/builtin.rs
- crates/ir/src/builtin_resolved.rs
- crates/ir/src/builtin_resolver.rs
- crates/ir/src/lowered.rs
- crates/ir/src/mod.rs
- crates/cli/Cargo.toml
- crates/backend-wasm/Cargo.toml
- crates/cli/src/lib.rs
- crates/cli/src/backend/*.rs

Do not touch:

- `crates/shared/`
- `crates/runtime-abi/`
- `docs/`
- `scripts/`
- `fixtures/`

## Acceptance criteria

- [ ] `crates/ir/src/` contains full IR implementation
- [ ] `crates/cli/src/ir/` directory is removed
- [ ] `cargo check` passes
- [ ] `cargo nextest run` passes (all 205 tests)
- [ ] No behavior changes in IR semantics

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

- [ ] created: `issues/open/026-migrate-backend-module-to-backend-wasm-crate.md`
- [ ] created: `issues/open/027-migrate-frontend-code-to-frontend-crate.md`

## Notes

IR depends on `ts2wasm-shared`, which is already in `crates/ir/Cargo.toml`. Backend-wasm also needs to depend on `ts2wasm-ir`.

Import replacement pattern:
- `crate::ir::builtin` → `ts2wasm_ir::builtin`
- `crate::ir::builtin_resolved` → `ts2wasm_ir::builtin_resolved`
- `crate::ir::builtin_resolver` → `ts2wasm_ir::builtin_resolver`
- `crate::ir::lowered` → `ts2wasm_ir::lowered`

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
