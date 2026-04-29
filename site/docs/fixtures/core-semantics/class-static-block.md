# class-static-block

```typescript
console.log("before");

class Counter {
  static value() {
    return "value";
  }

  static {
    console.log("first:" + Counter.value());
  }

  static {
    let label = "second";
    console.log(label + ":" + Counter.value());
  }
}

console.log("after");

```

**Path:** `fixtures/core-semantics/class-static-block.ts`
