# new-function-constructor-unsupported

```typescript
// Diagnostic fixture for dynamic new Function(...) evaluation tracked by issue 062b.
let f = new Function("return 1");
console.log(f());

```

**Path:** `fixtures/core-semantics/new-function-constructor-unsupported.ts`
