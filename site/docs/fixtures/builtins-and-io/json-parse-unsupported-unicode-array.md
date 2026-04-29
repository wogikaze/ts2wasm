# json-parse-unsupported-unicode-array

```typescript
let arr = JSON.parse('["\\u00e9"]');
console.log(arr[0]);
console.log(JSON.stringify(arr));

```

**Path:** `fixtures/builtins-and-io/json-parse-unsupported-unicode-array.ts`
