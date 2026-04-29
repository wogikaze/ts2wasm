# function-this-receiver

```typescript
function read(delta) {
    return this.seed + delta;
}

function setSeed(next) {
    this.seed = next;
    return this.seed;
}

let first = { seed: 4, read: read, setSeed: setSeed };
let second = { seed: 9, read: read };

console.log(first.read(3));
console.log(second.read(3));
console.log(first.setSeed(12));
console.log(first.read(1));

```

**Path:** `fixtures/core-semantics/function-this-receiver.ts`
