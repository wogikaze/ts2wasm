# this-extracted-method-unsupported

```typescript
class Box {
  constructor(value) {
    this.value = value;
  }

  read() {
    return this.value;
  }
}

let box = new Box(7);
let read = box.read;

console.log(read());

```

**Path:** `fixtures/core-semantics/this-extracted-method-unsupported.ts`
