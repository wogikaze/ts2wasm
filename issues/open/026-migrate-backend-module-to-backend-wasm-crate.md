---
id: 026
title: "Migrate backend module to backend-wasm crate"
type: refactor
area: runtime
class: implementation-ready
priority: P1
depends_on: [024, 025, 028]
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Move `crates/cli/src/backend/` (capability_manifest/emitter/expr_emit/runtime_builder/runtime_fn/runtime_link_plan/stmt_emit/string_intern) to `crates/backend-wasm/src/` and update imports in cli.

## Problem

Backend code is currently in `crates/cli/src/backend/`, violating the target crate structure defined in AGENTS.md. This mixes WASM emission concerns with CLI orchestration.

## Desired final state

- `crates/backend-wasm/src/` contains full backend implementation
- `crates/cli/src/backend/` is removed
- `crates/cli` depends on `ts2wasm-backend-wasm`
- All imports in cli updated to use `ts2wasm_backend_wasm::`

## Scope

In scope:

- [ ] Move `crates/cli/src/backend/capability_manifest.rs` → `crates/backend-wasm/src/capability_manifest.rs`
- [ ] Move `crates/cli/src/backend/emitter.rs` → `crates/backend-wasm/src/emitter.rs`
- [ ] Move `crates/cli/src/backend/expr_emit.rs` → `crates/backend-wasm/src/expr_emit.rs`
- [ ] Move `crates/cli/src/backend/runtime_builder.rs` → `crates/backend-wasm/src/runtime_builder.rs`
- [ ] Move `crates/cli/src/backend/runtime_fn.rs` → `crates/backend-wasm/src/runtime_fn.rs`
- [ ] Move `crates/cli/src/backend/runtime_link_plan.rs` → `crates/backend-wasm/src/runtime_link_plan.rs`
- [ ] Move `crates/cli/src/backend/stmt_emit.rs` → `crates/backend-wasm/src/stmt_emit.rs`
- [ ] Move `crates/cli/src/backend/string_intern.rs` → `crates/backend-wasm/src/string_intern.rs`
- [ ] Remove `crates/cli/src/backend/mod.rs`
- [ ] Add `ts2wasm-backend-wasm` dependency to `crates/cli/Cargo.toml`
- [ ] Update all imports in `crates/cli/src/` from `crate::backend::` to `ts2wasm_backend_wasm::`
- [ ] Update `crates/cli/src/lib.rs` to remove `mod backend;`

Out of scope:

- Other crate migrations (frontend)
- Changing backend contracts or semantics

## Affected paths

Expected:

- `crates/backend-wasm/src/capability_manifest.rs`
- `crates/backend-wasm/src/emitter.rs`
- `crates/backend-wasm/src/expr_emit.rs`
- `crates/backend-wasm/src/runtime_builder.rs`
- `crates/backend-wasm/src/runtime_fn.rs`
- `crates/backend-wasm/src/runtime_link_plan.rs`
- `crates/backend-wasm/src/stmt_emit.rs`
- `crates/backend-wasm/src/string_intern.rs`
- `crates/cli/Cargo.toml`
- `crates/cli/src/lib.rs`

Do not touch:

- `crates/shared/`
- `crates/runtime-abi/`
- `crates/ir/`
- `docs/`
- `scripts/`
- `fixtures/`

## Acceptance criteria

- [ ] `crates/backend-wasm/src/` contains full backend implementation
- [ ] `crates/cli/src/backend/` directory is removed
- [ ] `cargo check` passes
- [ ] `cargo nextest run` passes (all 205 tests)
- [ ] No behavior changes in backend semantics

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

- [ ] created: `issues/open/027-migrate-frontend-code-to-frontend-crate.md`

## Notes

Backend depends on `ts2wasm-shared`, `ts2wasm-ir`, and `ts2wasm-runtime-abi`. These dependencies are already in `crates/backend-wasm/Cargo.toml`.

Import replacement pattern:
- `crate::backend::capability_manifest` → `ts2wasm_backend_wasm::capability_manifest`
- `crate::backend::emitter` → `ts2wasm_backend_wasm::emitter`
- `crate::backend::expr_emit` → `ts2wasm_backend_wasm::expr_emit`
- `crate::backend::runtime_builder` → `ts2wasm_backend_wasm::runtime_builder`
- `crate::backend::runtime_fn` → `ts2wasm_backend_wasm::runtime_fn`
- `crate::backend::runtime_link_plan` → `ts2wasm_backend_wasm::runtime_link_plan`
- `crate::backend::stmt_emit` → `ts2wasm_backend_wasm::stmt_emit`
- `crate::backend::string_intern` → `ts2wasm_backend_wasm::string_intern`

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
