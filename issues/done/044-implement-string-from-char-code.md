---
id: 044
title: "Implement String.fromCharCode and charCodeAt"
type: feature
area: runtime/builtins
class: done
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

- [x] Implement String.fromCharCode
- [x] Implement String.prototype.charCodeAt
- [x] Add fixtures for char code conversion

Out of scope:

- String.fromCodePoint (P2)
- String.prototype.codePointAt (P2)

## Affected paths

Expected:

- `crates/backend-wasm/src/` (runtime builtins)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] String.fromCharCode works correctly
- [x] String.prototype.charCodeAt works correctly
- [x] Fixtures cover char code conversion
- [x] No regression in existing fixtures

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

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `feat: implement String.fromCharCode and String.prototype.charCodeAt`

Validation result:

```text
command: cargo nextest run
result: 207 passed, 4 skipped
date: 2026-04-26
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/044-implement-string-from-char-code.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
