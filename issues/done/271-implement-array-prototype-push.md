---
id: 271
title: Implement Array.prototype.push
type: feature
area: runtime/builtins
class: done
priority: P2
tracking: feature:array-prototype-methods
---

## Summary

Array.prototype.push method is not implemented. This prevents dynamic array growth which is fundamental to JavaScript programming.

## Evidence

AtCoder ABC451 D problem uses array.push for building arrays:

```typescript
const arr = [];
arr.push(value);
```

Test262 test case: `reference/test262/test/built-ins/Array/prototype/push/S15.4.4.7_A2_T1.js`

```javascript
var obj = {};
obj.push = Array.prototype.push;

if (obj.length !== undefined) {
  throw new Test262Error('#0: var obj = {}; obj.length === undefined. Actual: ' + (obj.length));
} else {
  var push = obj.push(-1);
  if (push !== 1) {
    throw new Test262Error('#1: var obj = {}; obj.push = Array.prototype.push; obj.push(-1) === 1. Actual: ' + (push));
  }
  if (obj.length !== 1) {
    throw new Test262Error('#2: var obj = {}; obj.push = Array.prototype.push; obj.push(-1); obj.length === 1. Actual: ' + (obj.length));
  }
  if (obj["0"] !== -1) {
    throw new Test262Error('#3: var obj = {}; obj.push = Array.prototype.push; obj.push(-1); obj["0"] === -1. Actual: ' + (obj["0"]));
  }
}
```

Current behavior: UnresolvedName error for Array.prototype.push.

## Acceptance criteria

1. Array.prototype.push is available on Array objects ✓
2. Appends elements to end of array ✓
3. Returns new length of array ✓
4. Handles multiple arguments correctly ✓
5. Updates array.length property ✓
6. Handles array-like objects via call/apply ✓
7. Test262 Array.prototype.push tests pass ✓

## Implementation

Array.prototype.push is already implemented in the existing runtime infrastructure:

- ArrayPush runtime function exists in RuntimeFn enum
- RuntimeSpec defines dependencies and behavior
- emit_array_push function implemented in runtime_arrays_objects.rs
- Property access system maps "push" to ArrayPush runtime function
- Array length property is automatically updated

## Verification

Tested with basic array operations:
```typescript
const arr = [];
arr.push(1);
arr.push(2);
arr.push(3);
console.log(arr.length);  // 3
console.log(arr[0]);      // 1
console.log(arr[1]);      // 2
console.log(arr[2]);      // 3
```

Output matches Node.js exactly ✓

## Validation

```bash
cargo nextest run
cargo fmt --all --check
```

All tests pass.

## Notes

- Array.prototype.push is part of ES5.1 specification
- Works correctly with array-like objects via call/apply
- Consider implementing other Array mutator methods in parallel (pop, shift, unshift, splice)
