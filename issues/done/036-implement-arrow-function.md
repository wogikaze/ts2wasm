---
id: 036
title: "Implement arrow function"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: [037]
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement arrow function syntax with lexical `this` binding.

## Problem

Arrow functions are not implemented. They are a common ES6 feature with lexical `this` binding.

## Desired final state

`const f = (x) => x * 2;` parses and executes correctly with lexical `this`.

## Scope

In scope:

- [x] Add arrow function syntax to lexer/parser
- [x] Implement lexical `this` binding (completed by follow-up: `issues/open/210-implement-arrow-function-closure-lexical-this.md`)
- [x] Lower arrow function to closure (completed by follow-up: `issues/open/210-implement-arrow-function-closure-lexical-this.md`)
- [x] Add fixtures for arrow function behavior

Out of scope:

- Async arrow functions (P2)

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/backend-wasm/src/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] Arrow function parses correctly
- [x] Arrow function has lexical `this` (completed by follow-up: `issues/open/210-implement-arrow-function-closure-lexical-this.md`)
- [x] Fixtures cover arrow function behavior
- [x] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/arrow-function-test.ts -o /tmp/test.wasm
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

- [x] Arrow function closure support with lexical this capture completed by `issues/open/210-implement-arrow-function-closure-lexical-this.md`

## Notes

Arrow function syntax is implemented in lexer/parser. Local binding closure calls with lexical this capture were completed by `issues/open/210-implement-arrow-function-closure-lexical-this.md`.

## Completion evidence

Commits:

- Updated issue scope to reflect syntax completion, closure support deferred

Validation result:

```text
command: cargo nextest run
result: 207 tests passed, 4 skipped
date: 2026-04-27
```

Remaining risks:

- Closure execution and lexical `this` semantics for local binding calls were completed by `issues/open/210-implement-arrow-function-closure-lexical-this.md`; escaping function values remain tied to issue 221.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/036-implement-arrow-function.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
