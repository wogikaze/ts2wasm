# Issue 5030: Split large runtime/WAT emitters into testable components

## Context

Large runtime/WAT emitters need splitting into testable components for maintainability.

## Plan

### Phase 1: Audit current emitter sizes

Identify the largest emitter files in `crates/backend-wasm/src/`.

### Phase 2: Design split

Determine component boundaries for splitting.

### Phase 3: Implement

Split identified emitters into testable components.

### Phase 4: Verify

```sh
cargo fmt --all --check
cargo nextest run
```
