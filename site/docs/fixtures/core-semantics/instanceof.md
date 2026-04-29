# instanceof

```typescript
// @ts-nocheck
class Base {
  constructor(v) {
    this.base = v;
  }
}

class Child extends Base {
  constructor(v) {
    super(v);
  }
}

let base = new Base(1);
let child = new Child(2);
let plain = {};

console.log(base instanceof Base);
console.log(base instanceof Child);
console.log(child instanceof Child);
console.log(child instanceof Base);
console.log(plain instanceof Base);
console.log(1 instanceof Base);

Object.setPrototypeOf(plain, Object.getPrototypeOf(child));
console.log(plain instanceof Child);
console.log(plain instanceof Base);

```

**Path:** `fixtures/core-semantics/instanceof.ts`
