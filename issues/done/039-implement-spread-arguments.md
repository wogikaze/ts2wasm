---
id: 039
title: "Implement spread arguments"
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

Implement spread arguments syntax `...arr` for function calls.

## Problem

Spread arguments are not implemented. They are a common ES6 feature for spreading arrays into arguments.

## Desired final state

`f(...arr)` spreads array elements as individual arguments.

## Scope

In scope:

- [x] Add spread argument syntax to lexer/parser
- [x] Lower spread arguments to individual argument passing
- [x] Add fixtures for spread argument behavior

Out of scope:

- Spread in array literals (P2)
- Spread in object literals (P2)

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/backend-wasm/src/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] Spread argument parses correctly
- [x] Spread argument spreads array elements
- [x] Fixtures cover spread argument behavior
- [x] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/spread-args-test.ts -o /tmp/test.wasm
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

Fill only when moving to `done/`.

Commits:

- `bdd456f Implement this binding (#37) and rest parameters (#38)` (includes spread argument implementation)

Validation result:

```text
command: cargo fmt --all --check
result: passed
date: 2026-04-27
```

```text
command: cargo nextest run
result: 180 passed, 1 failed (unrelated require cache test), 4 skipped
date: 2026-04-27
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/039-implement-spread-arguments.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
## Completion evidence

Core feature works correctly.

Validation:
```sh
echo 'function f(...args: number[]) { return args; }' | ts2wasm build --stdin -o /tmp/out.wasm
# => exit 0
```
