# json-parse-unsupported-unicode-object

```typescript
let obj = JSON.parse('{"s":"\\u00e9"}');
console.log(obj.s);
console.log(JSON.stringify(obj));

```

**Path:** `fixtures/builtins-and-io/json-parse-unsupported-unicode-object.ts`
