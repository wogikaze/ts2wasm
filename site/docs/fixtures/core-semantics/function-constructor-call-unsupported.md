# function-constructor-call-unsupported

```typescript
// Diagnostic fixture for dynamic Function(...) evaluation tracked by issue 062b.
let f = Function("return 1");
console.log(f());

```

**Path:** `fixtures/core-semantics/function-constructor-call-unsupported.ts`
