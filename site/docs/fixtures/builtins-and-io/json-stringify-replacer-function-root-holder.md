# json-stringify-replacer-function-root-holder

```typescript
function rootHolder(key, value) {
  if (key === "") {
    return this[""];
  }
  return value;
}

console.log(JSON.stringify({ a: 1 }, rootHolder));

```

**Path:** `fixtures/builtins-and-io/json-stringify-replacer-function-root-holder.ts`
