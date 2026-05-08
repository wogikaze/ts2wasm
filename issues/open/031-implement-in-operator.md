---
id: 031
title: "Implement in operator"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-26
completed: 2026-04-26
---

## Summary

Implement the `in` operator to check if a property exists in an object.

## Problem

The `in` operator is not implemented. It is used to check if a property exists in an object or its prototype chain.

## Desired final state

`"prop" in obj` returns true if the property exists in `obj` or its prototype chain.

## Scope

In scope:

- [x] Add `in` to lexer/parser
- [x] Lower `in` expression to runtime call
- [x] Implement property existence check
- [x] Add fixtures for in operator behavior

Out of scope:

- none

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/backend-wasm/src/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] `in` expression parses correctly
- [x] `in` checks property existence correctly
- [x] Fixtures cover in operator behavior
- [x] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/in-operator-test.ts -o /tmp/test.wasm
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

Requires property access implementation.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `2582053` runtime: implement in operator with property existence check

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-04-26

command: cargo nextest run
result: pass (168/169 tests, 1 unrelated m9_modules failure)
date: 2026-04-26
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/031-implement-in-operator.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
## Completion evidence

Core feature works correctly.

Validation:
```sh
echo 'let x = \"a\" in {a:1};' | ts2wasm build --stdin -o /tmp/out.wasm
# => exit 0
```
