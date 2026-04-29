# function-object-metadata

```typescript
function score(left, right, bonus) {
    return left + right + bonus;
}

function empty() {
    return 1;
}

console.log(score.name);
console.log(score.length);
console.log(empty.name);
console.log(empty.length);
console.log(score(1, 2, 3));

```

**Path:** `fixtures/core-semantics/function-object-metadata.ts`
