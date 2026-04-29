# gc-call-frame-root

```typescript
function keepAlive() {
  let holder = { keep: "call-frame-alive" };
  let i = 0;
  let s = "";

  while (i < 2000) {
    s = "gc-call-" + i;
    i = i + 1;
  }

  return holder.keep;
}

console.log(keepAlive());

```

**Path:** `fixtures/core-semantics/gc-call-frame-root.ts`
