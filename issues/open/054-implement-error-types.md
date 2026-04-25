---
id: 054
title: "Implement Error types"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement Error, TypeError, and other error types.

## Problem

Error types are not implemented. They are essential for error handling.

## Desired final state

`new Error()`, `new TypeError()`, etc. work correctly.

## Scope

In scope:

- [ ] Implement Error constructor
- [ ] Implement TypeError constructor
- [ ] Implement ReferenceError constructor
- [ ] Implement SyntaxError constructor
- [ ] Implement Error.prototype.message
- [ ] Implement Error.prototype.stack
- [ ] Add fixtures for Error behavior

Out of scope:

- Full Error spec compliance (start with basic error types)

## Affected paths

Expected:

- `crates/cli/src/backend/` (runtime builtins)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] Error constructors work correctly
- [ ] Error properties work correctly
- [ ] Fixtures cover Error behavior
- [ ] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/error-test.ts -o /tmp/test.wasm
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
