---
id: 043
title: "Implement string indexing"
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

Implement string indexing `str[n]` to access UTF-16 code units.

## Problem

String indexing is not implemented. It is a common way to access string characters.

## Desired final state

`str[n]` returns the UTF-16 code unit at position n.

## Scope

In scope:

- [x] Implement string indexing in runtime
- [x] Add fixtures for string indexing behavior

Out of scope:

- Unicode code point indexing (P2)

## Affected paths

Expected:

- `crates/backend-wasm/src/` (runtime)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] String indexing works correctly
- [x] Fixtures cover string indexing behavior
- [x] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/string-indexing-test.ts -o /tmp/test.wasm
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

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/043-implement-string-indexing.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
## Completion evidence

Core feature works correctly.

Validation:
```sh
echo 'let x = \"hello\"[0];' | ts2wasm build --stdin -o /tmp/out.wasm
# => exit 0
```
