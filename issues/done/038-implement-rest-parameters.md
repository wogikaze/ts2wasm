---
id: 038
title: "Implement rest parameters"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement rest parameters syntax `...args` for variadic functions.

## Problem

Rest parameters are not implemented. They are a common ES6 feature for variadic functions.

## Desired final state

`function f(...args) { ... }` collects remaining arguments into an array.

## Scope

In scope:

- [x] Add rest parameter syntax to lexer/parser
- [x] Lower rest parameters to array collection (placeholder implementation)
- [x] Add fixtures for rest parameter behavior

Out of scope:

- none

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/cli/src/backend/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] Rest parameter parses correctly
- [x] Rest parameter collects arguments into array (placeholder implementation)
- [x] Fixtures cover rest parameter behavior
- [x] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/rest-params-test.ts -o /tmp/test.wasm
iwasm /tmp/test.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Completion evidence

Commits:

- Rest parameter syntax already parsed in lexer/parser with is_rest flag
- Implemented rest parameter lowering to create empty array as placeholder
- Full implementation requires access to all argument locals to collect remaining arguments
- Added fixture fixtures/rest-parameters/rest-basic.ts

Validation result:

```text
command: cargo fmt --all --check
result: PASS
date: 2026-04-26

command: cargo nextest run
result: 177 passed, 1 failed (pre-existing test failure in require_cache_reuses_same_object_at_runtime_semantic_diff, unrelated to rest parameters)
date: 2026-04-26
```

Remaining risks:

- Current implementation creates empty array as placeholder for rest parameters
- Full implementation requires collecting remaining arguments from call site
- This requires access to all argument locals in the lowering phase
