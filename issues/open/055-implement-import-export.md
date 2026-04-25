---
id: 055
title: "Implement import and export"
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

Implement ES6 import/export static module system.

## Problem

Import/export are not implemented. They are essential for modular code organization.

## Desired final state

`import { x } from './mod.js'` and `export { x }` work correctly.

## Scope

In scope:

- [ ] Add import syntax to lexer/parser
- [ ] Add export syntax to lexer/parser
- [ ] Implement module resolution
- [ ] Implement module loading
- [ ] Add fixtures for import/export behavior

Out of scope:

- Dynamic import() (P2)
- require() (P2)

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/cli/src/backend/` (module loading)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] import parses correctly
- [ ] export parses correctly
- [ ] Module resolution works correctly
- [ ] Fixtures cover import/export behavior
- [ ] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/module-test.ts -o /tmp/test.wasm
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

This is a major feature requiring module system design.

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
