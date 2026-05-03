# Issue 5029: Expand direct wasm binary emission beyond console.log string literal MVP

## Context

Direct wasm binary emission needs to be expanded beyond the console.log string literal MVP.

## Plan

### Phase 1: Audit current direct emission

Review direct wasm binary emission in `crates/backend-wasm/src/`.

### Phase 2: Identify gaps

Find which patterns could use direct binary emission.

### Phase 3: Implement

Expand direct wasm binary emission for identified patterns.

### Phase 4: Verify

```sh
cargo fmt --all --check
cargo nextest run
```
