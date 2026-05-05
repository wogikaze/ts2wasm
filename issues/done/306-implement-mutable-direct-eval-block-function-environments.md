---
id: 306
title: "Implement mutable direct eval block-function environments"
type: feature
area: frontend/ir/runtime
class: done
priority: P3
depends_on: []
blocks: [302]
created: 2026-04-29
updated: 2026-04-29
status: done
completed: 2026-04-29
---

## Summary

Implement the concrete mutable environment support needed by the selected
direct-eval Annex B block-function scoping reference.

Problem: `func-block-decl-eval-func-block-scoping.js` now gets past the static
eval parser slice and test262 assertion lowering, but the remaining
function-valued local call observes bindings mutated from an eval-created
function.

## Problem

The supported direct-eval slice can expand a static block function declaration
into ordinary compiler statements. The block-scoping reference additionally
requires the eval-created function to mutate and observe:

- outer observer bindings: `initialBV`, `currentBV`, `varBinding`
- the eval block's function binding: `f`
- function-valued local calls after mutation: `initialBV()` and `varBinding()`

The current ordinary closure path captures immutable locals and deliberately
rejects or fails mutable captured environments. Treating this as broader
dynamic eval would be unsafe; the required work is a concrete mutable
environment cell slice for this static direct-eval shape.

## Current failure

Reproduction:

```sh
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference \
  mise run reference-coverage -- test262 \
  --path-filter annexB/language/eval-code/direct/func-block-decl-eval-func-block-scoping.js \
  --detail
```

Current result after issue-302 assertion passthrough:

```text
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-block-scoping.js: UnresolvedName: name-resolution
```

Smart triage evidence:

```text
Diagnostic: UnresolvedName / resolver-symbol
message: unresolved name: `initialBV`
failure follows assert.sameValue(initialBV(), "decl", ...)
```

## Desired final state

The compiler supports the selected static direct-eval block-function
environment behavior without enabling indirect eval, shadowed eval, non-string
eval inputs, or dynamic eval source.

## Scope

In scope:

- [x] Mutable environment cells for the static direct-eval block-function
      shape used by `func-block-decl-eval-func-block-scoping.js`.
- [x] Function-valued local calls for the eval-created function values needed
      by `initialBV()` and `varBinding()`.
- [x] Preservation of block function binding independence: mutation of block
      local `f` does not overwrite the outer var-scoped function value used by
      `varBinding()`.
- [x] Focused Node/iwasm fixture under `fixtures/core-semantics/`.

Out of scope:

- Indirect eval, shadowed eval, non-string eval input, and dynamic eval source.
- The broader `func-if-decl-*`, existing-binding, and early-error eval families.
- Issue 300/304 runtime memory policy work and BigInt runtime work.
- General class-method mutable environment work tracked by issue 301 unless a
  shared primitive is deliberately extracted in that issue's scope.

## Affected paths

Expected:

- `crates/ir/src/lowered/`
- `crates/backend-wasm/src/` only if new lowered environment operations need emission
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `issues/done/302-implement-direct-eval-block-function-declaration-slice.md`
- `issues/index.md`

Do not touch:

- `docs/`
- issue 300/304 runtime memory policy files
- BigInt runtime/ABI files
- unrelated web/report artifacts

## Acceptance criteria

- [x] `func-block-decl-eval-func-block-scoping.js` reports `build_pass` and
      `semantic_pass=1`.
- [x] `func-block-decl-eval-func-init.js` remains `build_pass` and
      `semantic_pass=1`.
- [x] A focused fixture covers the mutable direct-eval block-function
      environment behavior and passes Node/iwasm differential validation.
- [x] Existing direct-eval block-function fixtures and ordinary function direct
      call fixture remain in focused validation.
- [x] Unsupported shadowed eval remains rejected with the issue-302 diagnostic.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli direct_eval_block_function
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --path-filter annexB/language/eval-code/direct/func-block-decl-eval-func-init.js --detail
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --path-filter annexB/language/eval-code/direct/func-block-decl-eval-func-block-scoping.js --detail
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo nextest run
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 300
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

This issue is split from issue 302 because the remaining behavior is mutable
environment support, not direct eval source selection. Keep the implementation
limited to the static block-function reference shape until issue 302 can close.

2026-04-29 child progress:

- Added a narrow lowered IR env-cell collection path for the static direct-eval
  block-function IIFE shape only. The collector marks the eval block function
  binding and its mutated observer captures as environment cells, and marks the
  function-valued observer binding shape needed for `initialBV()`.
- Reused existing `EnvCellNew` / `EnvCellGet` / `EnvCellSet` and heap closure
  emission instead of adding new backend environment operations.
- Added
  `fixtures/core-semantics/direct-eval-block-function-mutable-env.ts`, covering
  the selected `initialBV` / `currentBV` / `varBinding` mutation behavior with
  Node/iwasm differential validation.
- Validation passed: `cargo fmt --all --check`; `cargo nextest run -p
  ts2wasm-cli direct_eval_block_function`; selected test262 path-filter
  coverage for both `func-block-decl-eval-func-init.js` and
  `func-block-decl-eval-func-block-scoping.js`, each reporting `build_pass=1`
  and `semantic_pass=1`; `mise run update-issue-index -- --check`.
- Not closed in this child cycle because `mise run check issues` is blocked by
  pre-existing missing test262 JSONL artifact references in unrelated issue
  files, and full `cargo nextest run` is blocked by the unrelated runtime memory
  policy test
  `ts2wasm-backend-wasm::tests::alloc_heap_emits_gc_header_and_trigger_contract`
  expecting `(memory (export "memory") 2 16)`.

## Completion evidence

Commits:

- `51c27cc5` (`issue-306: progress direct eval mutable env cells`)

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli direct_eval_block_function
result: pass; 2 tests passed
date: 2026-04-29

command: TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --path-filter annexB/language/eval-code/direct/func-block-decl-eval-func-init.js --detail
result: pass; build_pass=1, semantic_pass=1
date: 2026-04-29

command: TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --path-filter annexB/language/eval-code/direct/func-block-decl-eval-func-block-scoping.js --detail
result: pass; build_pass=1, semantic_pass=1
date: 2026-04-29

command: mise run update-issue-index -- --check && mise run check issues
result: pass
date: 2026-04-29
```

Remaining risks:

- The implementation is intentionally limited to the selected static direct-eval
  block-function IIFE shape. Broader dynamic eval families remain out of scope.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/306-implement-mutable-direct-eval-block-function-environments.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
