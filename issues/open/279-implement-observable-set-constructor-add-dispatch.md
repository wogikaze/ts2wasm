---
id: 279
title: "Implement observable Set constructor add dispatch"
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

Complete the spec-observable part of `new Set(values)` that calls the current `Set.prototype.add` property for each yielded value.

Problem: issue 276 added a narrow dense-array constructor path through the current direct `$set_add` runtime helper, but the runtime does not yet model mutable `Set.prototype.add` dispatch, so Test262-style monkey-patching is not observable.

## Current failure

The following Test262-derived behavior is not supported by the direct-helper Set constructor slice:

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

`new Set(values)` obtains and calls the observable `add` method for each supported input value, including when `Set.prototype.add` has been replaced before construction.

## Scope

In scope:

- [ ] Represent enough of `Set.prototype.add` for assignment and retrieval to be observable.
- [ ] Make `new Set(values)` call the resolved add method once per supported dense-array element.
- [ ] Add Node/iwasm differential coverage based on `set-iterable-calls-add.js`.

Out of scope:

- General custom iterators.
- Iterator closing on abrupt completion.
- Set iteration and spread behavior.
- SameValueZero identity fixes beyond the current Set representation.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `fixtures/builtins-and-io/`
- `crates/cli/tests/m2_node_diff.rs`

Do not touch:

- `web-ui/`

## Acceptance criteria

- [ ] The fixture proves `Set.prototype.add` replacement increments a counter once per supported array element during `new Set(values)`.
- [ ] The constructed Set still contains each provided value.
- [ ] Node and iwasm stdout match for the fixture.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli set
```

Impacted commands:

```sh
cargo nextest run -p ts2wasm-cli set_constructor_array_fixture_matches_node_output_under_iwasm
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

Split from issue 276 because the smallest safe progress slice supports dense-array construction through direct Set helpers, while prototype monkey-patching requires broader builtin prototype dispatch support.

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
