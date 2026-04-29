---
id: 269
title: Implement Math.pow
type: feature
area: runtime/builtins
class: implementation-ready
priority: P2
tracking: feature:math-builtins
---

## Summary

Math.pow builtin function is not implemented. This prevents using exponentiation operations which are commonly used in competitive programming and mathematical computations.

## Evidence

AtCoder ABC451 D problem requires Math.pow for exponentiation:

```typescript
const result = Math.pow(2, n);
```

Test262 test case: `reference/test262/test/built-ins/Math/pow/applying-the-exp-operator_A2.js`

```javascript
var exponent = +0;
var base = new Array();
base[0] = -Infinity;
base[1] = -1.7976931348623157E308;
base[2] = -0.000000000000001;
base[3] = -0;
base[4] = +0
base[5] = 0.000000000000001;
base[6] = 1.7976931348623157E308;
base[7] = +Infinity;
base[8] = NaN;
var basenum = 9;

for (var i = 0; i < basenum; i++) {
  if (Math.pow(base[i], exponent) !== 1) {
    throw new Test262Error("#1: Math.pow(" + base[i] + ", " + exponent + ") !== 1");
  }
}
```

Current behavior: UnresolvedName error for Math.pow.

## Acceptance criteria

1. Math object is available in global scope
2. Math.pow function is implemented with correct semantics
3. Handles edge cases: Infinity, NaN, +0, -0
4. Returns correct results for typical use cases
5. Test262 Math.pow tests pass

## Validation

```bash
cargo nextest run
cargo fmt --all --check
```

## Notes

- Math.pow is part of ES5.1 specification
- Should handle special cases per IEEE 754 semantics
- Consider implementing other Math builtins in parallel (Math.floor, Math.ceil, Math.round, etc.)
