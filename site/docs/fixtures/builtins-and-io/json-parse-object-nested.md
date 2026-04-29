# json-parse-object-nested

```typescript
let obj = JSON.parse('{"a":[1,2],"b":{"c":3,"d":[4,5]}}');
console.log(obj.a.length);
console.log(obj.a[1]);
console.log(obj.b.c);
console.log(obj.b.d[0]);

```

**Path:** `fixtures/builtins-and-io/json-parse-object-nested.ts`
