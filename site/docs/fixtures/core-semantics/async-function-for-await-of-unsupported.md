# async-function-for-await-of-unsupported

```typescript
// Diagnostic fixture for the async function wrapper used by for-await-of tests.
async function f() {
  for await (var value of values) {
    console.log(value);
  }
}

```

**Path:** `fixtures/core-semantics/async-function-for-await-of-unsupported.ts`
