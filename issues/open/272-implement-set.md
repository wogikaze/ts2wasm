---
id: 272
title: Implement Set
type: feature
area: runtime/builtins
class: implementation-ready
priority: P2
tracking: feature:set-map
---

## Summary

Set builtin object is not implemented. This prevents using Set data structure for unique value collections.

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

Current behavior: UnresolvedName error for Set constructor.

## Acceptance criteria

1. Set constructor is available in global scope
2. Set.prototype.add method works correctly
3. Set.prototype.has method works correctly
4. Set.prototype.delete method works correctly
5. Set.prototype.size property works correctly
6. Set.prototype.clear method works correctly
7. Handles iterable constructor argument
8. Maintains value uniqueness
9. Test262 Set tests pass

## Validation

```bash
cargo nextest run
cargo fmt --all --check
```

## Notes

- Set is part of ES6 specification
- Should handle SameValueZero for equality
- Consider implementing Map in parallel (similar structure)
- Iterator protocol integration required for for...of loops
