---
id: 046
title: "Implement extends inheritance"
type: feature
area: runtime/semantics
class: done
priority: P1
depends_on: [045]
blocks: []
created: 2026-04-26
updated: 2026-04-28
completed: 2026-04-28
---

## Summary

Implement class inheritance with `extends` keyword.

## Problem

Class inheritance is not implemented. It is essential for object-oriented programming.

## Desired final state

`class Child extends Parent { ... }` correctly sets up prototype chain.

## Scope

In scope:

- [x] Add extends syntax to lexer/parser
- [x] Implement prototype chain setup
- [x] Add fixtures for inheritance behavior

Out of scope:

- super (047)

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/backend-wasm/src/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] extends parses correctly
- [x] Prototype chain is set up correctly
- [x] Fixtures cover inheritance behavior
- [x] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/extends-test.ts -o /tmp/test.wasm
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

- [x] 047 (super)

## Notes

Requires prototype chain implementation (016).

## Completion evidence

Commits:

- Close commit recorded by branch HEAD / parent event.

Validation result:

```text
command: node fixtures/classes-and-inheritance/class-extends.ts
result: pass; stdout `7`, proving the fixture's Node reference behavior
date: 2026-04-28

command: cargo run -p ts2wasm-cli -- build fixtures/classes-and-inheritance/class-extends.ts -o /tmp/ts2wasm-046-class-extends.wasm && iwasm /tmp/ts2wasm-046-class-extends.wasm
result: pass; iwasm stdout `7`, matching Node for inherited method lookup through `class Child extends Base`
date: 2026-04-28

command: cargo nextest run -p ts2wasm-cli class_extends
result: pass; 2 passed, including Node/iwasm differential for fixtures/classes-and-inheritance/class-extends.ts
date: 2026-04-28

command: cargo nextest run -p ts2wasm-cli prototype
result: pass; 1 passed, validating prototype-chain differential coverage
date: 2026-04-28

command: cargo nextest run -p ts2wasm-cli class
result: pass; 14 passed, including class extends build smoke and Node/iwasm differential coverage
date: 2026-04-28

command: cargo fmt --all --check
result: pass
date: 2026-04-28

command: scripts/manager check-agent-state
result: pass
date: 2026-04-28

command: scripts/manager update-issue-index --check
result: pass; issues/index.md up to date after moving 046 to done
date: 2026-04-28

command: scripts/manager check-issue-index
result: pass
date: 2026-04-28

command: scripts/manager check-issue-health
result: pass
date: 2026-04-28

command: scripts/manager check-repo-smoke
result: pass
date: 2026-04-28

command: cargo nextest run
result: pass; 253 passed, 4 skipped
date: 2026-04-28
```

Remaining risks:

- none
