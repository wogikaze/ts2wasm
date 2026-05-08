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
completed: 2026-04-26
---

## Summary

Implement `break` and `continue` statements for loop control.

## Problem

The `break` and `continue` statements are not implemented. They are essential for loop control flow.

## Desired final state

`break` exits the current loop, `continue` skips to the next iteration.

## Scope

In scope:

- [x] Add `break` to lexer/parser
- [x] Add `continue` to lexer/parser
- [x] Implement loop exit for break
- [x] Implement loop iteration skip for continue
- [x] Add fixtures for break/continue behavior

Out of scope:

- Labeled break/continue (P2)

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/backend-wasm/src/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] `break` statement parses correctly
- [x] `continue` statement parses correctly
- [x] Both statements work in loops
- [x] Fixtures cover break/continue behavior
- [x] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/control-flow-and-exceptions/while.ts -o /tmp/test.wasm
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

- [x] Implement labeled break/continue (P2; tracked by `issues/open/209-implement-labeled-break-continue.md`)

## Notes

Break and continue statements are already fully implemented.

## Completion evidence

**Validation results:**

```text
command: cargo nextest run -E 'test(break|continue)'
result: 5 tests passed (while_break_build_smoke, while_continue_build_smoke)
date: 2026-04-26
```

**Implementation:**
- Break and continue statement support was already implemented in the codebase:
  - Lexer: Token::Break and Token::Continue already defined in crates/frontend/src/lexer.rs
  - Parser: break_statement() and continue_statement() functions already exist in crates/cli/src/lib.rs
  - AST: Stmt::Break and Stmt::Continue variants already exist
  - Lowering: LoweredStmt::Break and LoweredStmt::Continue already implemented in crates/ir/src/lowered.rs
  - Emitter: Break/continue statement emission already implemented in crates/backend-wasm/src/stmt_emit.rs
  - Fixtures: fixtures/control-flow-and-exceptions/while.ts already includes break/continue
  - Tests: 5 break/continue related tests already pass

**Remaining risks:**
- Labeled break/continue is implemented and tracked by `issues/open/209-implement-labeled-break-continue.md`

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/035-implement-break-continue.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
