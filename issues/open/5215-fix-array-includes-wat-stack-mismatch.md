---
id: 5215
title: "Fix array includes WAT stack mismatch"
type: bug
area: runtime/builtins
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Fix the `Array.prototype.includes` runtime emission so `fixtures/builtins-and-io/array-includes.ts` builds to valid WAT/WASM and the Node/iwasm differential fixture can run.

## Problem

Problem: `fixtures/builtins-and-io/array-includes.ts` emits invalid WAT with a residual `[i32, i32, i32]` stack at function end.

While validating issue 5205 with the full `cargo nextest run` gate, the suite progressed past the backend residual-expression tests and then failed while building `fixtures/builtins-and-io/array-includes.ts`.

The emitted runtime WAT leaves extra stack values at the end of a void/control-flow context, causing `wat2wasm` to reject the module.

## Current failure

Reproduction:

```sh
cargo nextest run -p ts2wasm-cli array_includes_fixture_matches_node_output_under_iwasm
```

Observed during:

```sh
cargo nextest run
```

Failure excerpt:

```text
fixtures/builtins-and-io/array-includes.ts
BackendIo: wat2wasm failed
/tmp/ts2wasm-23834-0.wat:1439:26: error: type mismatch at end of function, expected [] but got [i32, i32, i32]
```

## Desired final state

`Array.prototype.includes` runtime lowering emits stack-balanced WAT, the fixture builds successfully, and the Node/iwasm differential test compares runtime output instead of failing during `wat2wasm`.

## Scope

In scope:

- [ ] Fix the root stack-balance issue in runtime array includes emission without suppressing the fixture.

Out of scope:

- Broad array-builtin triage for issue 313.
- Changing the M2 differential harness.

## Affected paths

Expected:

- `crates/backend-wasm/src/runtime_arrays.rs`
- `fixtures/builtins-and-io/array-includes.ts`

Do not touch:

- `scripts/lib/test262_harness.py`
- `crates/compiler/src/test262_preprocessor.rs`

## Acceptance criteria

- [ ] `cargo nextest run -p ts2wasm-cli array_includes_fixture_matches_node_output_under_iwasm` passes.
- [ ] `cargo nextest run -p ts2wasm-cli build_smoke_array_includes_method` does not regress.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli array_includes_fixture_matches_node_output_under_iwasm
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
git diff --check
```

Impacted commands:

```sh
cargo nextest run -p ts2wasm-cli build_smoke_array_includes_method
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

Discovered while validating issue 5205.

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
