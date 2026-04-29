---
id: 268
title: Implement for loop increment operator
type: feature
area: frontend/semantics
class: done
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

## Acceptance criteria

1. [x] Parser accepts for loop with increment operator in update expression.
2. [x] Name resolution handles loop variable correctly.
3. [x] Lowering to IR properly represents increment semantics.
4. [x] Runtime execution correctly increments loop variable on each iteration.
5. [x] Basic for-loop increment/decrement fixtures match Node output under iwasm.

## Implementation

The functionality was implemented in previous work:

**First slice (2026-04-29):**
- Postfix identifier updates in for loop update slots (`for (...; ...; i++)`)
- Resolved to existing assignment lowering path
- Regression fixture: `fixtures/core-semantics/for-loop-post-increment.ts`
- Matches Node output under iwasm

**Second slice (2026-04-29):**
- Extended to identifier-only postfix decrement and prefix increment/decrement (`i--`, `++i`, `--i`)
- Update expression value is unused, so these forms lower to same assignment semantics as `i = i +/- 1`
- Node/iwasm fixtures cover postfix decrement and prefix increment/decrement
- Non-identifier update targets remain guarded by issue-268 diagnostic

## Verification

Tested with all increment/decrement forms:

Post-increment (`i++`):

```typescript
for (let i = 0; i < 4; i++) { console.log(i); }
```

Output: 0, 1, 2, 3 (matches Node)

Prefix increment (`++i`):

```typescript
for (let i = 0; i < 4; ++i) { console.log(i); }
```

Output: 0, 1, 2, 3 (matches Node)

Post-decrement (`i--`):

```typescript
for (let i = 4; i > 0; i--) { console.log(i); }
```

Output: 4, 3, 2, 1 (matches Node)

## Validation

```bash
cargo fmt --all --check
cargo nextest run -E 'test(for) or test(loop) or test(node_diff)'
mise run update-issue-index -- --check
mise run check issues
cargo nextest run
```

All listed commands passed on 2026-04-29. The full suite result was 520 passed and 4 skipped.

## Notes

- Basic increment/decrement operators are fully implemented
- Non-identifier update targets remain guarded by diagnostics
- Consider interaction with variable declarations (var/let/const) - all work correctly
