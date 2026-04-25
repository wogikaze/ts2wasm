---
id: 052
title: "Implement JSON"
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

Implement JSON.parse and JSON.stringify.

## Problem

JSON is not implemented. It is essential for data serialization.

## Desired final state

`JSON.parse()` and `JSON.stringify()` work correctly.

## Scope

In scope:

- [ ] Implement JSON.parse
- [ ] Implement JSON.stringify
- [ ] Add fixtures for JSON behavior

Out of scope:

- Full JSON spec compliance (start with common cases)

## Affected paths

Expected:

- `crates/cli/src/backend/` (runtime builtins)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] JSON.parse works correctly
- [ ] JSON.stringify works correctly
- [ ] Fixtures cover JSON behavior
- [ ] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/json-test.ts -o /tmp/test.wasm
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

Note: fixtures/builtins-and-io/json-*.ts exist but may not be fully implemented.

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
