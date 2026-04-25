---
id: 035
title: "Implement break and continue statements"
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

Implement `break` and `continue` statements for loop control.

## Problem

The `break` and `continue` statements are not implemented. They are essential for loop control flow.

## Desired final state

`break` exits the current loop, `continue` skips to the next iteration.

## Scope

In scope:

- [ ] Add `break` to lexer/parser
- [ ] Add `continue` to lexer/parser
- [ ] Implement loop exit for break
- [ ] Implement loop iteration skip for continue
- [ ] Add fixtures for break/continue behavior

Out of scope:

- Labeled break/continue (P2)

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/cli/src/backend/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] `break` statement parses correctly
- [ ] `continue` statement parses correctly
- [ ] Both statements work in loops
- [ ] Fixtures cover break/continue behavior
- [ ] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/break-continue-test.ts -o /tmp/test.wasm
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

Requires loop implementation (034).

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
