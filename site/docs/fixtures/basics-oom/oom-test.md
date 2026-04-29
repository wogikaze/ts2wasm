# oom-test

```typescript
// Test fixture for heap OOM check
// This attempts to allocate more memory than available, which should trap gracefully

// Try to allocate a very large string that exceeds the bounded runtime memory limit.
// Initial memory is 2 pages, and the runtime may grow only up to its configured max pages.
// This allocation should trigger an OOM trap

// Create a large string by concatenating many times
let s = "x";
let i = 0;
while (i < 10000) {
    s = s + "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
    i = i + 1;
}
console.log(s);

```

**Path:** `fixtures/basics-oom/oom-test.ts`
