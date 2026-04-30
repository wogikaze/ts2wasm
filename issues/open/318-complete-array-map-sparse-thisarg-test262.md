---
id: 318
title: "Complete Array.prototype.map sparse thisArg and Test262 semantics"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-04-30
updated: 2026-04-30
---

## Summary

Complete the residual `Array.prototype.map` compatibility work left out of the
named-callback slice closed by issue 270 and the arrow/chained receiver slice
closed by issue 295.

This issue owns sparse-array behavior, `thisArg`, `Array.prototype.map.call(...)`
or a deliberate split of generic call behavior, and a fixed Test262 validation
window.

## Problem

Problem: supported dense-array map calls work, but `Array.prototype.map` still
lacks sparse array hole handling, callback `thisArg`, generic call behavior, and
Test262-backed compatibility evidence.

## Current failure

Representative Test262 case:

```sh
cargo run -q -- build reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-2-19.js -o /tmp/array-map-call.wasm --host-deny
```

The current implementation keeps `Array.prototype.map.call(...)` unsupported.
Existing diagnostics are covered by `crates/cli/tests/m2_node_diff.rs` for
fixtures including `fixtures/builtins-and-io/array-map-call-unsupported.ts`.

## Desired final state

`Array.prototype.map` has Node-compatible behavior for a fixed compatibility
slice that includes sparse arrays and `thisArg`, with generic call semantics
implemented or split into a specific child issue with exact diagnostics.

## Scope

In scope:

- [ ] Sparse array holes are skipped and preserved according to Node behavior.
- [ ] Callback `thisArg` is passed correctly for supported callback forms.
- [ ] `Array.prototype.map.call(...)` behavior is implemented or split into a
      narrower issue with exact evidence.
- [ ] A fixed Test262 map window is validated and recorded.
- [ ] Existing dense-array named callback and arrow callback fixtures keep
      matching Node output.

Out of scope:

- Async callbacks and Promise semantics unless a child issue explicitly owns
  them.
- Broad iterator or object-model redesign outside what map requires.

## Affected paths

Expected:

- `crates/ir/src/lowered/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/`
- `artifacts/coverage/` if reference coverage is updated

Do not touch:

- unrelated array methods
- problem-specific source rewrite hooks

## Acceptance criteria

- [ ] A sparse-array map fixture matches Node output under `iwasm`.
- [ ] A `thisArg` map fixture matches Node output under `iwasm`.
- [ ] The representative Test262 path
      `reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-2-19.js`
      is implemented or split with a precise follow-up.
- [ ] Existing issue-270 and issue-295 dense-array map fixtures still pass.
- [ ] Docs/current-state/issues are synchronized when behavior changes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
python scripts/manager.py update-issue-index --check
python scripts/manager.py check issues
```

Impacted commands:

```sh
cargo nextest run -p ts2wasm-cli array_map
cargo run -q -- build reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-2-19.js -o /tmp/array-map-call.wasm --host-deny
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected
- [ ] updated if the array representation contract changes

Current state:

- [ ] not affected
- [ ] updated: `current-state.md` if sparse/generic map behavior is implemented

Follow-up issues:

- [ ] none
- [ ] created if generic call semantics are split

## Notes

Issue 270 is now the historical dense-array named-callback slice. Issue 295 is
the historical arrow/chained receiver slice. Do not use either to claim sparse
array, `thisArg`, or broad Test262 map compatibility.

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
