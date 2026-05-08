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

- [x] Fix the root stack-balance issue in runtime array includes emission without suppressing the fixture.

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

- [x] `cargo nextest run -p ts2wasm-cli array_includes_fixture_matches_node_output_under_iwasm` passes.
- [x] `cargo nextest run -p ts2wasm-cli build_smoke_array_includes_method` does not regress.

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

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] 5216 tracks the next unrelated full-gate failure found after the array includes fixture passed.

## Notes

Discovered while validating issue 5205.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- This commit.

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-05-06

command: cargo nextest run -p ts2wasm-cli array_includes_fixture_matches_node_output_under_iwasm
result: pass
date: 2026-05-06

command: cargo nextest run -p ts2wasm-cli build_smoke_array_includes_method
result: pass
date: 2026-05-06

command: cargo nextest run
result: fail after the 5215 array-includes failure was cleared; stopped at issue 5216 (`function-arguments.ts` TS2554 arity mismatch)
date: 2026-05-06
```

Remaining risks:

- Full `cargo nextest run` still fails on unrelated issue 5216.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/open/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

