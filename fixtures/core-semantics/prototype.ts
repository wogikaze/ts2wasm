// @ts-nocheck
// Test prototype chain lookup
const parent = { a: 1 };
const child = { b: 2 };
// Note: Object.create not yet supported, so we'll test own properties only for now
console.log(child.a); // Should be undefined (no prototype yet)
console.log(child.b); // Should be 2 (own property)

class Base {
  constructor(v) {
    this.base = v;
  }

  value() {
    return this.base + 1;
  }
}

class Child extends Base {
  constructor(v) {
    super(v);
    this.offset = 3;
  }

  sum(delta) {
    return this.value() + this.offset + delta;
  }
}

let c = new Child(10);
console.log(c.value());
console.log(c.sum(4));
console.log(c["base"]);
