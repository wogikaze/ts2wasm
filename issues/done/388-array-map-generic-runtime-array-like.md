---
id: 388
title: "Runtime array-like generic call for Array.prototype.map"
type: feature
area: runtime/builtins
class: done
priority: P2
depends_on: [340]
blocks: []
created: 2026-05-01
updated: 2026-05-01
completed: 2026-05-01
---

## Summary

Implemented the first runtime array-like receiver slice for
`Array.prototype.map.call(receiver, callback)` beyond the static dense receiver
slice closed by issue 340. This closed slice supports identity arrow callbacks
and reads both `length` and numeric indexed properties at runtime.

## Problem

Issue 340 supports static dense receivers only. Generic map still needs runtime
property lookup over array-like receivers with dynamic `length` and indexed
properties.

## Desired final state

- [x] `Array.prototype.map.call(obj, callback)` reads `obj.length` at runtime
      for the identity-arrow callback slice.
- [x] Numeric indexed properties are read through runtime property lookup.
- [x] Missing or sparse properties follow the project sparse-array policy and issue
  338 boundary.
- [x] Existing static dense generic-call fixtures continue to pass.

Split follow-up work:

- issue 395 tracks non-identity runtime callback dispatch for array-like generic
  map receivers.

## Scope

In scope for the completed slice:

- Runtime receiver lowering for non-static array-like objects.
- Runtime indexed property reads for map iteration.
- Node/iwasm differential fixtures for dynamic object receivers.
- Identity arrow callbacks (`value => value`) over dense runtime array-like
  object receivers.

Out of scope:

- Sparse hole semantics already tracked by issue 338 unless this issue adopts a
  narrow shared helper behind that boundary.
- Test262 parser/frontend blockers tracked by issue 389.
- Non-identity runtime callback dispatch for array-like generic map receivers,
  tracked by issue 395.

## Acceptance criteria

- [x] A dynamic object receiver fixture matches Node output under `iwasm`.
- [x] Existing issue 340 static dense generic-call fixtures still pass.
- [x] The implementation documents any remaining sparse-hole limitation.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli array_map
mise run update-issue-index -- --check
mise run check issues
```

## Completion evidence

Commits:

- child close commit: `issue-388: implement runtime array-like map identity slice`

Validation result:

```text
cargo fmt --all --check: pass
cargo nextest run -p ts2wasm-cli array_map: pass, 14/14 selected tests
mise run update-issue-index -- --check: pass
mise run check issues: pass
```

Evidence:

- Added
  `fixtures/core-semantics/array-map-generic-call-runtime-array-like.ts`, which
  creates an object receiver, writes numeric properties dynamically, computes
  `length` through `1 + 1`, and validates
  `Array.prototype.map.call(receiver, value => value)` against Node output under
  `iwasm`.
- Added `$array_map_array_like_identity`, which reads runtime `length` through
  `$get_length` and reads numeric indexed values through `$index`.
- Existing issue 340 generic-call fixtures remained covered by
  `cargo nextest run -p ts2wasm-cli array_map`.

Remaining risks:

- Sparse-hole semantics remain issue 338. The closed runtime helper currently
  stores `undefined` for missing indexed properties rather than representing
  holes.
- Non-identity callback dispatch for runtime array-like receivers remains issue
  395.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/388-array-map-generic-runtime-array-like.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
