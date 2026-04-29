# json-parse-array-object-nested

```typescript
let arr = JSON.parse('[{"a":{"b":1}},{"c":[2,{"d":3}]}]');
console.log(arr.length);
console.log(arr[0].a.b);
console.log(arr[1].c.length);
console.log(arr[1].c[0]);
console.log(arr[1].c[1].d);

```

**Path:** `fixtures/builtins-and-io/json-parse-array-object-nested.ts`
