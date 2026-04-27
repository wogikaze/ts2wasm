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

- [ ] updated: current-state.md (repo root)

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

## Verification attempt: 2026-04-28

Outcome: BLOCKED, issue remains open.

Migration structure was rechecked:

- `crates/backend-wasm/src/` contains the migrated backend modules.
- `crates/cli/src/backend/` is absent.
- `crates/cli/Cargo.toml` depends on `ts2wasm-backend-wasm`.
- `crates/cli/src/lib.rs` imports `ts2wasm_backend_wasm as backend` and no longer declares `mod backend`.

Validation evidence:

```text
cargo check
result: pass

cargo fmt --all --check
result: pass

scripts/manager check-agent-state
result: pass

scripts/manager update-issue-index --check
result: pass

scripts/manager check-issue-health
result: fail
reason: stale issue path references to removed crates/cli/src/backend in issue 206 and done issues 029-044/053; those files are outside this worker assignment scope.

cargo nextest run --no-fail-fast
result: fail
summary: 185 passed, 3 failed, 4 skipped out of 188 tests
failures:
- ts2wasm-cli::official_corpora official_corpora_smoke_gate_finds_reference_shards: missing reference/test262/test/language
- ts2wasm-cli::m2_node_diff m5_array_object_fixtures_match_node_output_under_iwasm: fixtures/arrays-objects/dynamic-property.ts stdout mismatch
- ts2wasm-cli::m2_node_diff m3_semantic_fixtures_match_node_output_under_iwasm: fixtures/core-semantics/prototype.ts build failed with UnsupportedSyntax method `value` requires an identifier receiver

scripts/manager check-repo-smoke
result: fail
reason: check_issue_health stale path failures listed above
```

Close decision: do not move to `done/` until required gates pass or the parent assigns a scope that permits stale issue-path cleanup and semantic/reference failure handling.

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
