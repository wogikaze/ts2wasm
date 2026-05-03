# Issue 5049: Complete destructuring, rest, and default binding lowering

## Context

Destructuring, rest, and default binding patterns need IR lowering.

## Plan

### Phase 1: Audit current lowering

Review existing destructuring/rest/default binding code in `crates/ir/src/lowered/`.

### Phase 2: Identify gaps

Find which patterns are missing lowering.

### Phase 3: Implement

Add lowering for identified patterns.

### Phase 4: Verify

```sh
cargo fmt --all --check
cargo nextest run
```
