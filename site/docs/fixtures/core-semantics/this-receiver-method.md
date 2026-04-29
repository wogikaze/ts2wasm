# this-receiver-method

```typescript
class Counter {
  constructor(seed) {
    this.seed = seed;
  }

  value(delta) {
    return this.seed + delta;
  }

  setSeed(next) {
    this.seed = next;
    return this.seed;
  }
}

let first = new Counter(4);
let second = new Counter(9);

console.log(first.value(3));
console.log(second.value(3));
console.log(first.setSeed(12));
console.log(first.value(1));

```

**Path:** `fixtures/core-semantics/this-receiver-method.ts`
