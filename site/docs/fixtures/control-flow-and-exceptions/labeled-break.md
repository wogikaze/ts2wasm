# labeled-break

```typescript
let outer = 0;
let inner = 0;

outerLoop:
while (outer < 3) {
  outer = outer + 1;
  while (inner < 5) {
    inner = inner + 1;
    if (inner === 2) {
      break outerLoop;
    }
  }
}

console.log(outer);
console.log(inner);

```

**Path:** `fixtures/control-flow-and-exceptions/labeled-break.ts`
