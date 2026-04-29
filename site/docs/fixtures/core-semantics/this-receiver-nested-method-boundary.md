# this-receiver-nested-method-boundary

```typescript
class Cell {
  constructor(value) {
    this.value = value;
  }

  read() {
    return this.value;
  }

  pairWithNew(value) {
    let other = new Cell(value);
    return this.read() * 10 + other.read();
  }
}

let left = new Cell(2);
let right = new Cell(5);

console.log(left.pairWithNew(5));
console.log(right.pairWithNew(2));

```

**Path:** `fixtures/core-semantics/this-receiver-nested-method-boundary.ts`
