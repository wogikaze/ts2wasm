# private-class-field-read-write

```typescript
class Counter {
  #value = 1;

  read() {
    return this.#value;
  }

  write(next) {
    this.#value = next;
    return this.#value;
  }

  bump() {
    this.#value = this.#value + 1;
    return this.#value;
  }
}

let first = new Counter();
console.log(first.read());
console.log(first.write(4));
console.log(first.bump());

let second = new Counter();
console.log(second.read());

```

**Path:** `fixtures/core-semantics/private-class-field-read-write.ts`
