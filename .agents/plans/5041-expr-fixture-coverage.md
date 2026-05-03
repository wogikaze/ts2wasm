# Issue 5041: Complete Expr AST fixture coverage

## Context

All `Expr` variants need parse → AST snapshot fixtures to surface syntax coverage gaps.

## Plan

### Phase 1: Audit Expr enum variants

List all Expr variants in `crates/frontend/src/`.

### Phase 2: Add fixture tests

For each variant without a fixture, add a `.ts` fixture and node-diff assertion.

### Phase 3: Verify

```sh
cargo fmt --all --check
cargo nextest run
```
