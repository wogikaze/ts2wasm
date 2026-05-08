---
id: 025
title: "Migrate ir module to ir crate"
type: refactor
area: ir
class: implementation-ready
priority: P1
depends_on: [024]
blocks: [026]
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Move IR builtin/builtin_resolved/builtin_resolver/lowered modules from cli to ir crate and update imports in cli and backend-wasm.

## Problem

IR code is currently in cli/src/ir/, violating the target crate structure defined in AGENTS.md. This mixes IR concerns with CLI orchestration.

## Desired final state

- crates/ir/src/ contains builtin.rs, builtin_resolved.rs, builtin_resolver.rs, lowered.rs with full implementation
- cli/src/ir/ is removed
- crates/cli and crates/backend-wasm depend on ts2wasm-ir
- All imports updated to use ts2wasm_ir

## Scope

In scope:

- [x] Move IR builtin/builtin_resolved/builtin_resolver/lowered modules from cli to ir crate
- [x] Remove ir module from cli
- [x] Add ts2wasm-ir dependency to cli and backend-wasm
- [x] Update all imports in cli and backend-wasm to use ts2wasm_ir
- [x] Update cli lib.rs to remove mod ir

Out of scope:

- Other crate migrations (backend, frontend)
- Changing IR contracts or semantics

## Affected paths

Expected (after migration):

- crates/ir/src/builtin.rs (DONE)
- crates/ir/src/builtin_resolved.rs (DONE)
- crates/ir/src/builtin_resolver.rs (DONE)
- crates/ir/src/lowered.rs (DONE)
- crates/ir/src/mod.rs (DONE)
- crates/cli/Cargo.toml (DONE)
- crates/backend-wasm/Cargo.toml (DONE)
- crates/cli/src/lib.rs (DONE)
- crates/cli/src/backend/*.rs (DONE)

Do not touch:

- crates/shared/
- crates/runtime-abi/
- docs/

## Acceptance criteria

- [x] crates/ir/src/ contains full IR implementation
- [x] crates/cli/src/ir/ directory is removed
- [x] cargo check passes
- [x] cargo nextest run passes (all 186 tests)
- [x] No behavior changes in IR semantics

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

- [x] created: issues/open/026-migrate-backend-module-to-backend-wasm-crate.md
- [x] created: issues/open/027-migrate-frontend-code-to-frontend-crate.md

## Notes

IR depends on ts2wasm-shared, which is already in crates/ir/Cargo.toml. Backend-wasm also needs to depend on ts2wasm-ir.

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

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/025-migrate-ir-module-to-ir-crate.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
