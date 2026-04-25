---
id: 033
title: "Implement switch statement"
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

Implement the `switch` / `case` statement for multi-way branching.

## Problem

The `switch` statement is not implemented. It is a common control flow structure for multi-way branching.

## Desired final state

`switch (expr) { case v1: ... case v2: ... default: ... }` executes the matching case block.

## Scope

In scope:

- [ ] Add `switch` / `case` / `default` to lexer/parser
- [ ] Lower switch statement to conditional branches
- [ ] Implement fall-through behavior
- [ ] Add fixtures for switch statement behavior

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

- [ ] `switch` statement parses correctly
- [ ] `switch` executes matching case correctly
- [ ] `default` case works when no match
- [ ] Fixtures cover switch statement behavior
- [ ] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/switch-test.ts -o /tmp/test.wasm
iwasm /tmp/test.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Switch can be lowered to a series of if-else statements or a jump table.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
