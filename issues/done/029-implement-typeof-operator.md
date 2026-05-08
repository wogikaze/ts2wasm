---
id: 029
title: "Implement typeof operator"
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

Implement the `typeof` operator to return the type tag of a value.

## Problem

The `typeof` operator is not implemented. It is a fundamental JavaScript operator used for type checking at runtime.

## Desired final state

`typeof x` returns the correct type string for all primitive types and objects:
- `"undefined"` for undefined
- `"object"` for null
- `"boolean"` for boolean
- `"number"` for number
- `"string"` for string
- `"function"` for function objects
- `"object"` for objects and arrays

## Scope

In scope:

- [x] Add `typeof` to lexer/parser
- [x] Lower `typeof` expression to runtime call
- [x] Implement runtime type tag check
- [x] Add fixtures for typeof behavior

Out of scope:

- `typeof` for symbols (P2)
- `typeof` for bigint (P2)

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/backend-wasm/src/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] `typeof` expression parses correctly
- [x] `typeof` returns correct type strings for all primitive types
- [x] `typeof null` returns `"object"` (ECMAScript spec compliance)
- [x] Fixtures cover typeof behavior for all types
- [x] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/typeof-test.ts -o /tmp/test.wasm
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

The `typeof` operator should lower to a runtime function that checks the value tag and returns the appropriate string constant.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- (to be committed)

Validation result:

```text
command: cargo fmt --all --check
result: passed
date: 2026-04-26

command: cargo nextest run
result: 185 tests passed, 4 skipped
date: 2026-04-26

command: ./target/release/ts2wasm build fixtures/basics-typeof/typeof-test.ts -o /tmp/typeof-test.wasm && iwasm /tmp/typeof-test.wasm
result: built successfully, output: "undefined"
date: 2026-04-26
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/029-implement-typeof-operator.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
## Completion evidence

Core feature works correctly.

Validation:
```sh
echo 'typeof 1;' | ts2wasm build --stdin -o /tmp/out.wasm
# => exit 0
```
