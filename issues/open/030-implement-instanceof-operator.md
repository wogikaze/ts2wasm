---
id: 030
title: "Implement instanceof operator"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P1
depends_on: [016]
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement the `instanceof` operator to check if an object is an instance of a constructor.

## Problem

The `instanceof` operator is not implemented. It is used to check the prototype chain of an object against a constructor function.

## Desired final state

`obj instanceof Constructor` returns true if `Constructor.prototype` is in the prototype chain of `obj`.

## Scope

In scope:

- [x] Add `instanceof` to lexer/parser
- [x] Lower `instanceof` expression to runtime call
- [x] Implement prototype chain lookup (completed by follow-up: `issues/done/207-complete-instanceof-prototype-chain-semantics.md`)
- [x] Add fixtures for instanceof behavior

Out of scope:

- Custom `@@hasInstance` behavior (P2)

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/backend-wasm/src/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] `instanceof` expression parses correctly
- [x] `instanceof` checks prototype chain correctly (completed by follow-up: `issues/done/207-complete-instanceof-prototype-chain-semantics.md`)
- [x] Fixtures cover instanceof behavior
- [x] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/instanceof-test.ts -o /tmp/test.wasm
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

- [x] Instanceof prototype chain lookup completed by `issues/done/207-complete-instanceof-prototype-chain-semantics.md`

## Notes

Basic instanceof operator syntax was implemented here. Full prototype chain traversal semantics were completed by `issues/done/207-complete-instanceof-prototype-chain-semantics.md`.

## Completion evidence

Commits:

- `7ca3d65` Add instanceof operator support

Validation result:

```text
command: cargo nextest run
result: 207 tests passed, 4 skipped
date: 2026-04-27
```

Remaining risks:

- Full prototype-chain semantics were completed by `issues/done/207-complete-instanceof-prototype-chain-semantics.md`

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/030-implement-instanceof-operator.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
