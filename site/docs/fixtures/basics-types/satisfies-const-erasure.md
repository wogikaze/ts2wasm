# satisfies-const-erasure

```typescript
let value = ({ x: 3 } satisfies { x: number }) as const;
let angle = <const>{ x: value.x + 4 };

console.log(value.x);
console.log(angle.x);

```

**Path:** `fixtures/basics-types/satisfies-const-erasure.ts`
