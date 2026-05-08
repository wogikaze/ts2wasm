---
id: 5225
title: "W0: introduce typed WAT writer"
type: refactor
area: backend
class: design-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Introduce a typed `WatWriter` API to replace raw `push_str`/`format!` WAT emission. The backend currently generates WAT through 943 `push_str` calls across 7+ emitter files, making bracket mismatches, stack discipline errors, and ABI changes undetectable at compile time.

## Problem

`docs/12-coding-standard.md §1.2` identifies raw WAT string generation as a root cause of backend bugs:

```
wat.push_str("(local.get 0)\n");
wat.push_str(&format!("(i32.const {})\n", value));
```

These patterns are:
- Not type-safe: bracket mismatches found only at wat2wasm time
- Not stack-discipline-safe: operand stack errors invisible until validation
- Hard to refactor: spread across `emitter.rs` (1757 lines), `expr_emit.rs` (1935 lines), and 7+ runtime_*.rs files
- Hard to audit: no single place to verify ABI contract conformance

Problem: Raw WAT string concatenation is the primary emission path, preventing compile-time validation of WAT structure and runtime ABI contracts.

## Desired final state

A `WatWriter` interface (similar to `wasm-encoder` but WAT-specific) provides typed methods for WAT emission:

```rust
// instead of:
wat.push_str(&format!("(local.get {})\n", id));

// the writer provides:
writer.local_get(id);
```

Key types:
- `WatWriter`: struct with `write_*` methods for each WAT construct
- `WatImport`, `WatExport`, `WatFunction`, `WatInstr` types for structured construction
- Stack validation: methods like `if_`, `then`, `else_`, `end` enforce nesting at compile time

Exemptions:
- Runtime helper WAT templates (`runtime_*.rs`) may continue using raw WAT strings temporarily if they are pure function templates without complex nesting
- New runtime helpers must use typed writer or `wasm-encoder`

## Scope

In scope:

- [x] Define `WatWriter` struct with typed methods for common WAT constructs
- [x] Define `WatImport`, `WatExport`, `WatFunction` wrapper types
- [x] Migrate `emitter.rs` main module emission to typed writer
- [x] Migrate `expr_emit.rs` expression emission to typed writer
- [x] Update `runtime_builder.rs` to use typed writer
- [x] Keep existing snapshot/WAT test infrastructure passing
- [x] `docs/14-runtime-abi.md` update if ABI contracts changed
- [x] `current-state.md` update

Out of scope:

- Migration of runtime helper templates in `runtime_arrays.rs`, `runtime_core.rs`, etc. (separate issue)
- `wasm-encoder` binary path (separate issue: W0-004/5228)
- Any behavioral or ABI change

## Affected paths

Expected:

- `crates/backend-wasm/src/` (new `wat_writer.rs` + updates to `emitter.rs`, `expr_emit.rs`, `runtime_builder.rs`)
- `current-state.md`

Do not touch:

- `crates/frontend/`, `crates/ir/`, `crates/runtime-abi/`, `crates/compiler/`
- Runtime helper WAT templates in `runtime_*.rs` (those are separate issues)
- Any fixture, test, or coverage data

## Acceptance criteria

- [x] `WatWriter` module exists with typed methods covering: `module`, `import`, `export`, `memory`, `global`, `type`, `function`, `local.get/set/tee`, `i32/f32/f64/i64` const/arithmetic/compare, `block`/`loop`/`if`/`then`/`else`/`end`, `call`/`call_indirect`, `return`, `br`/`br_if`, `memory.size`/`grow`, `load`/`store`
- [x] `emitter.rs` uses `WatWriter` (no raw WAT `push_str` in main module emission)
- [x] `expr_emit.rs` uses `WatWriter` (no raw WAT `push_str` in expression emission)
- [x] `cargo test` and `cargo nextest run` all pass
- [x] WAT snapshot tests unchanged (zero behavioral diff)
- [x] `docs/12-coding-standard.md §13` (WAT generation) updated to reference typed writer

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
git diff --stat
```

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected
- [x] updated: `docs/12-coding-standard.md §13` reference typed writer

Current state:

- [x] not affected
- [x] updated: `current-state.md`

Follow-up issues:

- [x] none
- [x] created/updated: follow-up for runtime helper WAT migration

## Notes

The `WatWriter` should be a standalone module in `crates/backend-wasm/src/wat_writer.rs`. Keep it simple: a struct with an internal `String` buffer that provides type-safe wrappers, not a full WAT parser/generator. The goal is compile-time safety for nesting and ABI, not WAT formatting perfection.

Reference: `docs/12-coding-standard.md §1.2` and `§19.13` for the prohibition on raw WAT strings.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in . This issue has repo-local close evidence
(implementation commit or completion evidence).

Future-work tracking: none identified.
