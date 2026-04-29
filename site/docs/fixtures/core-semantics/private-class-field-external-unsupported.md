# private-class-field-external-unsupported

```typescript
class C {
  #x = 1;
}

let c = new C();
console.log(c.#x);

```

**Path:** `fixtures/core-semantics/private-class-field-external-unsupported.ts`
