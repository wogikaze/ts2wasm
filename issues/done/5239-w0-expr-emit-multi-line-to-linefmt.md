---
id: 5239
title: "W0: migrate remaining multi-line WAT to line_fmt in expr_emit.rs"
type: cleanup
area: backend
class: design-ready
priority: P2
depends_on: [5231]
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Issue 5231 migrated 11 simple single-line patterns to typed methods. This follow-up converts the ~209 remaining multi-line `writer.push_str(&format!(...))` patterns to `writer.line_fmt()`. These are complex inline WAT expressions (e.g., `i32.store` with `i32.add`, `local.set` with `call`, branching constructs) that cannot use simple typed methods but should use the `line_fmt` escape hatch instead of raw `push_str`.

## Problem

After 5231, ~209 `writer.push_str(&format!(...))` multi-line patterns remain in `expr_emit.rs`. These bypass the typed writer's safety guarantees for even the `line_fmt` path.

Example:
```rust
// Before:
writer.push_str(&format!(
    "{pad}(i32.store (local.get {}) (i32.const {}))\n",
    frame.heap_base_tmp(),
    prop_count,
));
// After:
writer.line_fmt(indent, format_args!(
    "(i32.store (local.get {}) (i32.const {}))",
    frame.heap_base_tmp(),
    prop_count,
));
```

## Desired final state

Zero `writer.push_str(&format!(...))` calls in `expr_emit.rs`. All complex multi-line patterns use `writer.line_fmt(indent, format_args!(...))`.

## Scope

In scope:

- [ ] Convert remaining ~209 multi-line `push_str(&format!(...))` blocks to `writer.line_fmt()`
- [ ] `cargo fmt --all --check` passes
- [ ] WAT output unchanged

Out of scope:

- Migration of `runtime_*.rs` helper templates
- `stmt_emit.rs` or `emitter.rs`
- Any behavioral or ABI change
- Typed method conversion (those need Wasm-level stack/type discipline)

## Affected paths

Expected:

- `crates/backend-wasm/src/expr_emit.rs`

## Acceptance criteria

- [ ] Zero `push_str(&format!(...))` calls in `expr_emit.rs`
- [ ] All converted patterns use `writer.line_fmt(indent, format_args!(...))`
- [ ] `cargo nextest run` passes
- [ ] WAT snapshot output unchanged

## Validation

```sh
cargo fmt --all --check
cargo nextest run
# Verify conversion count
grep -c "push_str.*format" crates/backend-wasm/src/expr_emit.rs
# Should be 0 for format patterns
```

## Notes

The conversion is purely mechanical: each `push_str(&format!(...))` block becomes `writer.line_fmt(indent, format_args!(...))`. The key challenge is handling multi-line format strings and multiple format arguments. A Python script may be needed to batch-convert, but each conversion must be verified by `cargo fmt --all --check`.
