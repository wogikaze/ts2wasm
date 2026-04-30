---
id: 315
title: "Fix Math.max/min backend-io errors"
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

Fix backend-io errors for Math.max and Math.min test cases that are currently blocked.

## Problem

Two test262 test cases for Math.max and Math.min are blocked with backend-io errors:

- `reference/test262/test/built-ins/Math/max/zeros.js`
- `reference/test262/test/built-ins/Math/min/zeros.js`

These cases fail with `BackendIo: wat2wasm failed` during WASM generation, preventing compilation.

## Current failure

Reproduction:

```sh
cargo run -q -- build reference/test262/test/built-ins/Math/max/zeros.js -o /tmp/math-max-zeros.wasm --host-deny
cargo run -q -- build reference/test262/test/built-ins/Math/min/zeros.js -o /tmp/math-min-zeros.wasm --host-deny
```

Current result:

```text
error: [BackendIo] wat2wasm failed
```

## Desired final state

Both Math.max and Math.min test cases compile successfully and produce correct runtime output under iwasm.

## Scope

In scope:

- [ ] Fix backend-io error for Math.max with zero arguments
- [ ] Fix backend-io error for Math.min with zero arguments
- [ ] Ensure correct behavior for edge cases (zeros, no arguments)
- [ ] Add regression coverage for Math.max/min edge cases

Out of scope:

- Full Math.max/min implementation for all edge cases beyond the specific test cases
- Other Math built-in methods

## Affected paths

Expected:

- `crates/ir/src/lowered/program_builtins.rs`
- `crates/backend-wasm/src/runtime_builder.rs`
- `crates/backend-wasm/src/runtime_builtins_host.rs`
- `crates/backend-wasm/src/runtime_fn.rs`
- `crates/backend-wasm/src/runtime_fn_impl.rs`
- `crates/cli/tests/`

Do not touch:

- Other Math built-in implementations unless directly related to max/min

## Acceptance criteria

- [ ] Both test cases compile without backend-io errors
- [ ] Runtime output matches Node.js behavior
- [ ] Existing tests still pass
- [ ] No regression in other Math built-in methods

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
cargo run -q -- build reference/test262/test/built-ins/Math/max/zeros.js -o /tmp/math-max-zeros.wasm --host-deny
cargo run -q -- build reference/test262/test/built-ins/Math/min/zeros.js -o /tmp/math-min-zeros.wasm --host-deny
printf '' | iwasm /tmp/math-max-zeros.wasm
printf '' | iwasm /tmp/math-min-zeros.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected
- [ ] updated: `docs/14-runtime-abi.md` if Math built-in ABI changes

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
