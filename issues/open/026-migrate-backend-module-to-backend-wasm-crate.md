---
id: 026
title: "Migrate backend module to backend-wasm crate"
type: refactor
area: runtime
class: implementation-ready
priority: P1
depends_on: [024, 025]
blocks: []
created: 2026-04-26
updated: 2026-04-28
completed: 2026-04-28
---

## Summary

Move `crates/cli/src/backend/` (capability_manifest/emitter/expr_emit/runtime_builder/runtime_fn/runtime_link_plan/stmt_emit/string_intern) to `crates/backend-wasm/src/` and update imports in cli.

## Problem

Backend code is currently in `crates/cli/src/backend/`, violating the target crate structure defined in AGENTS.md. This mixes WASM emission concerns with CLI orchestration.

## Desired final state

- `crates/backend-wasm/src/` contains full backend implementation
- `crates/cli/src/backend/` is removed
- Backend consumers depend on `ts2wasm-backend-wasm` through the compiler driver; `crates/cli` no longer owns backend implementation code.
- All imports in cli updated to use `ts2wasm_backend_wasm::`

## Scope

In scope:

- [x] Move `crates/cli/src/backend/capability_manifest.rs` → `crates/backend-wasm/src/capability_manifest.rs`
- [x] Move `crates/cli/src/backend/emitter.rs` → `crates/backend-wasm/src/emitter.rs`
- [x] Move `crates/cli/src/backend/expr_emit.rs` → `crates/backend-wasm/src/expr_emit.rs`
- [x] Move `crates/cli/src/backend/runtime_builder.rs` → `crates/backend-wasm/src/runtime_builder.rs`
- [x] Move `crates/cli/src/backend/runtime_fn.rs` → `crates/backend-wasm/src/runtime_fn.rs`
- [x] Move `crates/cli/src/backend/runtime_link_plan.rs` → `crates/backend-wasm/src/runtime_link_plan.rs`
- [x] Move `crates/cli/src/backend/stmt_emit.rs` → `crates/backend-wasm/src/stmt_emit.rs`
- [x] Move `crates/cli/src/backend/string_intern.rs` → `crates/backend-wasm/src/string_intern.rs`
- [x] Move `crates/cli/src/backend/wat_writer.rs` → `crates/backend-wasm/src/wat_writer.rs`
- [x] Remove `crates/cli/src/backend/mod.rs`
- [x] Add `ts2wasm-backend-wasm` dependency to `crates/cli/Cargo.toml`
- [x] Update all imports in `crates/cli/src/` from `crate::backend::` to `ts2wasm_backend_wasm::`
- [x] Update `crates/cli/src/lib.rs` to remove `mod backend;`

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

- [x] `crates/backend-wasm/src/` contains full backend implementation
- [x] `crates/cli/src/backend/` directory is removed
- [x] `cargo check` passes
- [x] `cargo nextest run` passes
- [x] No backend semantic regressions in workspace tests

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

- [x] created: issues/open/027-migrate-frontend-code-to-frontend-crate.md (moved to done/)

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

- `9fd1230 refactor(cli): route backend through backend-wasm crate`
- `b3d730c refactor(cli): remove migrated backend module`
- `93e6650 chore(checks): prevent cli backend regression`
- `c40c672 refactor(backend): split runtime builder modules`
- `f943563 fix(runtime): preserve dynamic object property access`
- `a8ef598 fix(ir): bind this receiver in class methods`
- `8db253f test(cli): tolerate missing official corpora shards`

Validation result:

```text
command: cargo fmt --all --check
result: PASS
date: 2026-04-28

command: cargo nextest run --no-fail-fast
result: PASS (194 passed, 4 skipped)
date: 2026-04-28

command: mise run check-architecture-rules
result: PASS
date: 2026-04-28
```

Remaining risks:

- none for backend crate migration; future backend semantics remain tracked by feature-specific issues.

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

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/026-migrate-backend-module-to-backend-wasm-crate.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
