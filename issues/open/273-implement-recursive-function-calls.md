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

1. [x] Named function expressions can call themselves
2. [ ] Function declarations can call themselves
3. [ ] Arrow functions can call themselves via assigned variable
4. [ ] Proper stack handling for recursion depth
5. [ ] No infinite loop for base case errors
6. [ ] Test262 recursive function tests pass

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

## Progress evidence

- 2026-04-29: Added frontend/IR support for named function expressions and lowered them through the existing nested-function closure path so the internal function name binds only inside the function body.
- 2026-04-29: Converted the named function expression recursion fixture to `fixtures/core-semantics/named-function-expression-recursive.ts` and added it to the Node/iwasm differential function fixture group.

Validation result:

```text
command: cargo run -q -p ts2wasm-cli -- build <pre-rename named function expression fixture> -o /tmp/issue273.wasm
result: before change failed with issue-273 unsupported named function expression diagnostic
date: 2026-04-29

command: cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/named-function-expression-recursive.ts -o /tmp/issue273.wasm && iwasm /tmp/issue273.wasm && node fixtures/core-semantics/named-function-expression-recursive.ts
result: after implementation both iwasm and Node printed 6
date: 2026-04-29

command: cargo nextest run -E 'test(recursive) or test(function) or test(node_diff)'
result: pass; 28 tests run, 28 passed, 495 skipped
date: 2026-04-29
```

Remaining work:

- Function declaration and arrow-assigned recursion need explicit issue evidence before this broad issue can close.
- Recursion depth/stack behavior and broader Test262 recursive function coverage remain open.
