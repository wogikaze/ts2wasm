# generic-erasure

```typescript
function id<T>(value: T): T {
  return value;
}

function choose<T, U>(left: T, right: U): U {
  return right;
}

let direct: number = id<number>(3);
let selected: number = choose<string, number>("ignored", direct + 4);

console.log(direct);
console.log(selected);

```

**Path:** `fixtures/basics-types/generic-erasure.ts`
