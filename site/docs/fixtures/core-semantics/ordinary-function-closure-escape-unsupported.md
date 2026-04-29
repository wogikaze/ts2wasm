# ordinary-function-closure-escape-unsupported

```typescript
function makeReader() {
  let value = "escaped-closure";

  function read() {
    return value;
  }

  return read;
}

let reader = makeReader();
console.log(reader());

```

**Path:** `fixtures/core-semantics/ordinary-function-closure-escape-unsupported.ts`
