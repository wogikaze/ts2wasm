---
id: 274
title: Implement spread operator
type: feature
area: frontend/semantics
class: implementation-ready
priority: P2
tracking: feature:spread-operator
---

## Summary

Spread operator (`...`) is not implemented. This prevents expanding iterables into function arguments, array literals, and object literals.

## Evidence

AtCoder ABC451 D problem uses spread operator for array operations:

```typescript
const arr = [...iterable];
```

Test262 test case: `reference/test262/test/language/expressions/call/spread-sngl-literal.js`

```javascript
var callCount = 0;

(function() {
  assert.sameValue(arguments.length, 3);
  assert.sameValue(arguments[0], 3);
  assert.sameValue(arguments[1], 4);
  assert.sameValue(arguments[2], 5);
  callCount += 1;
}(...[3, 4, 5]));

assert.sameValue(callCount, 1);
```

Current behavior: UnsupportedSyntax error for spread operator.

## Acceptance criteria

1. Parser accepts spread operator syntax
2. Spread in function arguments works
3. Spread in array literals works
4. Spread in object literals works
5. Iterator protocol integration
6. Handles sparse arrays correctly
7. Test262 spread operator tests pass

## Validation

```bash
cargo nextest run
cargo fmt --all --check
```

## Notes

- Spread operator is part of ES6 specification
- Requires iterator protocol implementation
- Should work with Array, Set, Map, and custom iterables
- Consider rest parameter syntax in parallel (related feature)

## Progress

- 2026-04-29: Added the smallest safe slice for issue 274:
  - parser accepts spread syntax in array literals, object literals, and existing call arguments;
  - literal-array call spread such as `sum(...[3, 4, 5])` remains executable and is covered by a Node/iwasm fixture;
  - dynamic call spread, array literal spread, and object literal spread now fail with explicit `issue-274` unsupported diagnostics instead of falling through to generic spread/lowering errors.
- Remaining acceptance criteria are not complete: iterator protocol integration, sparse arrays, object property enumeration, Set/Map/custom iterable spread, and broad Test262 spread coverage still require follow-up implementation.
- 2026-04-29: Added a follow-up executable array-literal slice:
  - array literal spread over dense array literals, such as `[1, ...[2, 3], 4]`, lowers to the existing dense `ArrayNew` representation and matches Node/iwasm output;
  - dynamic array literal spread, object literal spread, and dynamic call spread remain explicitly guarded by `issue-274` unsupported diagnostics.
