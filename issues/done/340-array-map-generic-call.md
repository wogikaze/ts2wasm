---
id: 340
title: "Generic call for Array.prototype.map (static dense receiver slice)"
type: feature
area: runtime/builtins
class: done
priority: P2
depends_on: [334]
blocks: []
created: 2026-04-30
updated: 2026-05-01
completed: 2026-05-01
---

## Summary

Implemented the static dense receiver slice for `Array.prototype.map.call(...)`.
This covers array-literal receivers, dense object-literal array-like receivers,
and function receivers whose indexed properties are assigned statically before
the call.

## Problem

`Array.prototype.map.call(...)` was previously rejected for generic receiver
forms even when all receiver elements were statically known.

## Desired final state

For this closed slice:

- [x] Detect `Array.prototype.map.call(...)`.
- [x] Handle array-literal receivers.
- [x] Handle dense object-literal array-like receivers with numeric `length`.
- [x] Handle function receivers with static indexed assignments.
- [x] Support named function callbacks.
- [x] Support arrow/function-expression callbacks already covered by existing
      map lowering.
- [x] Add generic call map fixtures.
- [x] Preserve existing direct map fixtures.

Split follow-up work:

- issue 388 tracks arbitrary runtime array-like generic map semantics.
- issue 389 tracks the representative Test262 validation blocker for
  `reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-2-19.js`.

## Scope

In scope for the completed slice:

- Static receiver recognition for `Array.prototype.map.call(...)`.
- Dense receiver element extraction for array literals, object literals, and
  function receivers with statically assigned numeric indexes.
- Node/iwasm differential fixtures for the supported receiver forms.

Out of scope:

- Sparse array holes, tracked by issue 338.
- Arbitrary runtime array-like property iteration, tracked by issue 388.
- Representative Test262 blocker unrelated to map lowering, tracked by issue
  389.

## Affected paths

- `crates/ir/src/lowered/resolver.rs`
- `crates/ir/src/lowered/resolver_extra.rs`
- `crates/ir/src/lowered/resolver_expr.rs`
- `crates/cli/tests/common/m2_node_diff_fixture_tests.rs`
- `fixtures/builtins-and-io/array-map-call-unsupported.ts`
- `fixtures/core-semantics/array-map-generic-call-object-literal.ts`
- `fixtures/core-semantics/array-map-generic-call-function-receiver.ts`

## Acceptance criteria

- [x] Generic call map fixture for array receiver matches Node output under
      `iwasm`.
- [x] Generic call map fixture for object-literal receiver matches Node output
      under `iwasm`.
- [x] Generic call map fixture for function receiver with static indexed
      assignments matches Node output under `iwasm`.
- [x] Existing direct map fixtures still pass.
- [x] Representative Test262 case
      `reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-2-19.js`
      is blocked by the parser/frontend diagnostic recorded in issue 389.

## Validation

```text
command: cargo fmt --all --check && cargo nextest run -p ts2wasm-cli array_map
result: pass; 13/13 selected tests passed
 date: 2026-05-01
```

```text
command: mise run reference-triage -- test262 reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-2-19.js
result: blocked before map lowering by UnsupportedSyntax / issue-273 diagnostic on the function-expression initializer; split to issue 389
 date: 2026-05-01
```

## Completion evidence

Commits:

- parent close commit: `issue-340: close static generic array map call slice`

Validation result:

```text
cargo fmt --all --check: pass
cargo nextest run -p ts2wasm-cli array_map: pass, 13/13 selected tests
mise run reference-triage -- test262 reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-2-19.js: blocked by unrelated parser/frontend diagnostic; follow-up issue 389
```

Remaining risks:

- This is not full ECMAScript generic map behavior. Runtime array-like objects,
  sparse/missing indexed properties, and broader property iteration remain
  separate issues.

## Progress evidence

2026-04-30 child-338-340:

- Implemented static lowering for `Array.prototype.map.call(...)` when the
  receiver is an array literal or dense object literal with `"0"..."length - 1"`
  properties and numeric `length`.
- Added `fixtures/core-semantics/array-map-generic-call-object-literal.ts`.
- Promoted `fixtures/builtins-and-io/array-map-call-unsupported.ts` to a
  positive fixture path and extended it to check mapped values.
- `cargo fmt --all --check`: pass.
- `cargo check -p ts2wasm-ir`: pass.

2026-04-30 continuation after parent issue 356 close:

- Rebased this branch to current master after parent closed issue 356.
- `mise run update-issue-index -- --check`: pass.
- `mise run check issues`: pass.
- `cargo nextest run -p ts2wasm-cli array_map`: pass, 11/11 tests.

2026-05-01 parent close slice:

- Added function receiver tracking for static indexed writes such as
  `fun[0] = 12` after `var fun = function (a, b) { ... }`.
- Added `fixtures/core-semantics/array-map-generic-call-function-receiver.ts`.
- Confirmed the selected `array_map` differential tests pass.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/340-array-map-generic-call.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
