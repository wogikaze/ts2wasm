# Plan: Fix pre-existing env_cells test failure

## Problem

The test `env_cells_are_tagged_array_payloads_for_gc_tracing` in
`crates/backend-wasm/src/lib.rs` has stale assertions:

1. `ENV_CELL_VALUE_OFFSET` changed from 4 to `Layout::ARRAY_HEADER_SIZE = 20`
   when the GC array header was expanded (header grew from 4 bytes to 20 bytes).
   The test still expects offset 4.

2. The env cell local index changed due to class infrastructure (receiver
   parameters, prototype allocation, etc.). The test hardcodes `local.get 0` for
   reads but the actual env cell is at `local.get 1`.

3. The allocation size is 24 bytes (`ARRAY_HEADER_SIZE=20 + slot=4`), not 8.

## Fix

Update the test assertions to match the current emitted WAT:

1. **Allocation size**: `(call $alloc_heap (i32.const 24))` instead of `.8)`
2. **Env cell read pattern**: Use a flexible check that matches any local
   index instead of hardcoding `local.get 0`.
3. **Env cell write pattern**: Same flexible approach for the write.
4. **Offset**: Use `i32.const 20` instead of `i32.const 4`.

A `wat_contains_env_cell_read()` and `wat_contains_env_cell_write()` helper
will check line-by-line for patterns with arbitrary local indices.

## Scope

Allowed files: `crates/backend-wasm/src/lib.rs` (test file)
