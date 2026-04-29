---
id: 272
title: Implement Set
type: feature
area: runtime/builtins
class: docs-ready
priority: P2
tracking: feature:set-map
status: done
completed: 2026-04-29
---

## Summary

Set builtin object was listed as not implemented, but this issue is stale. Issue 049 already closed the basic Set constructor/add/has/delete subset with Node/iwasm coverage.

## Evidence

AtCoder ABC451 D problem uses Set for deduplication:

```typescript
const set = new Set();
set.add(value);
```

Test262 test case: `reference/test262/test/built-ins/Set/set-iterable-calls-add.js`

```javascript
var setAdd = Set.prototype.add;
var counter = 0;

Set.prototype.add = function(value) {
  counter++;
  setAdd.call(this, value);
};

var s = new Set([1, 2]);

assert.sameValue(counter, 2, "`Set.prototype.add` called twice.");
```

Historical behavior: UnresolvedName error for Set constructor.

Current evidence:

- `issues/done/049-implement-map-set.md` records `new Set()`, `Set.prototype.add`, `Set.prototype.has`, and `Set.prototype.delete` as complete.
- `fixtures/builtins-and-io/map-set.ts` covers `new Set()`, `add`, `has`, and `delete`.
- `crates/cli/tests/m2_node_diff.rs` contains `map_set_collection_fixture_matches_node_output_under_iwasm`, which runs the fixture as a Node/iwasm differential.
- issue 049's completion evidence records `cargo nextest run -p ts2wasm-cli map_set_collection_fixture_matches_node_output_under_iwasm` passing on 2026-04-28.

## Acceptance criteria

- [x] Set constructor is available in global scope: superseded by issue 049 evidence.
- [x] Set.prototype.add method works correctly: superseded by issue 049 evidence.
- [x] Set.prototype.has method works correctly: superseded by issue 049 evidence.
- [x] Set.prototype.delete method works correctly: superseded by issue 049 evidence.
- [x] Set.prototype.size property works correctly: remaining work split to issue 275.
- [x] Set.prototype.clear method works correctly: remaining work split to issue 275.
- [x] Handles iterable constructor argument: remaining work split to issue 276.
- [x] Maintains value uniqueness with SameValueZero identity: remaining work split to issue 277.
- [x] Test262 Set iteration/broader Set behavior passes: remaining work split to issue 278 for iteration, with future reference ramp issues expected after the executable slices land.

## Validation

```bash
mise run update-issue-index
mise run update-issue-index -- --check
mise run check issues
cargo nextest run -p ts2wasm-cli map_set_collection_fixture_matches_node_output_under_iwasm
```

## Notes

- Set is part of ES6 specification
- Should handle SameValueZero for equality
- Consider implementing Map in parallel (similar structure)
- Iterator protocol integration required for for...of loops

## Supersession decision

Issue 272 is closed as a stale broad duplicate of issue 049 for the already implemented basic Set subset. The remaining real work is split into executable follow-up issues:

- issue 275: `Set.prototype.size` and `Set.prototype.clear`
- issue 276: `new Set(values)` for supported iterable inputs
- issue 277: SameValueZero-compatible Set identity
- issue 278: Set iteration

## Completion evidence

Commits:

- `9d72ae8` (`issue-272: split stale set work`)

Validation result:

```text
command: cargo nextest run -p ts2wasm-cli map_set_collection_fixture_matches_node_output_under_iwasm
result: pass; 1 test run, 1 passed, 332 skipped
date: 2026-04-29

command: mise run update-issue-index
result: pass; issues/index.md regenerated
date: 2026-04-29

command: mise run update-issue-index -- --check
result: pass
date: 2026-04-29

command: mise run check issues
result: pass
date: 2026-04-29
```

Remaining risks:

- Broader Set semantics remain open in issues 275-278.
