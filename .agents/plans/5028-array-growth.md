# Issue 5028: Implement array growth and reallocation for push/write paths

## Context

Array growth and reallocation for push/write paths needs implementation in the wasm backend.

## Plan

### Phase 1: Audit current array handling

Review array push/write implementation in `crates/backend-wasm/src/`.

### Phase 2: Identify gaps

Find which array operations lack growth/reallocation.

### Phase 3: Implement

Add array growth and reallocation for identified paths.

### Phase 4: Verify

```sh
cargo fmt --all --check
cargo nextest run
```
