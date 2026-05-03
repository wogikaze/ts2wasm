# Issue 5043: Split large lexer/parser files by grammar responsibility

## Context

Large lexer/parser files need splitting by grammar responsibility for maintainability.

## Plan

### Phase 1: Audit current file sizes

Identify the largest files in `crates/frontend/src/`.

### Phase 2: Design split

Determine grammar boundaries for splitting.

### Phase 3: Implement

Split identified files by grammar responsibility.

### Phase 4: Verify

```sh
cargo fmt --all --check
cargo nextest run
```
