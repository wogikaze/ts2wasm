# json-parse-unsupported-surrogate-pair

```typescript
let value = JSON.parse('"\\ud83d\\ude00"');
console.log(value);
console.log(JSON.stringify(value));

```

**Path:** `fixtures/builtins-and-io/json-parse-unsupported-surrogate-pair.ts`
