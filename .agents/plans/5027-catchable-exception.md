# Issue 5027: Replace throw-as-return with catchable exception runtime

## Context

throw-as-return needs replacing with a catchable exception runtime in the wasm backend.

## Plan

### Phase 1: Audit current exception handling

Review throw-as-return implementation in `crates/backend-wasm/src/`.

### Phase 2: Design catchable exception runtime

Design the catchable exception runtime architecture.

### Phase 3: Implement

Replace throw-as-return with catchable exception runtime.

### Phase 4: Verify

```sh
cargo fmt --all --check
cargo nextest run
```
