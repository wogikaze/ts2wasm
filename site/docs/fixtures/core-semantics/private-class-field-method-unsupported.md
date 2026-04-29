# private-class-field-method-unsupported

```typescript
class C {
  #m() {
    return 1;
  }
}

let c = new C();
console.log(c);

```

**Path:** `fixtures/core-semantics/private-class-field-method-unsupported.ts`
