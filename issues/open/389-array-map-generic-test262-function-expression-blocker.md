---
id: 389
title: "Unblock Array.map generic Test262 representative with function-expression initializer"
type: bug
area: frontend/parser
class: ready
priority: P2
depends_on: [273, 340]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

The representative generic Array.map Test262 case still fails before map
lowering even after issue 273 was marked done.

## Problem

`mise run reference-triage -- test262 reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-2-19.js`
reports `UnsupportedSyntax` with an `issue-273` diagnostic at the
function-expression initializer:

```js
var fun = function(a, b) {
  return a + b;
};
```

The project fixture for the equivalent static function receiver now passes, so
this issue should determine whether the remaining failure is parser/frontend,
Test262 preprocessing, or stale diagnostic classification.

## Desired final state

- The representative Test262 file reaches Array.map lowering or passes.
- If it still fails, the diagnostic names the actual remaining unsupported
  feature instead of the closed issue 273.
- issue 340 static dense generic-call fixtures continue to pass.

## Scope

In scope:

- Reproduce and classify the current `issue-273` diagnostic on the Test262 path.
- Fix anonymous/named function-expression initializer handling or preprocessor
  handling if that is the true blocker.
- Add a narrow regression fixture if a compiler bug is found.

Out of scope:

- Full runtime array-like map behavior, tracked by issue 388.
- Sparse array holes, tracked by issue 338.

## Acceptance criteria

- [ ] `mise run reference-triage -- test262 reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-2-19.js` no longer reports a closed issue 273 blocker.
- [ ] Any new blocker is represented by an open issue with evidence.
- [ ] `cargo nextest run -p ts2wasm-cli array_map` still passes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli array_map
mise run reference-triage -- test262 reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-2-19.js
mise run update-issue-index -- --check
mise run check issues
```

## Completion evidence

Fill when moving to `done/`.
