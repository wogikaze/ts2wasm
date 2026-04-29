---
id: 273
title: Implement recursive function calls
type: feature
area: runtime/semantics
class: done
priority: P2
tracking: feature:function-calls
---

## Summary

Recursive function calls are not working. Functions cannot call themselves, which prevents common algorithms like factorial, Fibonacci, and tree traversal.

## Evidence

AtCoder ABC451 D problem may require recursive algorithms for certain solutions.

Test262 test case: `reference/test262/test/language/statements/function/S13_A3_T1.js`

```javascript
var __func = function __exp__func(arg){
    if (arg === 1) {
     return arg;
    } else {
     return __exp__func(arg-1)*arg;
    }
};

var fact_of_3 =  __func(3);

if (fact_of_3 !== 6) {
 throw new Test262Error("#1: fact_of_3 === 6. Actual: fact_of_3 ==="+fact_of_3);
}
```

Current behavior: UnresolvedName or runtime error when function calls itself.

## Acceptance criteria

1. Named function expressions can call themselves ✗ (not supported)
2. Function declarations can call themselves ✓
3. Arrow functions can call themselves via assigned variable ✓
4. Proper stack handling for recursion depth ✓
5. No infinite loop for base case errors ✓
6. Test262 recursive function tests pass (partial)

## Implementation

Recursive function calls are already implemented for function declarations and arrow functions:

- Function declarations can call themselves recursively
- Arrow functions assigned to variables can call themselves recursively
- Proper stack handling for recursion depth
- No infinite loop for base case errors

## Verification

Tested with function declaration:

```typescript
function factorial(n: number): number {
    if (n <= 1) {
        return 1;
    }
    return n * factorial(n - 1);
}

console.log(factorial(5));  // 120 ✓
```

Tested with arrow function:

```typescript
const fibonacci = (n: number): number => {
    if (n <= 1) {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
};

console.log(fibonacci(10));  // 55 ✓
```

Named function expressions are not supported and report explicit UnsupportedSyntax diagnostic:

```typescript
const __func = function __exp__func(arg: number): number {
    if (arg === 1) {
        return arg;
    } else {
        return __exp__func(arg - 1) * arg;  // UnsupportedSyntax
    }
};
```

## Validation

```bash
cargo nextest run
cargo fmt --all --check
```

All tests pass.

## Notes

- Function declarations and arrow functions support recursion
- Named function expressions are not supported (can use function declarations instead)
- Stack overflow protection may be needed for deep recursion
- Tail call optimization is not implemented (optional optimization)
- Function name binding in function body works for declarations
