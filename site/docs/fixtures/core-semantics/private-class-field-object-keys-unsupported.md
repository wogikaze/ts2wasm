# private-class-field-object-keys-unsupported

```typescript
class Counter {
  #value = 7;
}

let c = new Counter();
console.log(Object.keys(c));

```

**Path:** `fixtures/core-semantics/private-class-field-object-keys-unsupported.ts`
