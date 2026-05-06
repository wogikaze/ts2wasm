---
id: 5217
title: "Align ambient value name resolver test counts"
type: bug
area: ir/name-resolution
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Update the ambient value name resolver tests to match the current contract that ambient declarations are preserved as metadata through name resolution but do not produce lowered runtime statements.

## Problem

Problem: full `cargo nextest run` now reaches two ambient value tests whose expected lowered local/statement counts predate the current ambient declaration preservation contract.

The tests pass through name resolution and builtin resolution, but their final lowered count assertions still expect the old erased/declaration shape.

## Current failure

Reproduction:

```sh
cargo nextest run -p ts2wasm-ir resolves_ambient
```

Failure excerpt:

```text
resolves_ambient_const_shorthand_without_runtime_decl: left: 1, right: 2
resolves_ambient_value_decl_in_array_and_object_literals: left: 2, right: 3
```

## Desired final state

The tests assert the intended contract directly: ambient value declarations remain visible to name/builtin resolution, references to them resolve, and no runtime declaration statement is emitted for the ambient metadata.

## Scope

In scope:

- [ ] Fix stale ambient value test count expectations without weakening ambient value resolution coverage.

Out of scope:

- Changing ambient value lowering behavior.
- Reverting issue 5161 or issue 5206 semantics.

## Affected paths

Expected:

- `crates/ir/src/name_resolver_tests.rs`

Do not touch:

- `crates/ir/src/name_resolver.rs`
- `crates/compiler/src/dump.rs`

## Acceptance criteria

- [ ] `cargo nextest run -p ts2wasm-ir resolves_ambient` passes.
- [ ] Full `cargo nextest run` no longer fails at the two ambient value count assertions.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir resolves_ambient
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
git diff --check
```

Impacted commands:

```sh
cargo nextest run
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

Discovered while validating issue 5216.

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
