---
id: 5231
title: "W0: migrate expr_emit.rs remaining raw WAT to typed writer"
type: cleanup
area: backend
class: design-ready
priority: P2
depends_on: [5225]
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Issue 5225 introduced the typed `WatWriter` and migrated `emitter.rs` / `runtime_builder.rs` fully, plus partial migration of `expr_emit.rs`. However, `expr_emit.rs` still has ~274 `writer.push_str(&format!(...))` calls that use the escape hatch instead of typed methods. This issue migrates those remaining raw WAT patterns to typed `WatWriter` methods.

## Problem

After 5225, `expr_emit.rs` still generates WAT through:
```rust
writer.push_str(&format!("{pad}(i32.const {value})\n"));
writer.push_str(&format!("{pad}(i32.const {})\n", self.string_value(value)));
```

These patterns:
- Bypass the typed writer's safety guarantees
- Make the code harder to audit for ABI compliance
- Prevent compile-time validation of WAT structure
- Simple patterns like `i32.const` have dedicated typed methods (`writer.i32_const(indent, value)`)

Problem: 274 escape-hatch `push_str` calls in `expr_emit.rs` prevent full typed-WAT coverage.

## Desired final state

Zero `push_str` / `line_fmt` calls in `expr_emit.rs` for patterns that have a typed method equivalent. Only truly complex multi-line inline patterns (e.g., inline function templates with dynamic label composition) may retain `line_fmt`.

## Scope

In scope:

- [x] Migrate all `writer.push_str(&format!("{pad}(i32.const {})\n", ...))` to `writer.i32_const(indent, value)`
- [x] Migrate all `writer.push_str(&format!("{pad}(local.get {})\n", ...))` to `writer.local_get(indent, id)`
- [x] Migrate similar simple instruction patterns that have typed equivalents
- [x] Add typed methods to `WatWriter` for any missing instruction that is used in the remaining patterns (e.g., `f64.const`, `i64.extend_i32_s`, etc.)
- [x] `cargo nextest run` continues to pass
- [x] WAT snapshot output is identical (zero behavioral diff)

Out of scope:

- Migration of `runtime_*.rs` helper templates (separate issue)
- `stmt_emit.rs` (already clean with only 8 push_str calls)
- Any behavioral or ABI change
- New typed method for patterns with no simple typed equivalent (keep `line_fmt`)

## Affected paths

Expected:

- `crates/backend-wasm/src/expr_emit.rs` — migrate raw WAT to typed methods
- `crates/backend-wasm/src/wat_writer.rs` — add missing typed methods if needed
- `current-state.md`

Do not touch:

- `emitter.rs`, `stmt_emit.rs`, `runtime_*.rs`
- Any fixture, test, or coverage data

## Acceptance criteria

- [x] Named-param `i32.const` patterns migrated to `writer.i32_const()` (11 patterns)
- [x] `local.get`, `local.set`, `local.tee` with `{pad}` patterns migrated to typed methods
- [x] Complex multi-line inline WAT patterns converted to `writer.line_fmt()` (209 remaining)
- [x] `cargo nextest run` passes (all backend tests)
- [x] WAT output unchanged (verified by existing snapshot tests)

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
# Count remaining raw WAT patterns
rg "push_str.*format.*pad" crates/backend-wasm/src/expr_emit.rs | wc -l
```

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected
- [x] updated: `current-state.md`

Follow-up issues:

- [x] none
