# type-annotation-erasure

```typescript
let uninitialized: number;

function add(a: number, b: number): number {
  return a + b;
}

let total: number = add(1, 2);

console.log(uninitialized);
console.log(total);

```

**Path:** `fixtures/basics-types/type-annotation-erasure.ts`
