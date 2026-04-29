# type-alias-erasure

```typescript
type Id = number;

export type Point = {
  x: number;
  y: number;
  meta: { created: number };
  translate: (dx: number, dy: number) => number;
};

function sum(point: Point): Id {
  return point.x + point.y;
}

let origin: Point = {
  x: 2,
  y: 3,
  meta: { created: 1 }
};

console.log(sum(origin));

```

**Path:** `fixtures/basics-types/type-alias-erasure.ts`
