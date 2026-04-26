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
