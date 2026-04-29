# json-parse-surrogate-pair-object-array

```typescript
let obj = JSON.parse('{"face":"\\ud83d\\ude00","items":["\\u00e9","\\ud83d\\ude00"]}');
console.log(obj.face);
console.log(obj.items[0]);
console.log(obj.items[1]);
console.log(JSON.stringify(obj.items));

```

**Path:** `fixtures/builtins-and-io/json-parse-surrogate-pair-object-array.ts`
