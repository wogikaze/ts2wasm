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

Recursive function support is progressing, but the broad issue is not fully closed until each acceptance slice has source-backed coverage. Function declaration self-calls and named function expression self-calls have implementation evidence. Assigned arrow recursion, stack-depth behavior, and broader Test262 coverage remain open.

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

Historical behavior: UnresolvedName or UnsupportedSyntax when function forms call themselves.

## Acceptance criteria

1. [x] Named function expressions can call themselves.
2. [x] Function declarations can call themselves.
3. [ ] Arrow functions can call themselves via assigned variable.
4. [ ] Proper stack handling for recursion depth is covered.
5. [ ] Base-case regression coverage exists for supported recursive forms.
6. [ ] Test262 recursive function coverage is tracked or passes for the supported subset.

## Validation

```bash
cargo fmt --all --check
cargo nextest run -E 'test(recursive) or test(function) or test(node_diff)'
mise run update-issue-index -- --check
mise run check issues
```

## Notes

- Function name binding in function body is an ES5.1 requirement.
- Stack overflow protection may be needed.
- Tail call optimization is optional and not required for this issue.

## Progress evidence

2026-04-29 function declaration slice:

- Nested/function declaration self recursion is covered by `fixtures/core-semantics/recursive-nested-function.ts`.
- Parent validation after merge passed the relevant recursive/function Node differential tests.

2026-04-29 named function expression slice:

- Added frontend/IR support for named function expressions and lowered them through the existing nested-function closure path so the internal function name binds inside the function body.
- Converted the named function expression recursion fixture to `fixtures/core-semantics/named-function-expression-recursive.ts` and added it to the Node/iwasm differential function fixture group.

Validation result:

```text
command: cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/named-function-expression-recursive.ts -o /tmp/issue273.wasm && iwasm /tmp/issue273.wasm && node fixtures/core-semantics/named-function-expression-recursive.ts
result: after implementation both iwasm and Node printed 6
date: 2026-04-29

command: cargo nextest run -E 'test(recursive) or test(function) or test(node_diff)'
result: pass in child branch; 28 tests run, 28 passed, 495 skipped
date: 2026-04-29
```

Remaining work:

- Add explicit Node/iwasm coverage for assigned arrow recursion if supported, or diagnostics/follow-up if not.
- Add stack-depth/base-case regression coverage for supported recursive forms.
- Add or triage broader Test262 recursive function coverage.
