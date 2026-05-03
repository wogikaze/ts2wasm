# Issue 5045: Improve syntax error recovery and source spans

## Context

Syntax error recovery and source span accuracy need improvement.

## Plan

### Phase 1: Audit current error recovery

Review error recovery and source span handling in `crates/frontend/src/`.

### Phase 2: Identify gaps

Find patterns with poor error recovery or inaccurate spans.

### Phase 3: Implement

Improve error recovery and source spans for identified patterns.

### Phase 4: Verify

```sh
cargo fmt --all --check
cargo nextest run
```
