# Issue 5050: Implement iterator protocol lowering for spread and for-of

## Context

Spread and for-of need iterator protocol IR lowering.

## Plan

### Phase 1: Audit current iterator handling

Review existing iterator-related code in `crates/ir/src/lowered/`.

### Phase 2: Identify gaps

Find which iterator protocol operations lack lowering.

### Phase 3: Implement

Add lowering for identified iterator protocol operations.

### Phase 4: Verify

```sh
cargo fmt --all --check
cargo nextest run
```
