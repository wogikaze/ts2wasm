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

- [x] Support `new Set(array)` for dense arrays in the current runtime representation.
- [ ] Preserve observable calls through `Set.prototype.add` for the supported slice; split to issue 279.
- [x] Add Node/iwasm differential coverage for dense-array construction.

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

- [ ] A fixture based on `set-iterable-calls-add.js` shows the supported iterable constructor calls `Set.prototype.add` for each array element; split to issue 279.
- [x] The resulting Set contains each provided value.
- [x] Node and iwasm stdout match for the dense-array constructor fixture.

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

- [x] created: `issues/open/279-implement-observable-set-constructor-add-dispatch.md`

## Notes

Split from stale broad issue 272 after issue 049 closed the zero-argument constructor/add/has/delete subset.

## Progress evidence

- 2026-04-29: Added `SetFromArray` runtime helper and lowering for `new Set(array)` when the constructor argument is a known dense array. This path preserves existing `new Set()`, `add`, `has`, `delete`, `size`, and `clear` behavior.
- 2026-04-29: Added `fixtures/builtins-and-io/set-constructor-array.ts` and `set_constructor_array_fixture_matches_node_output_under_iwasm` to prove dense-array construction, duplicate suppression through current Set storage, `has`, and `size`.
- 2026-04-29: Split observable `Set.prototype.add` monkey-patch dispatch to issue 279 because current Set methods are direct runtime helpers and Set prototype method mutation is not modeled yet.

Validation result:

```text
command: cargo nextest run -p ts2wasm-cli set
result: pass; 5 tests run, 5 passed, 330 skipped
date: 2026-04-29

command: cargo nextest run
result: pass; 522 tests run, 522 passed, 4 skipped
date: 2026-04-29
```

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

- Observable `Set.prototype.add` dispatch remains tracked by issue 279.
