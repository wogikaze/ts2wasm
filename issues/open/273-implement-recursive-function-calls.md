---
id: 273
title: Implement recursive function calls
type: feature
area: runtime/semantics
class: implementation-ready
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

1. Named function expressions can call themselves
2. Function declarations can call themselves
3. Arrow functions can call themselves via assigned variable
4. Proper stack handling for recursion depth
5. No infinite loop for base case errors
6. Test262 recursive function tests pass

## Validation

```bash
cargo nextest run
cargo fmt --all --check
```

## Notes

- Function name binding in function body is ES5.1 requirement
- Stack overflow protection may be needed
- Consider tail call optimization (optional optimization)
- Ensure proper scoping of function name
