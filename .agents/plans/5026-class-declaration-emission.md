# Issue 5026: Implement real class declaration emission

## Context

Real class declaration emission needs implementation in the wasm backend.

## Plan

### Phase 1: Audit current class handling

Review class declaration handling in `crates/backend-wasm/src/`.

### Phase 2: Identify gaps

Find what's missing for real class declaration emission.

### Phase 3: Implement

Add real class declaration emission for identified patterns.

### Phase 4: Verify

```sh
cargo fmt --all --check
cargo nextest run
```
