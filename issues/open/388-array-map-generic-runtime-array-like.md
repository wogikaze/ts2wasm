---
id: 388
title: "Runtime array-like generic call for Array.prototype.map"
type: feature
area: runtime/builtins
class: ready
priority: P2
depends_on: [340]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Implement broader runtime array-like semantics for
`Array.prototype.map.call(receiver, callback)` beyond the static dense receiver
slice closed by issue 340.

## Problem

Issue 340 supports static dense receivers only. Generic map still needs runtime
property lookup over array-like receivers with dynamic `length` and indexed
properties.

## Desired final state

- `Array.prototype.map.call(obj, callback)` reads `obj.length` at runtime.
- Numeric indexed properties are read through runtime property lookup.
- Missing or sparse properties follow the project sparse-array policy and issue
  338 boundary.
- Existing static dense generic-call fixtures continue to pass.

## Scope

In scope:

- Runtime receiver lowering for non-static array-like objects.
- Runtime indexed property reads for map iteration.
- Node/iwasm differential fixtures for dynamic object receivers.

Out of scope:

- Sparse hole semantics already tracked by issue 338 unless this issue adopts a
  narrow shared helper behind that boundary.
- Test262 parser/frontend blockers tracked by issue 389.

## Acceptance criteria

- [ ] A dynamic object receiver fixture matches Node output under `iwasm`.
- [ ] Existing issue 340 static dense generic-call fixtures still pass.
- [ ] The implementation documents any remaining sparse-hole limitation.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli array_map
mise run update-issue-index -- --check
mise run check issues
```

## Completion evidence

Fill when moving to `done/`.
