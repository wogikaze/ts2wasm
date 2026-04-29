# labeled-break-statement

```typescript
let value = 0;

target:
if (true) {
  value = 1;
  break target;
  value = 2;
}

console.log(value);

```

**Path:** `fixtures/control-flow-and-exceptions/labeled-break-statement.ts`
