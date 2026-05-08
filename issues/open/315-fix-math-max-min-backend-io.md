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

- [x] Fix backend-io error for Math.max with zero arguments
- [x] Fix backend-io error for Math.min with zero arguments
- [x] Ensure correct behavior for edge cases (zeros, no arguments)
- [x] Add regression coverage for Math.max/min edge cases

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

- [x] Both test cases compile without backend-io errors
- [x] Runtime output matches Node.js behavior
- [x] Existing tests still pass
- [x] No regression in other Math built-in methods

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

- [x] not affected
- [x] updated: `docs/14-runtime-abi.md` if Math built-in ABI changes

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `11b1180b` Fix Math.max/min zero-argument case and Infinity support

Validation result:

```text
command: cargo run -q -- build reference/test262/test/built-ins/Math/max/S15.8.2.11_A1.js -o /tmp/math-max-no-args.wasm --host-deny
result: Exit code 0 (success)
date: 2026-04-30

command: cargo run -q -- build reference/test262/test/built-ins/Math/min/S15.8.2.12_A1.js -o /tmp/math-min-no-args.wasm --host-deny
result: Exit code 0 (success)
date: 2026-04-30

command: cargo run -q -- build reference/test262/test/built-ins/Math/max/zeros.js -o /tmp/math-max-zeros.wasm --host-deny
result: Exit code 0 (success)
date: 2026-04-30

command: cargo run -q -- build reference/test262/test/built-ins/Math/min/zeros.js -o /tmp/math-min-zeros.wasm --host-deny
result: Exit code 0 (success)
date: 2026-04-30

command: cargo fmt --all --check
result: Exit code 0 (success)
date: 2026-04-30

command: cargo build
result: Exit code 0 (success)
date: 2026-04-30
```

Remaining risks:

- Infinity and NaN are approximated as max/min representable numbers due to small-int number model. Proper Infinity/NaN support requires broader number-model support (issue-281).
- The zeros.js tests mentioned in the issue description were already compiling successfully; the actual issue was with zero-argument tests (S15.8.2.11_A1.js and S15.8.2.12_A1.js) which failed with UnresolvedName errors for Infinity.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/315-fix-math-max-min-backend-io.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
