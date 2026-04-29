# ordinary-function-closure-mutation-unsupported

```typescript
function outer() {
  let value = 1;

  function increment() {
    value = value + 1;
    return value;
  }

  return increment();
}

console.log(outer());

```

**Path:** `fixtures/core-semantics/ordinary-function-closure-mutation-unsupported.ts`
