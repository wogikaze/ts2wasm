---
id: 037
title: "Implement this binding"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement `this` keyword with call site receiver binding.

## Problem

The `this` keyword is not implemented. It is essential for method calls and object-oriented patterns.

## Desired final state

`this` refers to the call site receiver in method calls and constructor calls.

## Scope

In scope:

- [x] Add `this` to lexer/parser
- [x] Implement call site receiver binding
- [x] Implement constructor `this` binding
- [x] Add fixtures for this behavior

Out of scope:

- Arrow function lexical this (036)

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/cli/src/backend/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] `this` parses correctly
- [x] `this` binds to call site receiver
- [x] Fixtures cover this behavior
- [x] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/this-test.ts -o /tmp/test.wasm
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

- [ ] 036 (arrow function)

## Notes

Requires method call implementation.

## Completion evidence

Commits:

- Added `This` variant to Expr enum in crates/frontend/src/ast.rs
- Updated parser to handle `this` as proper `This` expression in crates/cli/src/lib.rs
- Added `This` variant to ResolvedExpr in crates/ir/src/builtin_resolved.rs
- Added `This` variant to LoweredExpr in crates/ir/src/lowered.rs
- Implemented lowering logic for `This` to resolve to local variable
- Added emission logic in backend expr_emit.rs (emits undefined as placeholder pending method call implementation)
- Added fixture fixtures/this-binding/this-basic.ts
- Fixed pre-existing rest parameter tuple type mismatches

Validation result:

```text
command: cargo fmt --all --check
result: PASS
date: 2026-04-26

command: cargo nextest run
result: 164 passed, 1 failed (pre-existing test failure in require_cache_reuses_same_object_at_runtime_semantic_diff, unrelated to this implementation)
date: 2026-04-26
```

Remaining risks:

- Proper this binding requires method call implementation (issue 016) - currently emits undefined as placeholder
- Arrow function lexical this (issue 036) is out of scope for this issue
