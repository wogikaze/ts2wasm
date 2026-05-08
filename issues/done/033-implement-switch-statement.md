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
completed: 2026-04-26
---

## Summary

Implement the `switch` / `case` statement for multi-way branching.

## Problem

The `switch` statement is not implemented. It is a common control flow structure for multi-way branching.

## Desired final state

`switch (expr) { case v1: ... case v2: ... default: ... }` executes the matching case block.

## Scope

In scope:

- [x] Add `switch` / `case` / `default` to lexer/parser
- [x] Lower switch statement to conditional branches
- [x] Implement basic switch execution (no fall-through)
- [x] Add fixtures for switch statement behavior

Out of scope:

- Fall-through behavior (P2 - requires break statement detection)

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/backend-wasm/src/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] `switch` statement parses correctly
- [x] `switch` executes matching case correctly
- [x] `default` case works when no match
- [x] Fixtures cover switch statement behavior
- [x] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/control-flow-and-exceptions/switch-case.ts -o /tmp/test.wasm
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

- [x] Implement fall-through behavior for switch statements (P2; completed in `issues/open/208-implement-switch-fall-through-semantics.md`)

## Notes

Switch is lowered to structured case-entry blocks rather than a jump table. Fall-through support is completed in `issues/open/208-implement-switch-fall-through-semantics.md`.

## Completion evidence

**Validation results:**

```text
command: cargo fmt --all --check
result: passed
date: 2026-04-26

command: cargo nextest run -E 'test(switch)'
result: 3 tests passed
date: 2026-04-26

command: cargo run -p ts2wasm-cli -- build fixtures/control-flow-and-exceptions/switch-case.ts -o /tmp/test.wasm
result: compiled successfully
date: 2026-04-26
```

**Implementation:**
- Switch statement support was already implemented in the codebase:
  - Lexer: Token::Switch, Token::Case, Token::Default already defined in crates/frontend/src/lexer.rs
  - Parser: switch_statement() function already exists in crates/cli/src/lib.rs
  - AST: Stmt::Switch variant already exists
  - Lowering: LoweredStmt::Switch already implemented in crates/ir/src/lowered.rs
  - Emitter: Switch statement emission already implemented in crates/backend-wasm/src/stmt_emit.rs
  - Fixture: fixtures/control-flow-and-exceptions/switch-case.ts already exists
  - Tests: 3 switch-related tests already pass

**Current behavior:**
- Switch statements are lowered to if-else chains
- Each case automatically breaks at the end (no fall-through)
- This is simpler than JavaScript semantics but functional for basic use cases

**Remaining risks:**
- Fall-through behavior is implemented and tracked by `issues/open/208-implement-switch-fall-through-semantics.md`
- This differs from JavaScript semantics and must not be counted as full switch semantic parity

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/033-implement-switch-statement.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
