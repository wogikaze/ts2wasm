---
id: 316
title: "Fix Object.keys backend-io error"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P0
depends_on: []
blocks: []
created: 2026-04-30
updated: 2026-04-30
---

## Summary

Fix backend-io error for Object.keys test case that is currently blocked.

## Problem

One test262 test case for Object.keys is blocked with backend-io error:

- `reference/test262/test/built-ins/Object/keys/15.2.3.14-3-4.js`

This case fails with `BackendIo: wat2wasm failed` during WASM generation, preventing compilation.

## Current failure

Reproduction:

```sh
cargo run -q -- build reference/test262/test/built-ins/Object/keys/15.2.3.14-3-4.js -o /tmp/object-keys.wasm --host-deny
```

Current result:

```text
error: [BackendIo] wat2wasm failed
```

## Desired final state

The Object.keys test case compiles successfully and produces correct runtime output under iwasm.

## Scope

In scope:

- [ ] Fix backend-io error for Object.keys in the specific test case
- [ ] Ensure correct behavior for the edge case in the test
- [ ] Add regression coverage for Object.keys edge case

Out of scope:

- Full Object.keys implementation for all edge cases beyond the specific test case
- Other Object built-in methods

## Affected paths

Expected:

- `crates/ir/src/lowered/program_builtins.rs`
- `crates/backend-wasm/src/runtime_builder.rs`
- `crates/backend-wasm/src/runtime_arrays_objects.rs`
- `crates/backend-wasm/src/runtime_fn.rs`
- `crates/backend-wasm/src/runtime_fn_impl.rs`
- `crates/backend-wasm/src/runtime_link_plan.rs`
- `crates/cli/tests/`

Do not touch:

- Other Object built-in implementations unless directly related to keys

## Acceptance criteria

- [ ] The test case compiles without backend-io errors
- [ ] Runtime output matches Node.js behavior
- [ ] Existing tests still pass
- [ ] No regression in other Object built-in methods

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo run -q -- build reference/test262/test/built-ins/Object/keys/15.2.3.14-3-4.js -o /tmp/object-keys.wasm --host-deny
printf '' | iwasm /tmp/object-keys.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected
- [ ] updated: `docs/14-runtime-abi.md` if Object built-in ABI changes

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
