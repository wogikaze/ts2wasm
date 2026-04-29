# ordinary-function-closure-capture

```typescript
function outer() {
  let left = "ordinary-";
  let right = "closure";

  function read(suffix) {
    return left + right + suffix;
  }

  return read("-capture");
}

console.log(outer());

```

**Path:** `fixtures/core-semantics/ordinary-function-closure-capture.ts`
