# json-stringify-space-boxed-symbol

```typescript
console.log(JSON.stringify({ a: 1 }, null, new Number(2)));
console.log(JSON.stringify({ a: 1 }, null, new String(">>")));
console.log(JSON.stringify({ a: 1 }, null, Symbol));
console.log(JSON.stringify({ a: 1 }, null, Symbol("gap")));
console.log(JSON.stringify({ a: 1 }, null, new Boolean(true)));
console.log(JSON.stringify({ a: 1 }, null, new Number(-2)));
console.log(JSON.stringify({ a: 1 }, null, new Number()));
console.log(JSON.stringify({ a: 1 }, null, new String()));
console.log(JSON.stringify({ a: 1 }, null, new Boolean(false)));
console.log(JSON.stringify({ a: 1 }, null, new Object()));

```

**Path:** `fixtures/builtins-and-io/json-stringify-space-boxed-symbol.ts`
