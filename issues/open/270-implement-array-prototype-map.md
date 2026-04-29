---
id: 270
title: Implement Array.prototype.map
type: feature
area: runtime/builtins
class: implementation-ready
priority: P2
tracking: feature:array-prototype-methods
---

## Summary

Array.prototype.map method is not implemented. This prevents functional array transformations commonly used in modern JavaScript.

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

Current behavior: UnresolvedName error for Array.prototype.map.

## Acceptance criteria

1. Array.prototype.map is available on Array objects
2. Callback function receives (value, index, array) arguments
3. Returns new array with transformed values
4. Handles sparse arrays correctly
5. Does not modify original array
6. Handles thisArg parameter correctly
7. Test262 Array.prototype.map tests pass

## Validation

```bash
cargo nextest run
cargo fmt --all --check
```

## Notes

- Array.prototype.map is part of ES5.1 specification
- Should handle array-like objects via call/apply
- Consider implementing other Array prototype methods in parallel (filter, reduce, forEach, etc.)
