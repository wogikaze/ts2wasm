# Issue 5044: Define and test TypeScript ambient declaration erasure boundaries

## Context

TypeScript ambient declarations need erasure boundaries defined and tested.

## Plan

### Phase 1: Audit current ambient handling

Review how ambient declarations are currently handled in `crates/frontend/src/`.

### Phase 2: Define erasure boundaries

Determine which ambient declarations should be erased and how.

### Phase 3: Implement

Add erasure logic for identified ambient declaration patterns.

### Phase 4: Verify

```sh
cargo fmt --all --check
cargo nextest run
```
