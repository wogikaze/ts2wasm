---
id: 268
title: Implement for loop increment operator
type: feature
area: frontend/semantics
class: implementation-ready
priority: P2
tracking: feature:for-loop
---

## Summary

For loops with increment operators (e.g., `i++`) are not currently supported. The parser accepts basic for loop syntax, but the increment operator in the update expression is not properly handled during lowering to IR and runtime execution.

## Evidence

AtCoder ABC451 D problem requires for loops with increment operators:

```typescript
for (let i = 0; i < n; i++) {
    // ...
}
```

Test262 test case: `reference/test262/test/language/statements/for/12.6.3_2-3-a-ii-7.js`

```javascript
var accessed = false;
var strObj = new String("");
for (var i = 0; strObj;) {
    accessed = true;
    break;
}

assert(accessed, 'accessed !== true');
```

Current behavior: UnsupportedSyntax error for increment operator in for loop update expression.

2026-04-29 progress: the first runtime slice supports postfix identifier updates in `for`
loop update slots (`for (...; ...; i++)`) by resolving them to the existing assignment
lowering path. The regression fixture `fixtures/core-semantics/for-loop-post-increment.ts`
matches Node output under iwasm. Prefix increment and decrement forms remain out of scope
for this slice and are covered by source-spanned issue-268 diagnostics.

## Acceptance criteria

1. Parser accepts for loop with increment operator in update expression
2. Name resolution handles loop variable correctly
3. Lowering to IR properly represents increment semantics
4. Runtime execution correctly increments loop variable on each iteration
5. Test262 for loop tests pass (at least basic cases)

## Validation

```bash
cargo nextest run
cargo fmt --all --check
```

## Notes

- Focus on basic `i++` first; `++i` and `i--` can follow
- Ensure increment happens after loop body execution
- Consider interaction with variable declarations (var/let/const)
