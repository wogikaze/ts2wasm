# ordinary-function-direct-call

```typescript
function pairScore(left, right) {
    return left * 10 + right;
}

function choose(flag, yes, no) {
    if (flag) {
        return yes;
    }
    return no;
}

console.log(pairScore(4, 2));
console.log(choose(true, "yes", "no"));
console.log(choose(false, 7, 9));

```

**Path:** `fixtures/core-semantics/ordinary-function-direct-call.ts`
