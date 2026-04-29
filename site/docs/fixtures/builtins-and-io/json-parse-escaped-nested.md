# json-parse-escaped-nested

```typescript
let obj = JSON.parse('{"a":"x\\\"y","b":["c\\\\d"]}');
console.log(obj.a);
console.log(obj.b[0]);

```

**Path:** `fixtures/builtins-and-io/json-parse-escaped-nested.ts`
