# nullish-coalescing

```typescript
console.log(null ?? 2);
console.log(undefined ?? 2);

let calls = 0;
console.log(null ?? (calls = calls + 1));
console.log(false ?? (calls = calls + 1));
console.log(0 ?? (calls = calls + 1));
console.log("" ?? (calls = calls + 1));
console.log(calls);

```

**Path:** `fixtures/core-semantics/nullish-coalescing.ts`
