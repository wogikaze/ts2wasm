# unsupported-fixture

```typescript
// Test fixture: should fail to compile (uses unsupported async syntax)
async function test() {
    await Promise.resolve();
}
test();

```

**Path:** `fixtures/test-infrastructure/unsupported-fixture.ts`
