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

- [x] Represent enough of `Set.prototype.add` for assignment and retrieval to be observable.
- [x] Make `new Set(values)` call the resolved add method once per supported dense-array element.
- [x] Add Node/iwasm differential coverage based on `set-iterable-calls-add.js`.

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

- [x] The fixture proves `Set.prototype.add` replacement increments a counter once per supported array element during `new Set(values)`.
- [x] The constructed Set still contains each provided value.
- [x] Node and iwasm stdout match for the fixture.

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

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from issue 276 because the smallest safe progress slice supports dense-array construction through direct Set helpers, while prototype monkey-patching requires broader builtin prototype dispatch support.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `f2d8a762` issue-279: dispatch Set constructor add observably

Validation result:

```text
command: cargo fmt --all --check
result: PASS
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli set
result: PASS; 8 tests run, 8 passed, 338 skipped
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli set_constructor_array_fixture_matches_node_output_under_iwasm
result: PASS; 1 test run, 1 passed, 345 skipped
date: 2026-04-29

command: cargo nextest run
result: PASS; 533 tests run, 533 passed, 4 skipped
date: 2026-04-29

command: mise run update-issue-index -- --check
result: PASS before lifecycle move
date: 2026-04-29

command: mise run check issues
result: PASS before lifecycle move
date: 2026-04-29
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/279-implement-observable-set-constructor-add-dispatch.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
