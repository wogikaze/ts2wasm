---
id: 395
title: "Runtime array-like Array.prototype.map callback dispatch"
type: feature
area: runtime/builtins
class: done
priority: P2
depends_on: [388]
blocks: []
created: 2026-05-01
updated: 2026-05-01
completed: 2026-05-01
---

## Summary

Extend the runtime array-like `Array.prototype.map.call(receiver, callback)`
slice closed by issue 388 beyond identity arrow callbacks.

## Problem

Issue 388 validates runtime `length` and numeric property lookup for dense
array-like object receivers, but only for `value => value`. Non-identity
callbacks still need a safe dispatch strategy over runtime-length receivers.

## Desired final state

- Runtime array-like generic map can apply supported callback forms to each
  runtime-indexed element.
- Callback arguments preserve the existing map contract: value, index, and
  receiver where supported.
- Existing issue 388 identity runtime fixture and issue 340 static dense
  fixtures continue to pass.

## Scope

In scope:

- One narrow non-identity callback family over runtime array-like receivers.
- Node/iwasm differential fixture for the supported callback family.
- Follow the issue 338 sparse-hole boundary.

Out of scope:

- Sparse-hole correctness, tracked by issue 338.
- Test262 parser/frontend blockers, tracked by issue 389.

## Acceptance criteria

- [x] A non-identity runtime array-like generic map fixture matches Node output
      under `iwasm`.
- [x] Existing issue 388 runtime identity fixture still passes.
- [x] Existing issue 340 static dense generic-call fixtures still pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli array_map
mise run update-issue-index -- --check
mise run check issues
```

## Completion evidence

2026-05-01 progress slice:

- Implemented the narrow runtime array-like callback family
  `value => value * 2` for `Array.prototype.map.call(receiver, callback)`.
- Added `fixtures/core-semantics/array-map-generic-call-runtime-array-like-double.ts`
  as Node/iwasm differential coverage for the non-identity callback family.
- Preserved the issue 388 identity runtime fixture and issue 340 static dense
  generic-call fixtures in the required `array_map` validation target.

Validation:

- `cargo fmt --all --check`: pass
- `cargo nextest run -p ts2wasm-cli array_map`: pass, 15/15 selected tests
- `mise run update-issue-index -- --check`: pass
- `mise run check issues`: pass
