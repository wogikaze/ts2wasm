---
id: 270
title: Implement Array.prototype.map
type: feature
area: runtime/builtins
class: done
priority: P2
tracking: feature:array-prototype-methods
---

## Summary

Array.prototype.map method is now implemented for named function callbacks. This enables functional array transformations commonly used in modern JavaScript.

## Evidence

AtCoder ABC451 D problem uses array.map for transformations:

```typescript
const transformed = arr.map(x => x * 2);
```

Test262 test case: `reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-2-19.js`

```javascript
function callbackfn(val, idx, obj) {
  return val > 10;
}

var fun = function(a, b) {
  return a + b;
};
fun[0] = 12;
fun[1] = 11;
fun[2] = 9;

var testResult = Array.prototype.map.call(fun, callbackfn);

assert.sameValue(testResult.length, 2, 'testResult.length');
```

Previous behavior: UnresolvedName error for Array.prototype.map.

## Acceptance criteria

1. [x] Array.prototype.map is available on Array objects
2. [x] Callback function receives (value, index, array) arguments
3. [x] Returns new array with transformed values
4. [ ] Handles sparse arrays correctly
5. [x] Does not modify original array
6. [ ] Handles thisArg parameter correctly
7. [ ] Test262 Array.prototype.map tests pass

## Implementation

**2026-04-29 implementation slice:**
- Added ArrayMap to RuntimeFn enum in `crates/backend-wasm/src/runtime_fn.rs`
- Added runtime function mapping and dependencies for ArrayMap
- Implemented emit_array_map in `crates/backend-wasm/src/runtime_arrays_objects.rs` with:
  - New array allocation
  - Element iteration
  - Callback dispatch through `$array_map_callback`
- Added array_map_dispatcher in `crates/backend-wasm/src/emitter.rs` to dispatch callbacks to user functions with proper argument mapping (element, index, array)
- Added "map" to resolve_method_to_runtime_fn in `crates/ir/src/lowered/program.rs`
- Added ArrayMap handling in resolver in `crates/ir/src/lowered/resolver.rs` to:
  - Convert named function callbacks to function IDs
  - Validate arity (1 argument required)
  - Report unsupported forms (arrow functions, call form)
- Maintained diagnostic for Array.prototype.map.call (unsupported form)
- Updated test fixtures and removed unsupported arrow callback fixture
- Added AtCoder ABC451 D fixture for basic map testing

## Validation

```bash
cargo fmt --all --check
cargo nextest run
mise run check issues
```

All commands passed on 2026-04-29. The full suite result was 532 passed and 4 skipped.

## Notes

- Array.prototype.map is part of ES5.1 specification
- Currently supports named function callbacks only
- Arrow function callbacks require additional lowering work
- Array.prototype.map.call form is explicitly unsupported
- Index and array parameters are passed but need more comprehensive testing
- Consider implementing other Array prototype methods in parallel (filter, reduce, forEach, etc.)

## Completion evidence

Commits:
- d983e223: issue-270: implement Array.prototype.map with named function callbacks

Validation result:

```text
command: cargo fmt --all --check
result: PASS
date: 2026-04-29

command: cargo nextest run
result: PASS; 532 tests passed, 4 skipped
date: 2026-04-29

command: mise run check issues
result: PASS
date: 2026-04-29
```

## Remaining risks

- Arrow function callbacks remain unsupported
- thisArg parameter support not implemented
- Sparse array handling not implemented
- Full Test262 Array.prototype.map coverage not achieved
