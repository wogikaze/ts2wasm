---
id: 276
title: "Implement Set constructor from supported iterables"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Add a narrow executable slice for `new Set(values)` using currently supported array values, while preserving the spec-observable requirement that construction goes through `Set.prototype.add`.

Problem: issue 049 validates only the zero-argument Set constructor; the broad issue 272 cited `reference/test262/test/built-ins/Set/set-iterable-calls-add.js`, which requires constructor input to call `Set.prototype.add` once per yielded value.

## Current failure

The following reference behavior is not covered by the current fixture:

```javascript
var setAdd = Set.prototype.add;
var counter = 0;

Set.prototype.add = function(value) {
  counter++;
  setAdd.call(this, value);
};

var s = new Set([1, 2]);

console.log(counter);
console.log(s.has(1));
console.log(s.has(2));
```

Expected Node stdout:

```text
2
true
true
```

## Desired final state

`new Set(values)` consumes supported array inputs in insertion order, invokes the observable `add` method for each element, and produces Node/iwasm-equivalent output for the narrow fixture.

## Scope

In scope:

- [ ] Support `new Set(array)` for dense arrays in the current runtime representation.
- [ ] Preserve observable calls through `Set.prototype.add` for the supported slice.
- [ ] Add Node/iwasm differential coverage based on `set-iterable-calls-add.js`.

Out of scope:

- General custom iterator objects.
- Iterator closing on abrupt completion.
- Spread syntax over Set.
- Full Test262 Set constructor coverage.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `fixtures/builtins-and-io/`
- `crates/cli/tests/m2_node_diff.rs`

Do not touch:

- `web-ui/`

## Acceptance criteria

- [ ] A fixture based on `set-iterable-calls-add.js` shows the supported iterable constructor calls `Set.prototype.add` for each array element.
- [ ] The resulting Set contains each provided value.
- [ ] Node and iwasm stdout match for the fixture.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli set
```

Impacted commands:

```sh
cargo nextest run -p ts2wasm-cli map_set_collection_fixture_matches_node_output_under_iwasm
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

Split from stale broad issue 272 after issue 049 closed the zero-argument constructor/add/has/delete subset.

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
