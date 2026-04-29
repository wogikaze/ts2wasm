# json-stringify-replacer-function-keep

```typescript
function keep(key, value) {
  return value;
}

console.log(JSON.stringify({ a: 1 }, keep));

```

**Path:** `fixtures/builtins-and-io/json-stringify-replacer-function-keep.ts`
