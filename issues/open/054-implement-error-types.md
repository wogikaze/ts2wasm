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

- `crates/backend-wasm/src/` (runtime builtins)
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

Progress 2026-04-28:

- Reproduced current behavior for `new Error("msg")`, `new TypeError("msg")`, `new ReferenceError("msg")`, and `new SyntaxError("msg")`: each failed at build time with `issue-207: instanceof right-hand side must be a supported class constructor`.
- Implemented a first runtime slice that lowers `new Error(...)`, `new TypeError(...)`, `new ReferenceError(...)`, and `new SyntaxError(...)` to heap objects with a Node-differential `.message` property.
- Added `fixtures/builtins-and-io/error-message.ts` and `error_message_fixture_matches_node_output_under_iwasm`.
- Remaining criteria before close: `.stack` is still not implemented; non-string message coercion and Error prototype identity are not yet covered.

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
