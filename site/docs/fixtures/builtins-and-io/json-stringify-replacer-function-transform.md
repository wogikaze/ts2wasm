# json-stringify-replacer-function-transform

```typescript
function transform(key, value) {
  if (key === "a") {
    return "one";
  }
  return value;
}

console.log(JSON.stringify({ a: 1, b: 2 }, transform));

```

**Path:** `fixtures/builtins-and-io/json-stringify-replacer-function-transform.ts`
