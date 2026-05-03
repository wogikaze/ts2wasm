# Issue 5052: Validate runtime memory map for overlap and headroom

## Context

Runtime memory map needs validation for overlap and headroom.

## Plan

### Phase 1: Audit current memory map

Review the runtime memory map in `crates/runtime-abi/src/`.

### Phase 2: Check for overlap

Check if any memory regions overlap.

### Phase 3: Check headroom

Check if memory regions have adequate headroom.

### Phase 4: Verify

```sh
cargo fmt --all --check
cargo nextest run
```
