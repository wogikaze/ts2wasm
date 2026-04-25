---
id: 044
title: "Implement String.fromCharCode and charCodeAt"
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

Implement String.fromCharCode and String.prototype.charCodeAt.

## Problem

String.fromCharCode and charCodeAt are not implemented. They are essential for character code conversion.

## Desired final state

`String.fromCharCode(65)` returns "A", `"A".charCodeAt(0)` returns 65.

## Scope

In scope:

- [ ] Implement String.fromCharCode
- [ ] Implement String.prototype.charCodeAt
- [ ] Add fixtures for char code conversion

Out of scope:

- String.fromCodePoint (P2)
- String.prototype.codePointAt (P2)

## Affected paths

Expected:

- `crates/cli/src/backend/` (runtime builtins)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] String.fromCharCode works correctly
- [ ] String.prototype.charCodeAt works correctly
- [ ] Fixtures cover char code conversion
- [ ] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/char-code-test.ts -o /tmp/test.wasm
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
