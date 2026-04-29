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
