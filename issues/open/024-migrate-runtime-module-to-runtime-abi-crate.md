---
id: 024
title: "Migrate runtime module to runtime-abi crate"
type: refactor
area: abi
class: implementation-ready
priority: P1
depends_on: []
blocks: [025, 026, 027]
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Move `crates/cli/src/runtime/` (value/layout/consts) to `crates/runtime-abi/src/` and update imports in cli.

## Problem

Runtime ABI code is currently in `crates/cli/src/runtime/`, violating the target crate structure defined in AGENTS.md. This mixes runtime ABI concerns with CLI orchestration.

## Desired final state

- `crates/runtime-abi/src/` contains value.rs, layout.rs, consts.rs with full implementation
- `crates/cli/src/runtime/` is removed
- `crates/cli` depends on `ts2wasm-runtime-abi`
- All imports in cli updated to use `ts2wasm_runtime_abi::`

## Scope

In scope:

- [ ] Move `crates/cli/src/runtime/value.rs` → `crates/runtime-abi/src/value.rs`
- [ ] Move `crates/cli/src/runtime/layout.rs` → `crates/runtime-abi/src/layout.rs`
- [ ] Move `crates/cli/src/runtime/consts.rs` → `crates/runtime-abi/src/consts.rs`
- [ ] Remove `crates/cli/src/runtime/mod.rs`
- [ ] Add `ts2wasm-runtime-abi` dependency to `crates/cli/Cargo.toml`
- [ ] Update all imports in `crates/cli/src/` from `crate::runtime::` to `ts2wasm_runtime_abi::`
- [ ] Update `crates/cli/src/lib.rs` to remove `mod runtime;`

Out of scope:

- Other crate migrations (ir, backend, frontend)
- Changing runtime ABI contracts or semantics

## Affected paths

Expected (after migration):

- crates/runtime-abi/src/value.rs
- crates/runtime-abi/src/layout.rs
- crates/runtime-abi/src/consts.rs
- crates/cli/Cargo.toml
- crates/cli/src/lib.rs
- crates/cli/src/backend/*.rs

Do not touch:

- `crates/shared/`
- `docs/`
- `scripts/`
- `fixtures/`

## Acceptance criteria

- [ ] `crates/runtime-abi/src/` contains full runtime ABI implementation
- [ ] `crates/cli/src/runtime/` directory is removed
- [ ] `cargo check` passes
- [ ] `cargo nextest run` passes (all 205 tests)
- [ ] No behavior changes in runtime semantics

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

- [ ] created: `issues/open/025-migrate-ir-module-to-ir-crate.md`
- [ ] created: `issues/open/026-migrate-backend-module-to-backend-wasm-crate.md`
- [ ] created: `issues/open/027-migrate-frontend-code-to-frontend-crate.md`

## Notes

Runtime ABI has no external dependencies other than std, so this is the simplest migration. Start here to establish the migration pattern.

Import replacement pattern:
- `crate::runtime::ValueTag` → `ts2wasm_runtime_abi::ValueTag`
- `crate::runtime::Layout` → `ts2wasm_runtime_abi::Layout`
- `crate::runtime::RuntimeConst` → `ts2wasm_runtime_abi::RuntimeConst`
- `crate::runtime::RuntimeString` → `ts2wasm_runtime_abi::RuntimeString`

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
