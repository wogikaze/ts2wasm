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

Move runtime value/layout/consts modules from cli to runtime-abi crate and update imports in cli.

## Problem

Runtime ABI code is currently in cli/src/runtime/, violating the target crate structure defined in AGENTS.md. This mixes runtime ABI concerns with CLI orchestration.

## Desired final state

- crates/runtime-abi/src/ contains value.rs, layout.rs, consts.rs with full implementation
- cli/src/runtime/ is removed
- crates/cli depends on ts2wasm-runtime-abi
- All imports in cli updated to use ts2wasm_runtime_abi

## Scope

In scope:

- [x] Move runtime value/layout/consts modules from cli to runtime-abi crate
- [x] Remove runtime module from cli
- [x] Add ts2wasm-runtime-abi dependency to cli
- [x] Update all imports in cli to use ts2wasm_runtime_abi
- [x] Update cli lib.rs to remove mod runtime

Out of scope:

- Changing runtime ABI contracts or semantics

## Affected paths

Expected (after migration):

- crates/runtime-abi/src/value.rs (DONE)
- crates/runtime-abi/src/layout.rs (DONE)
- crates/runtime-abi/src/consts.rs (DONE)
- crates/cli/Cargo.toml (DONE)
- crates/cli/src/lib.rs (DONE)
- crates/cli/src/backend/*.rs (DONE)

Do not touch:
- crates/shared/
- docs/

## Acceptance criteria

- [x]  contains full runtime ABI implementation
- [x]  directory is removed
- [x] `cargo check` passes
- [x] `cargo nextest run` passes (all 186 tests)
- [x] No behavior changes in runtime semantics

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

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] created: issues/open/025-migrate-ir-module-to-ir-crate.md
- [x] created: issues/open/026-migrate-backend-module-to-backend-wasm-crate.md
- [x] created: issues/open/027-migrate-frontend-code-to-frontend-crate.md

## Notes

Runtime ABI has no external dependencies other than std, so this is the simplest migration. Start here to establish the migration pattern.

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

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/024-migrate-runtime-module-to-runtime-abi-crate.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
